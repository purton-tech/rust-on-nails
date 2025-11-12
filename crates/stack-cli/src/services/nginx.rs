use crate::cli::apply;
use crate::error::Error;
use crate::services::application::APPLICATION_NAME;
use k8s_openapi::api::{
    apps::v1::Deployment as KubeDeployment,
    core::v1::{ConfigMap, Service},
};
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

use super::deployment;

pub const NGINX_NAME: &str = "nginx";
pub const NGINX_PORT: u16 = 80;

pub enum NginxMode {
    Oidc,
    StaticJwt { token: String },
}

// The web user interface
pub async fn deploy_nginx(
    client: &Client,
    namespace: &str,
    mode: NginxMode,
    upstream_port: u16,
) -> Result<(), Error> {
    let env = vec![];

    let image_name = "nginx:1.27.2".to_string();

    let config_body = match mode {
        NginxMode::Oidc => r#"
server {
    listen 80;

    # Increase buffer sizes to handle large headers
    proxy_buffer_size   128k;
    proxy_buffers       4 256k;
    proxy_busy_buffers_size 256k;

    location /oidc {
        proxy_pass http://keycloak-service:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_redirect ~^http://keycloak-service\.keycloak\.svc\.cluster\.local:8080/(.*)$ $scheme://$host/$1;
    }

    location /realms {
        proxy_pass http://keycloak-service:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_redirect ~^http://keycloak-service\.keycloak\.svc\.cluster\.local:8080/(.*)$ $scheme://$host/$1;
    }

    location / {
        proxy_pass http://oauth2-proxy:7900;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_redirect ~^http://keycloak-service\.keycloak\.svc\.cluster\.local:8080/(.*)$ $scheme://$host/$1;
    }
}
"#
        .to_string(),
        NginxMode::StaticJwt { token } => {
            let escaped_token = token.replace('"', "\\\"");
            format!(
                r#"
server {{
    listen 80;

    proxy_buffer_size   128k;
    proxy_buffers       4 256k;
    proxy_busy_buffers_size 256k;

    location / {{
        proxy_pass http://{app}:{port};
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header Authorization "Bearer {token}";
        proxy_set_header X-Auth-JWT "{token}";
    }}
}}
"#,
                app = APPLICATION_NAME,
                port = upstream_port,
                token = escaped_token
            )
        }
    };

    // Put the nginx config into a ConfigMap
    let config_map = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": NGINX_NAME,
            "namespace": namespace
        },
        "data": {
            "default.conf": config_body,
        }
    });

    apply::apply(client, &config_map.to_string(), Some(namespace))
        .await
        .map_err(Error::from)?;

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
