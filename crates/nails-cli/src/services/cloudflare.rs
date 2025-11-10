use crate::cli::apply;
use crate::services::nginx::{NGINX_NAME, NGINX_PORT};
use anyhow::Result;
use kube::Client;

const CLOUDFLARE_YAML: &str = include_str!("../../config/cloudflare.yaml");
const CLOUDFLARE_QUICK_YAML: &str = r#"---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cloudflared
spec:
  selector:
    matchLabels:
      app: cloudflared
  replicas: 1
  template:
    metadata:
      labels:
        app: cloudflared
    spec:
      containers:
      - name: cloudflared
        image: cloudflare/cloudflared:latest
        args:
        - tunnel
        - --no-autoupdate
        - --protocol
        - http2
        - --url
        - $TARGET_URL
"#;

pub async fn deploy(
    client: &Client,
    namespace: &str,
    tunnel_name: &str,
    token: Option<&str>,
) -> Result<()> {
    let nginx_target = format!(
        "http://{nginx}.{namespace}.svc.cluster.local:{port}",
        nginx = NGINX_NAME,
        namespace = namespace,
        port = NGINX_PORT
    );

    if let Some(token) = token {
        let yaml = CLOUDFLARE_YAML
            .replace("$TUNNEL_TOKEN", token)
            .replace("$TUNNEL_NAME", tunnel_name)
            .replace("$INGRESS_TARGET", &nginx_target);
        apply::apply(client, &yaml, Some(namespace)).await
    } else {
        let yaml = CLOUDFLARE_QUICK_YAML.replace("$TARGET_URL", &nginx_target);
        apply::apply(client, &yaml, Some(namespace)).await
    }
}

pub async fn install(installer: &crate::cli::CloudflareInstaller) -> Result<()> {
    println!("Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("Connected");
    deploy(
        &client,
        &installer.namespace,
        &installer.name,
        Some(installer.token.as_str()),
    )
    .await?;
    println!("Cloudflare tunnel installed");
    Ok(())
}
