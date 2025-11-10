use super::crd::{NailsApp, NailsAppSpec};
use super::finalizer;
use crate::error::Error;
use crate::services::application::{APPLICATION_NAME, APPLICATION_PORT};
use crate::services::{database, deployment};
use k8s_openapi::api::{apps::v1::Deployment as KubeDeployment, core::v1::Service};
use kube::api::DeleteParams;
use kube::{Api, Client, Resource, ResourceExt};
use kube_runtime::controller::Action;
use serde_json::json;
use std::{sync::Arc, time::Duration};

const DEFAULT_DB_DISK_SIZE_GB: i32 = 20;
const WEB_APP_REPLICAS: i32 = 1;

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
        database::delete(client.clone(), &namespace).await?;
        finalizer::delete(client, &name, &namespace).await?;
        return Ok(Action::await_change());
    }

    finalizer::add(client.clone(), &name, &namespace).await?;

    database::deploy(client.clone(), &namespace, DEFAULT_DB_DISK_SIZE_GB, &None).await?;

    deploy_web_app(&client, &namespace, &app.spec).await?;

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
        json!({"name": "HOSTNAME_URL", "value": spec.hostname_url.clone()}),
    ];

    env.push(json!({"name": "WEB_IMAGE", "value": spec.web.image.clone()}));

    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: APPLICATION_NAME.to_string(),
            image_name: spec.web.image.clone(),
            replicas: WEB_APP_REPLICAS,
            port: APPLICATION_PORT,
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
