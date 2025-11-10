use super::deployment;
use crate::error::Error;
use crate::operator::crd::NailsAppSpec;
use crate::services::application::{APPLICATION_NAME, APPLICATION_PORT};
use crate::services::keycloak::{RealmConfig, KEYCLOAK_INTERNAL_URL, KEYCLOAK_REALM_BASE_PATH};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Secret, Service};
use kube::api::{DeleteParams, Patch, PatchParams};
use kube::{Api, Client};
use rand::rngs::OsRng;
use rand::RngCore;
use serde_json::json;
use url::Url;

const OAUTH2_PROXY_IMAGE: &str = "quay.io/oauth2-proxy/oauth2-proxy:v7.5.1";
const OAUTH2_PROXY_PORT: u16 = 7900;

// Oauth2 Proxy handles authentication as our Open ID Connect provider
pub async fn deploy(client: Client, spec: &NailsAppSpec, namespace: &str) -> Result<(), Error> {
    let whitelist_domain = Url::parse(&spec.hostname_url);
    let whitelist_domain = if let Ok(host) = &whitelist_domain {
        host.host_str().unwrap_or_default()
    } else {
        ""
    };

    // Oauth2 Proxy
    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: "oauth2-proxy".to_string(),
            image_name: OAUTH2_PROXY_IMAGE.to_string(),
            replicas: 1,
            port: OAUTH2_PROXY_PORT,
            env: vec![
                json!({"name": "OAUTH2_PROXY_HTTP_ADDRESS", "value": format!("0.0.0.0:{}", OAUTH2_PROXY_PORT)}),
                json!({
                    "name":
                    "OAUTH2_PROXY_COOKIE_SECRET",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "cookie-secret"
                        }
                    }
                }),
                json!({"name": "OAUTH2_PROXY_EMAIL_DOMAINS", "value": "*"}),
                json!({"name": "OAUTH2_PROXY_COOKIE_SECURE", "value": "false"}),
                json!({"name": "OAUTH2_PROXY_UPSTREAMS", "value": format!("http://{}:{}", APPLICATION_NAME, APPLICATION_PORT)}),
                json!({"name": "OAUTH2_PROXY_UPSTREAM_TIMEOUT", "value": "600s"}),
                json!({
                    "name":
                    "OAUTH2_PROXY_CLIENT_SECRET",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "client-secret"
                        }
                    }
                }),
                json!({
                    "name":
                    "OAUTH2_PROXY_CLIENT_ID",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "client-id"
                        }
                    }
                }),
                json!({
                    "name":
                    "OAUTH2_PROXY_REDIRECT_URL",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "redirect-uri"
                        }
                    }
                }),
                json!({
                    "name":
                    "OAUTH2_PROXY_OIDC_ISSUER_URL",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "oidc-secret",
                            "key": "issuer-url"
                        }
                    }
                }),

                // This line sends us the user info in a JWT (which is base64 encoded)
                json!({"name": "OAUTH2_PROXY_PASS_ACCESS_TOKEN", "value": "true"}),

                json!({"name": "OAUTH2_PROXY_INSECURE_OIDC_SKIP_ISSUER_VERIFICATION", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_INSECURE_OIDC_ALLOW_UNVERIFIED_EMAIL", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_PROVIDER", "value": "oidc"}),
                json!({"name": "OAUTH2_PROXY_PROVIDER_DISPLAY_NAME", "value": "Keycloak"}),
                json!({"name": "OAUTH2_PROXY_AUTH_LOGGING", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_SKIP_PROVIDER_BUTTON", "value": "true"}),
                json!({"name": "OAUTH2_PROXY_WHITELIST_DOMAINS", "value": whitelist_domain}),
                json!({"name": "OAUTH2_PROXY_SKIP_AUTH_ROUTES", "value": "^/v1*"}),
                json!({"name": "OAUTH2_PROXY_SCOPE", "value": "openid email profile"})
            ],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![],
            }),
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn ensure_secret(
    client: Client,
    namespace: &str,
    spec: &NailsAppSpec,
) -> Result<RealmConfig, Error> {
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    let existing_secret = secret_api.get("oidc-secret").await.ok();
    let allow_registration = true;

    let realm = existing_secret
        .as_ref()
        .and_then(|secret| read_secret_field(secret, "realm"))
        .unwrap_or_else(|| namespace.to_string());
    let default_client_id = format!("{}-client", namespace);

    let client_id = existing_secret
        .as_ref()
        .and_then(|secret| read_secret_field(secret, "client-id"))
        .unwrap_or(default_client_id);
    let client_secret = existing_secret
        .as_ref()
        .and_then(|secret| read_secret_field(secret, "client-secret"))
        .unwrap_or_else(rand_base64);
    let cookie_secret = existing_secret
        .as_ref()
        .and_then(|secret| read_secret_field(secret, "cookie-secret"))
        .unwrap_or_else(rand_base64);

    let redirect_uri = redirect_uri_value(&spec.hostname_url);
    let issuer_url = format!(
        "{base}{path}/{realm}",
        base = KEYCLOAK_INTERNAL_URL,
        path = KEYCLOAK_REALM_BASE_PATH,
        realm = realm
    );

    let secret_manifest = json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": "oidc-secret",
            "namespace": namespace
        },
        "stringData": {
            "client-id": client_id,
            "client-secret": client_secret,
            "redirect-uri": redirect_uri.clone(),
            "issuer-url": issuer_url,
            "cookie-secret": cookie_secret,
            "realm": realm
        }
    });

    secret_api
        .patch(
            "oidc-secret",
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(secret_manifest),
        )
        .await?;

    Ok(RealmConfig {
        namespace: namespace.to_string(),
        realm,
        client_id,
        client_secret,
        redirect_uris: vec![redirect_uri],
        allow_registration,
    })
}

pub fn rand_base64() -> String {
    // Generate random bytes
    let mut rng = OsRng;
    let mut random_bytes = [0u8; 32];
    rng.fill_bytes(&mut random_bytes);

    // Encode random bytes to Base64
    base64::encode_config(random_bytes, base64::URL_SAFE_NO_PAD)
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get("oauth2-proxy").await.is_ok() {
        api.delete("oauth2-proxy", &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get("oauth2-proxy").await.is_ok() {
        api.delete("oauth2-proxy", &DeleteParams::default()).await?;
    }

    let api: Api<Secret> = Api::namespaced(client.clone(), namespace);
    if api.get("oidc-secret").await.is_ok() {
        api.delete("oidc-secret", &DeleteParams::default()).await?;
    }

    Ok(())
}

fn read_secret_field(secret: &Secret, key: &str) -> Option<String> {
    secret
        .data
        .as_ref()
        .and_then(|data| data.get(key))
        .and_then(|value| String::from_utf8(value.0.clone()).ok())
}

fn redirect_uri_value(hostname_url: &str) -> String {
    format!("{}/oauth2/callback", hostname_url.trim_end_matches('/'))
}
