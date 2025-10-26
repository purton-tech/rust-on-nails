use crate::error::Error;
use crate::operator::crd::NailsApp;
use crate::services::{keycloak, keycloak_db};
use anyhow::{Context, Result};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::api::core::v1::ServiceAccount;
use k8s_openapi::api::rbac::v1::ClusterRole;
use k8s_openapi::api::rbac::v1::ClusterRoleBinding;
use k8s_openapi::api::rbac::v1::PolicyRule;
use k8s_openapi::api::rbac::v1::RoleRef;
use k8s_openapi::api::rbac::v1::Subject;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::api::ObjectMeta;
use kube::api::Patch;
use kube::api::PatchParams;
use kube::api::PostParams;
use kube::Api;
use kube::Client;
use kube::CustomResourceExt;
use kube_runtime::conditions;
use kube_runtime::wait::await_condition;
use kube_runtime::wait::Condition;
use serde_json::json;

const OPERATOR_IMAGE: &str = "ghcr.io/nails/manager";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const CNPG_YAML: &str = include_str!("../../config/cnpg-1.22.1.yaml");
const NGINX_YAML: &str = include_str!("../../config/nginx-ingress.yaml");
const KEYCLOAK_CRD_KEYCLOAKS: &str = include_str!("../../config/keycloak-crd-keycloaks.yaml");
const KEYCLOAK_CRD_REALM_IMPORTS: &str =
    include_str!("../../config/keycloak-crd-keycloakrealmimports.yaml");
const KEYCLOAK_OPERATOR_YAML: &str = include_str!("../../config/keycloak-operator.yaml");
const KEYCLOAK_DB_SIZE_GIB: i32 = 10;

pub async fn init(initializer: &crate::cli::Initializer) -> Result<()> {
    println!("🔌 Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("✅ Connected");

    install_postgres_operator(&client).await?;
    install_keycloak_operator(&client).await?;
    if !initializer.disable_ingress {
        install_nginx_operator(&client).await?;
    }
    create_namespace(&client, &initializer.operator_namespace).await?;
    create_crd(&client).await?;
    create_roles(&client, &initializer.operator_namespace).await?;
    if !initializer.no_operator {
        create_operator(&client, &initializer.operator_namespace).await?;
    }

    bootstrap_keycloak_namespace(&client).await?;

    Ok(())
}

async fn install_nginx_operator(client: &Client) -> Result<()> {
    println!("🌐 Installing Nginx Ingress Operator");
    super::apply::apply(client, NGINX_YAML, None).await?;

    fn is_deployment_available() -> impl Condition<Deployment> {
        |obj: Option<&Deployment>| {
            if let Some(deployment) = &obj {
                if let Some(status) = &deployment.status {
                    if let Some(phase) = &status.available_replicas {
                        return phase >= &1;
                    }
                }
            }
            false
        }
    }

    println!("⏳ Waiting for Nginx Operator to be Available");
    let deploys: Api<Deployment> = Api::namespaced(client.clone(), "ingress-nginx");
    let establish = await_condition(
        deploys,
        "ingress-nginx-controller",
        is_deployment_available(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(120), establish)
        .await
        .unwrap();

    Ok(())
}

async fn bootstrap_keycloak_namespace(client: &Client) -> Result<()> {
    println!(
        "🗄️ Ensuring Keycloak database in namespace {}",
        keycloak::KEYCLOAK_NAMESPACE
    );
    match keycloak_db::deploy(
        client.clone(),
        keycloak::KEYCLOAK_NAMESPACE,
        KEYCLOAK_DB_SIZE_GIB,
    )
    .await?
    {
        Some(_) => println!("✅ Keycloak database created."),
        None => println!("ℹ️ Keycloak database already present."),
    }

    println!(
        "🛡️ Ensuring Keycloak instance in namespace {}",
        keycloak::KEYCLOAK_NAMESPACE
    );
    keycloak::bootstrap(client.clone()).await?;

    Ok(())
}

async fn install_postgres_operator(client: &Client) -> Result<()> {
    println!("🐘 Installing Cloud Native Postgres Operator (CNPG)");
    super::apply::apply(client, CNPG_YAML, None).await?;

    fn is_deployment_available() -> impl Condition<Deployment> {
        |obj: Option<&Deployment>| {
            if let Some(deployment) = &obj {
                if let Some(status) = &deployment.status {
                    if let Some(phase) = &status.available_replicas {
                        return phase >= &1;
                    }
                }
            }
            false
        }
    }

    println!("⏳ Waiting for Cloud Native Postgres Controller Manager");
    let deploys: Api<Deployment> = Api::namespaced(client.clone(), "cnpg-system");
    let establish = await_condition(
        deploys,
        "cnpg-controller-manager",
        is_deployment_available(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(120), establish)
        .await
        .unwrap();

    Ok(())
}

async fn install_keycloak_operator(client: &Client) -> Result<()> {
    println!("🛡️ Installing Keycloak Operator");
    create_namespace(client, keycloak::KEYCLOAK_NAMESPACE).await?;

    super::apply::apply(client, KEYCLOAK_CRD_KEYCLOAKS, None).await?;
    super::apply::apply(client, KEYCLOAK_CRD_REALM_IMPORTS, None).await?;

    wait_for_crd(client, "keycloaks.k8s.keycloak.org").await?;
    wait_for_crd(client, "keycloakrealmimports.k8s.keycloak.org").await?;
    super::apply::apply(
        client,
        KEYCLOAK_OPERATOR_YAML,
        Some(keycloak::KEYCLOAK_NAMESPACE),
    )
    .await?;

    fn is_deployment_available() -> impl Condition<Deployment> {
        |obj: Option<&Deployment>| {
            if let Some(deployment) = &obj {
                if let Some(status) = &deployment.status {
                    if let Some(phase) = &status.available_replicas {
                        return phase >= &1;
                    }
                }
            }
            false
        }
    }

    println!("⏳ Waiting for Keycloak Operator to be Available");
    let deploys: Api<Deployment> = Api::namespaced(client.clone(), keycloak::KEYCLOAK_NAMESPACE);
    let establish = await_condition(deploys, "keycloak-operator", is_deployment_available());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(120), establish)
        .await
        .unwrap();

    Ok(())
}

async fn wait_for_crd(client: &Client, name: &str) -> Result<()> {
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    let establish = await_condition(crds.clone(), name, conditions::is_crd_established());
    tokio::time::timeout(std::time::Duration::from_secs(60), establish)
        .await
        .context(format!("Timed out waiting for CRD {}", name))??;
    Ok(())
}

async fn create_operator(client: &Client, namespace: &str) -> Result<()> {
    println!("🤖 Installing the operator into {}", namespace);
    let app_labels = serde_json::json!({
        "app": "nails-operator",
    });

    let deployment = serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": "nails-operator-deployment",
            "namespace": namespace
        },
        "spec": {
            "replicas": 1,
            "selector": {
                "matchLabels": app_labels
            },
            "template": {
                "metadata": {
                    "labels": app_labels
                },
                "spec": {
                    "serviceAccountName": "nails-operator-service-account",
                    "containers": json!([{
                        "name": "nails-operator",
                        "image": format!("{}:{}", OPERATOR_IMAGE, VERSION)
                    }]),
                }
            }
        }
    });

    // Create the deployment defined above
    let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    deployment_api
        .patch(
            "nails-operator-deployment",
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(deployment),
        )
        .await?;

    Ok(())
}

