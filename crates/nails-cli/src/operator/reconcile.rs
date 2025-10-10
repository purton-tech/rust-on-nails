use super::crd::NailsApp;
use super::finalizer;
use crate::error::Error;
use crate::services::application;
use crate::services::database;
use crate::services::envoy;
use crate::services::ingress;
use crate::services::keycloak;
use crate::services::keycloak_db;
use crate::services::llm;
use crate::services::mailhog;
use crate::services::nginx::deploy_nginx;
use crate::services::oauth2_proxy;
use crate::services::observability;
use crate::services::pgadmin;
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use kube::Api;
use kube::Client;
use kube::Resource;
use kube::ResourceExt;
use kube_runtime::controller::Action;
use std::{sync::Arc, time::Duration};

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

/// Action to be taken upon a NailsApp resource during reconciliation
enum AppAction {
    /// Create the subresources, this includes spawning `n` pods with the core application service
    Create,
    /// Delete all subresources created in the `Create` phase
    Delete,
    /// CRD version has chnaged, upgrade the installation.
    Upgrade,
    /// The resource is in desired state and requires no actions to be taken
    NoOp,
}

pub async fn reconcile(app: Arc<NailsApp>, context: Arc<ContextData>) -> Result<Action, Error> {
    let client: Client = context.client.clone(); // The `Client` is shared -> a clone from the reference is obtained

    let namespace: String = app.namespace().unwrap_or("default".to_string());
    let name = app.name_any();

    let pgadmin = app.spec.pgadmin.unwrap_or_default();

    let disable_ingress = app.spec.disable_ingress.unwrap_or_default();

    let observability = app.spec.observability.unwrap_or_default();

    let development = app.spec.development.unwrap_or_default();

    let current_version = get_current_application_version(&client, &namespace).await?;

    // Performs action as decided by the `determine_action` function.
    match determine_action(&app, current_version) {
        AppAction::Create | AppAction::Upgrade => {
            // Creates a deployment with `n` application service pods, but applies a finalizer first.
            // Finalizer is applied first, as the operator might be shut down and restarted
            // at any time, leaving subresources in intermediate state. This prevents leaks on
            // the resource deletion.

            // Apply the finalizer first. If that fails, the `?` operator invokes automatic conversion
            // of `kube::Error` to the `Error` defined in this crate.
            finalizer::add(client.clone(), &name, &namespace).await?;

            let override_db_password = if development {
                Some("testpassword".to_string())
            } else {
                None
            };

            // The databases
            let primary_db_pass = database::deploy(
                client.clone(),
                &namespace,
                app.spec.primary_db_disk_size,
                &override_db_password,
            )
            .await?;
            let keycloak_db_pass =
                keycloak_db::deploy(client.clone(), &namespace, app.spec.keycloak_db_disk_size)
                    .await?;

            application::deploy(client.clone(), app.spec.clone(), &namespace).await?;
            envoy::deploy(client.clone(), app.spec.clone(), &namespace).await?;
            keycloak::deploy(client.clone(), app.spec.clone(), &namespace).await?;
            oauth2_proxy::deploy(client.clone(), app.spec.clone(), &namespace).await?;
            if !disable_ingress || development {
                ingress::deploy(client.clone(), &namespace, pgadmin, observability).await?;
            }

            if development {
                deploy_nginx(&client, &namespace).await.unwrap();
            }

            mailhog::deploy(client.clone(), &namespace).await?;
            llm::deploy(client.clone(), app.spec.clone(), &namespace).await?;
            if pgadmin {
                pgadmin::deploy(
                    client.clone(),
                    primary_db_pass.clone(),
                    keycloak_db_pass,
                    &namespace,
                )
                .await?;
            }
            if observability {
                observability::deploy(
                    client.clone(),
                    primary_db_pass,
                    app.spec.clone(),
                    &namespace,
                )
                .await?;
            }
            Ok(Action::requeue(Duration::from_secs(10)))
        }
        AppAction::Delete => {
            llm::delete(client.clone(), &namespace).await?;

            envoy::delete(client.clone(), &namespace).await?;
            keycloak::delete(client.clone(), &namespace).await?;
            keycloak_db::delete(client.clone(), &namespace).await?;
            oauth2_proxy::delete(client.clone(), &namespace).await?;

            if !disable_ingress {
                ingress::delete(client.clone(), &namespace).await?;
            }
            mailhog::delete(client.clone(), &namespace).await?;

            if !development {
                application::delete(client.clone(), &namespace).await?;
            }
            database::delete(client.clone(), &namespace).await?;
            if pgadmin {
                pgadmin::delete(client.clone(), &namespace).await?;
            }
            if observability {
                observability::delete(client.clone(), &namespace).await?;
            }

            // Once the deployment is successfully removed, remove the finalizer to make it possible
            // for Kubernetes to delete the resource.
            finalizer::delete(client, &name, &namespace).await?;
            Ok(Action::await_change()) // Makes no sense to delete after a successful delete, as the resource is gone
        }
        // The resource is already in desired state, do nothing and re-check after 10 seconds
        AppAction::NoOp => Ok(Action::requeue(Duration::from_secs(10))),
    }
}

/// If we already have a deployment, get the version we are running.
/// We can do this by getting the application pod by name
async fn get_current_application_version(
    client: &Client,
    namespace: &str,
) -> Result<Option<String>, Error> {
    // Get the API for Pod resources in the specified namespace
    let pods: Api<Pod> = Api::namespaced(client.clone(), namespace);
    let lp = ListParams::default().labels("app=nails-app"); // for this app only

    for p in pods.list(&lp).await? {
        if let Some(spec) = p.spec {
            for container in spec.containers {
                if let Some(env_vars) = container.env {
                    for env_var in env_vars {
                        if env_var.name == "VERSION" {
                            return Ok(env_var.value);
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Resources arrives into reconciliation queue in a certain state. This function looks at
/// the state of given NailsApp resource and decides which actions needs to be performed.
/// The finite set of possible actions is represented by the `AppAction` enum.
///
/// # Arguments
/// - `resource`: A reference to the NailsApp resource being reconciled.
fn determine_action(app: &NailsApp, current_version: Option<String>) -> AppAction {
    let current_version = if let Some(current_version) = current_version {
        current_version
    } else {
        "".to_string()
    };

    if app.meta().deletion_timestamp.is_some() {
        AppAction::Delete
    } else if app
        .meta()
        .finalizers
        .as_ref()
        .map(|v| v.is_empty())
        .unwrap_or(true)
    {
        AppAction::Create
    } else if app.spec.version != current_version {
        tracing::info!(
            "Upgrade detected from {} to {}",
            current_version,
            app.spec.version
        );
        AppAction::Upgrade
    } else {
        AppAction::NoOp
    }
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
