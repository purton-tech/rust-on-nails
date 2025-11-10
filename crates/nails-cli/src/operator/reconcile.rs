use super::crd::{NailsApp, NailsAppSpec};
use super::finalizer;
use crate::error::Error;
use crate::services::application::APPLICATION_NAME;
use crate::services::{cloudflare, database, deployment, keycloak, nginx, oauth2_proxy};
use k8s_openapi::api::{
    apps::v1::Deployment as KubeDeployment,
    core::v1::{ConfigMap, Secret, Service},
};
use kube::api::DeleteParams;
use kube::{Api, Client, Resource, ResourceExt};
use kube_runtime::controller::Action;
use serde_json::json;
use std::{sync::Arc, time::Duration};

const DEFAULT_DB_DISK_SIZE_GB: i32 = 20;
const WEB_APP_REPLICAS: i32 = 1;
const CLOUDFLARE_DEPLOYMENT_NAME: &str = "cloudflared";
const CLOUDFLARE_SECRET_NAME: &str = "cloudflare-credentials";
const CLOUDFLARE_CONFIG_NAME: &str = "cloudflared";

/// Context injected with each `reconcile` and `on_error` method invocation.
pub struct ContextData {
    /// Kubernetes client to make Kubernetes API requests with. Required for K8S resource management.
    client: Client,
}

impl ContextData {
    // Constructs a new instance of ContextData.
    //
    // # Arguments:
    // - `client`: A Kubernetes client to make Kubernetes REST API requests with.
    // Resources will be created and deleted with this client.
    pub fn new(client: Client) -> Self {
        ContextData { client }
    }
}

pub async fn reconcile(app: Arc<NailsApp>, context: Arc<ContextData>) -> Result<Action, Error> {
    let client: Client = context.client.clone(); // The `Client` is shared -> a clone from the reference is obtained

    let namespace: String = app.namespace().unwrap_or("default".to_string());
    let name = app.name_any();

    if app.meta().deletion_timestamp.is_some() {
        delete_application_resources(&client, &namespace).await?;
        oauth2_proxy::delete(client.clone(), &namespace).await?;
        nginx::delete_nginx(client.clone(), &namespace).await?;
        keycloak::delete(client.clone(), &namespace).await?;
        delete_cloudflare_resources(&client, &namespace).await?;
        database::delete(client.clone(), &namespace).await?;
        finalizer::delete(client, &name, &namespace).await?;
        return Ok(Action::await_change());
    }

    finalizer::add(client.clone(), &name, &namespace).await?;

    database::deploy(client.clone(), &namespace, DEFAULT_DB_DISK_SIZE_GB, &None).await?;

    let auth_hostname = app
        .spec
        .auth
        .as_ref()
        .and_then(|auth| auth.hostname_url.clone());
    let jwt_value = app
        .spec
        .auth
        .as_ref()
        .and_then(|auth| auth.jwt.clone())
        .unwrap_or_else(|| "1".to_string());

    if let Some(hostname_url) = auth_hostname {
        let realm_config =
            oauth2_proxy::ensure_secret(client.clone(), &namespace, &hostname_url).await?;
        keycloak::ensure_realm(client.clone(), &realm_config).await?;
        oauth2_proxy::deploy(client.clone(), &namespace, &hostname_url, app.spec.web.port).await?;
        nginx::deploy_nginx(
            &client,
            &namespace,
            nginx::NginxMode::Oidc,
            app.spec.web.port,
        )
        .await?;
    } else {
        cleanup_auth_resources(client.clone(), &namespace).await?;
        nginx::deploy_nginx(
            &client,
            &namespace,
            nginx::NginxMode::StaticJwt {
                token: jwt_value.clone(),
            },
            app.spec.web.port,
        )
        .await?;
    }

    deploy_web_app(&client, &namespace, &app.spec).await?;

    cloudflare::deploy(&client, &namespace, &namespace, None)
        .await
        .map_err(Error::from)?;

    Ok(Action::requeue(Duration::from_secs(10)))
}