async fn create_roles(client: &Client, operator_namespace: &str) -> Result<()> {
    println!("🔐 Setting up roles");
    let sa_api: Api<ServiceAccount> = Api::namespaced(client.clone(), operator_namespace);
    let service_account = ServiceAccount {
        metadata: ObjectMeta {
            name: Some("nails-operator-service-account".to_string()),
            namespace: Some(operator_namespace.to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    sa_api
        .patch(
            "nails-operator-service-account",
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(service_account),
        )
        .await?;
    let role_api: Api<ClusterRole> = Api::all(client.clone());
    let role = ClusterRole {
        metadata: ObjectMeta {
            name: Some("nails-operator-cluster-role".to_string()),
            ..Default::default()
        },
        rules: Some(vec![PolicyRule {
            api_groups: Some(vec!["*".to_string()]),
            resources: Some(vec!["*".to_string()]),
            verbs: vec!["*".to_string()],
            ..Default::default()
        }]),
        ..Default::default()
    };
    role_api
        .patch(
            "nails-operator-cluster-role",
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(role),
        )
        .await?;

    // Now the cluster role
    let role_binding_api: Api<ClusterRoleBinding> = Api::all(client.clone());
    let role_binding = ClusterRoleBinding {
        metadata: ObjectMeta {
            name: Some("nails-operator-cluster-role-binding".to_string()),
            ..Default::default()
        },
        role_ref: RoleRef {
            api_group: "rbac.authorization.k8s.io".to_string(),
            kind: "ClusterRole".to_string(),
            name: "nails-operator-cluster-role".to_string(),
        },
        subjects: Some(vec![Subject {
            kind: "ServiceAccount".to_string(),
            name: "nails-operator-service-account".to_string(),
            namespace: Some(operator_namespace.to_string()),
            ..Default::default()
        }]),
    };
    role_binding_api
        .patch(
            "nails-operator-cluster-role-binding",
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(role_binding),
        )
        .await?;
    Ok(())
}

async fn create_crd(client: &Client) -> Result<(), Error> {
    println!("📜 Installing NailsApp CRD");
    let crd = NailsApp::crd();
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    crds.patch(
        "nailsapps.nails-cli.dev",
        &PatchParams::apply(crate::MANAGER).force(),
        &Patch::Apply(crd),
    )
    .await?;

    println!("⏳ Waiting for NailsApp CRD");
    let establish = await_condition(
        crds,
        "nailsapps.nails-cli.dev",
        conditions::is_crd_established(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish)
        .await
        .unwrap();
    Ok(())
}

async fn create_namespace(client: &Client, namespace: &str) -> Result<Namespace> {
    println!("📦 Creating namespace {}", namespace);
    // Define the API object for Namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());
    match namespaces.get(namespace).await {
        Ok(ns) => Ok(ns),
        Err(kube::Error::Api(err)) if err.code == 404 => {
            let new_namespace = Namespace {
                metadata: ObjectMeta {
                    name: Some(namespace.to_string()),
                    ..Default::default()
                },
                ..Default::default()
            };

            match namespaces
                .create(&PostParams::default(), &new_namespace)
                .await
            {
                Ok(ns) => Ok(ns),
                Err(kube::Error::Api(err)) if err.code == 409 => {
                    Ok(namespaces.get(namespace).await?)
                }
                Err(err) => Err(err.into()),
            }
        }
        Err(err) => Err(err.into()),
    }
}
