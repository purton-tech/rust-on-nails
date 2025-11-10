use crate::cli::apply;
use crate::error::Error;
use anyhow::Result;
use k8s_openapi::api::{
    apps::v1::Deployment as KubeDeployment,
    core::v1::{ConfigMap, Service},
};
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

use super::deployment;

pub const NGINX_NAME: &str = "nginx";
pub const NGINX_PORT: u16 = 7903;
const NGINX_CONF: &str = include_str!("../../config/nginx-proxy.conf");

// The web user interface
pub async fn deploy_nginx(client: &Client, namespace: &str) -> Result<()> {
    let env = vec![];

    let image_name = "nginx:1.27.2".to_string();

    // Put the nginx config into a ConfigMap
    let config_map = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": NGINX_NAME,
            "namespace": namespace
        },
        "data": {
            "default.conf": NGINX_CONF,
        }
    });

    apply::apply(client, &config_map.to_string(), Some(namespace)).await?;

    // Application with the migrations as a sidecar
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: NGINX_NAME.to_string(),
            image_name,
            replicas: 1,
            port: NGINX_PORT,
            env,
            command: None,
            init_container: None,
            volume_mounts: vec![json!({"name": NGINX_NAME, "mountPath": "/etc/nginx/conf.d"})],
            volumes: vec![json!({"name": NGINX_NAME,
                "configMap": {
                    "name": NGINX_NAME
                }
            })],
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn delete_nginx(client: Client, namespace: &str) -> Result<(), Error> {
    let deployments: Api<KubeDeployment> = Api::namespaced(client.clone(), namespace);
    if deployments.get(NGINX_NAME).await.is_ok() {
        deployments
            .delete(NGINX_NAME, &DeleteParams::default())
            .await?;
    }

    let services: Api<Service> = Api::namespaced(client.clone(), namespace);
    if services.get(NGINX_NAME).await.is_ok() {
        services
            .delete(NGINX_NAME, &DeleteParams::default())
            .await?;
    }

    let configs: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    if configs.get(NGINX_NAME).await.is_ok() {
        configs.delete(NGINX_NAME, &DeleteParams::default()).await?;
    }

    Ok(())
}