/// Actions to be taken when a reconciliation fails - for whatever reason.
/// Prints out the error to `stderr` and requeues the resource for another reconciliation after
/// five seconds.
///
/// # Arguments
/// - `resource`: The erroneous resource.
/// - `error`: A reference to the `kube::Error` that occurred during reconciliation.
/// - `_context`: Unused argument. Context Data "injected" automatically by kube-rs.
pub fn on_error(resource: Arc<NailsApp>, error: &Error, _context: Arc<ContextData>) -> Action {
    eprintln!("Reconciliation error:\n{:?}.\n{:?}", error, resource);
    Action::requeue(Duration::from_secs(5))
}

async fn deploy_web_app(
    client: &Client,
    namespace: &str,
    spec: &NailsAppSpec,
) -> Result<(), Error> {
    let hostname_env = spec
        .auth
        .as_ref()
        .and_then(|a| a.hostname_url.clone())
        .unwrap_or_default();

    let mut env = vec![
        json!({
            "name": "DATABASE_URL",
            "valueFrom": {
                "secretKeyRef": {
                    "name": "database-urls",
                    "key": "application-url"
                }
            }
        }),
        json!({
            "name": "DATABASE_READONLY_URL",
            "valueFrom": {
                "secretKeyRef": {
                    "name": "database-urls",
                    "key": "readonly-url"
                }
            }
        }),
        json!({
            "name": "DATABASE_MIGRATIONS_URL",
            "valueFrom": {
                "secretKeyRef": {
                    "name": "database-urls",
                    "key": "migrations-url"
                }
            }
        }),
        json!({
            "name": "DB_OWNER_USERNAME",
            "valueFrom": {
                "secretKeyRef": {
                    "name": "db-owner",
                    "key": "username"
                }
            }
        }),
        json!({
            "name": "DB_OWNER_PASSWORD",
            "valueFrom": {
                "secretKeyRef": {
                    "name": "db-owner",
                    "key": "password"
                }
            }
        }),
        json!({"name": "HOSTNAME_URL", "value": hostname_env}),
    ];

    env.push(json!({"name": "WEB_IMAGE", "value": spec.web.image.clone()}));

    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: APPLICATION_NAME.to_string(),
            image_name: spec.web.image.clone(),
            replicas: WEB_APP_REPLICAS,
            port: spec.web.port,
            env,
            init_container: None,
            command: None,
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await
}

async fn delete_application_resources(client: &Client, namespace: &str) -> Result<(), Error> {
    let deployments: Api<KubeDeployment> = Api::namespaced(client.clone(), namespace);
    if deployments.get(APPLICATION_NAME).await.is_ok() {
        deployments
            .delete(APPLICATION_NAME, &DeleteParams::default())
            .await?;
    }

    let services: Api<Service> = Api::namespaced(client.clone(), namespace);
    if services.get(APPLICATION_NAME).await.is_ok() {
        services
            .delete(APPLICATION_NAME, &DeleteParams::default())
            .await?;
    }

    Ok(())
}

async fn delete_cloudflare_resources(client: &Client, namespace: &str) -> Result<(), Error> {
    let deployments: Api<KubeDeployment> = Api::namespaced(client.clone(), namespace);
    if deployments.get(CLOUDFLARE_DEPLOYMENT_NAME).await.is_ok() {
        deployments
            .delete(CLOUDFLARE_DEPLOYMENT_NAME, &DeleteParams::default())
            .await?;
    }

    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);
    if secrets.get(CLOUDFLARE_SECRET_NAME).await.is_ok() {
        secrets
            .delete(CLOUDFLARE_SECRET_NAME, &DeleteParams::default())
            .await?;
    }

    let configs: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    if configs.get(CLOUDFLARE_CONFIG_NAME).await.is_ok() {
        configs
            .delete(CLOUDFLARE_CONFIG_NAME, &DeleteParams::default())
            .await?;
    }

    Ok(())
}

async fn cleanup_auth_resources(client: Client, namespace: &str) -> Result<(), Error> {
    oauth2_proxy::delete(client.clone(), namespace).await?;
    keycloak::delete(client, namespace).await?;
    Ok(())
}
