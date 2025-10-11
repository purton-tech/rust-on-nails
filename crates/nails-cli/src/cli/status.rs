use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::{Pod, Secret};
use kube::api::{ListParams, LogParams};
use kube::{Api, Client, ResourceExt};

fn decode_secret_field(secret: &Secret, key: &str) -> Option<String> {
    if let Some(data) = &secret.data {
        if let Some(value) = data.get(key) {
            if let Ok(decoded) = String::from_utf8(value.0.clone()) {
                if !decoded.is_empty() {
                    return Some(decoded);
                }
            }
        }
    }
    if let Some(string_data) = &secret.string_data {
        if let Some(value) = string_data.get(key) {
            if !value.is_empty() {
                return Some(value.clone());
            }
        }
    }
    None
}

fn extract_cloudflare_url(logs: &str) -> Option<String> {
    for line in logs.lines().rev() {
        if let Some(start) = line.find("https://") {
            let slice = &line[start..];
            let end = slice
                .find(|c: char| c.is_whitespace())
                .unwrap_or(slice.len());
            let mut url = slice[..end].trim_end_matches('.').to_string();
            if url.ends_with('"') {
                url.pop();
            }
            if url.ends_with('"') {
                url.pop();
            }
            if !url.is_empty() {
                return Some(url);
            }
        }
    }
    None
}

pub async fn status(args: &crate::cli::StatusArgs) -> Result<()> {
    println!("🔌 Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("✅ Connected");

    let keycloak_secret_api: Api<Secret> =
        Api::namespaced(client.clone(), args.keycloak_namespace.as_str());
    let admin_secret = keycloak_secret_api
        .get("keycloak-initial-admin")
        .await
        .context("Missing keycloak initial admin secret")?;
    let username = decode_secret_field(&admin_secret, "username").unwrap_or_else(|| "admin".into());
    let password =
        decode_secret_field(&admin_secret, "password").unwrap_or_else(|| "<unknown>".into());

    println!("🛡️ Keycloak Admin");
    println!("   Username: {}", username);
    println!("   Password: {}", password);

    let pods: Api<Pod> = Api::namespaced(client.clone(), args.namespace.as_str());
    let pod_list = pods
        .list(&ListParams::default().labels("app=cloudflared"))
        .await
        .context("Unable to list cloudflared pods")?;

    if let Some(pod) = pod_list.items.first() {
        let pod_name = pod.name_any();
        let logs = pods
            .logs(
                &pod_name,
                &LogParams {
                    tail_lines: Some(200),
                    ..Default::default()
                },
            )
            .await
            .unwrap_or_default();
        if let Some(url) = extract_cloudflare_url(&logs) {
            let base = url.trim_end_matches('/');
            println!("☁️ Cloudflare URL: {}", base);
            println!("   Keycloak login: {}/oidc", base);
        } else {
            println!("☁️ Cloudflare URL: (not found in recent logs – is the tunnel running?)");
        }
    } else {
        println!(
            "☁️ Cloudflare deployment not found in namespace '{}'",
            args.namespace
        );
    }

    Ok(())
}
