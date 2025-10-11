use std::collections::BTreeMap;

use crate::error::Error;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::ByteString;
use kube::api::{DeleteParams, Patch, PatchParams};
use kube::core::dynamic::{ApiResource, DynamicObject};
use kube::core::gvk::GroupVersionKind;
use kube::{Api, Client};
use serde_json::{json, Value};

const CONFIG_JSON: &str = include_str!("../../keycloak/realm.json");
pub const KEYCLOAK_NAMESPACE: &str = "keycloak";
pub const KEYCLOAK_NAME: &str = "keycloak";
pub const KEYCLOAK_INTERNAL_URL: &str = "http://keycloak-service.keycloak.svc.cluster.local:8080";
pub const KEYCLOAK_REALM_BASE_PATH: &str = "/oidc/realms";

const INITIAL_ADMIN_SECRET: &str = "keycloak-initial-admin";
const KEYCLOAK_SECRET: &str = "keycloak-secrets";
const KEYCLOAK_INSTALL_HINT: &str =
    "Keycloak operator is not installed. Run `nails-cli init` or apply the manifests in `crates/nails-cli/config` before reconciling.";

#[derive(Clone, Debug)]
pub struct RealmConfig {
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub allow_registration: bool,
}

pub async fn bootstrap(client: Client) -> Result<(), Error> {
    ensure_admin_secrets(client.clone(), super::database::rand_hex()).await?;
    apply_keycloak_cr(client.clone()).await?;
    apply_static_realms(client.clone()).await?;

    crate::services::network_policy::default_deny(client, KEYCLOAK_NAME, KEYCLOAK_NAMESPACE)
        .await?;
    Ok(())
}

pub async fn ensure_realm(client: Client, config: &RealmConfig) -> Result<(), Error> {
    let realm_api = keycloak_realm_import_api(client, KEYCLOAK_NAMESPACE);
    let resource_name = format!("keycloak-realm-{}", config.realm);

    let realm_resource = json!({
        "apiVersion": "keycloak.org/v2alpha1",
        "kind": "KeycloakRealmImport",
        "metadata": {
            "name": resource_name,
            "namespace": KEYCLOAK_NAMESPACE,
        },
        "spec": {
            "keycloakCRName": KEYCLOAK_NAME,
            "realm": {
                "realm": config.realm,
                "enabled": true,
                "registrationAllowed": config.allow_registration,
                "registrationEmailAsUsername": true,
                "sslRequired": "none",
                "clients": [
                    {
                        "clientId": config.client_id,
                        "clientAuthenticatorType": "client-secret",
                        "secret": config.client_secret,
                        "redirectUris": config.redirect_uris,
                        "protocol": "openid-connect",
                        "publicClient": false,
                        "directAccessGrantsEnabled": true,
                        "standardFlowEnabled": true,
                        "bearerOnly": false,
                        "consentRequired": false,
                        "frontchannelLogout": true,
                        "webOrigins": ["*"],
                    }
                ]
            }
        }
    });

    match realm_api
        .patch(
            &resource_name,
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(realm_resource),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(kube::Error::Api(err)) if err.code == 404 => {
            Err(Error::DependencyMissing(KEYCLOAK_INSTALL_HINT))
        }
        Err(err) => Err(err.into()),
    }
}

pub async fn delete(client: Client, realm: &str) -> Result<(), Error> {
    let realm_api = keycloak_realm_import_api(client, KEYCLOAK_NAMESPACE);
    let resource_name = format!("keycloak-realm-{}", realm);
    if realm_api.get(&resource_name).await.is_ok() {
        realm_api
            .delete(&resource_name, &DeleteParams::default())
            .await?;
    }
    Ok(())
}

async fn ensure_admin_secrets(client: Client, fallback_password: String) -> Result<(), Error> {
    let secret_api: Api<Secret> = Api::namespaced(client, KEYCLOAK_NAMESPACE);
    let existing_password = match secret_api.get(KEYCLOAK_SECRET).await {
        Ok(secret) => extract_password(secret),
        Err(_) => None,
    };

    let admin_password = existing_password.unwrap_or(fallback_password);

    let mut secret_data = BTreeMap::new();
    secret_data.insert("admin-password".to_string(), admin_password.clone());

    let legacy_secret = json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": KEYCLOAK_SECRET,
            "namespace": KEYCLOAK_NAMESPACE
        },
        "stringData": secret_data
    });

    secret_api
        .patch(
            KEYCLOAK_SECRET,
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(legacy_secret),
        )
        .await?;

    let initial_admin_secret = json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": INITIAL_ADMIN_SECRET,
            "namespace": KEYCLOAK_NAMESPACE
        },
        "stringData": {
            "username": "admin",
            "password": admin_password
        }
    });

    secret_api
        .patch(
            INITIAL_ADMIN_SECRET,
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(initial_admin_secret),
        )
        .await?;

    Ok(())
}

async fn apply_keycloak_cr(client: Client) -> Result<(), Error> {
    let keycloak_api = keycloak_api(client, KEYCLOAK_NAMESPACE);

    let keycloak_resource = json!({
        "apiVersion": "keycloak.org/v2alpha1",
        "kind": "Keycloak",
        "metadata": {
            "name": KEYCLOAK_NAME,
            "namespace": KEYCLOAK_NAMESPACE,
            "labels": {
                "app": KEYCLOAK_NAME
            }
        },
        "spec": {
            "instances": 1,
            "http": {
                "relativePath": "/oidc"
            },
            "db": {
                "vendor": "postgres",
                "host": "keycloak-db-cluster-rw",
                "port": 5432,
                "database": "keycloak",
                "usernameSecret": {
                    "name": "keycloak-db-owner",
                    "key": "username"
                },
                "passwordSecret": {
                    "name": "keycloak-db-owner",
                    "key": "password"
                }
            },
            "initialAdmin": {
                "secret": {
                    "name": INITIAL_ADMIN_SECRET
                }
            },
            "hostname": {
                "strict": false,
                "backchannelDynamic": true
            },
            "proxy": {
                "headers": "xforwarded"
            }
        }
    });

    match keycloak_api
        .patch(
            KEYCLOAK_NAME,
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(keycloak_resource),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(kube::Error::Api(err)) if err.code == 404 => {
            Err(Error::DependencyMissing(KEYCLOAK_INSTALL_HINT))
        }
        Err(err) => Err(err.into()),
    }
}

async fn apply_static_realms(client: Client) -> Result<(), Error> {
    let realm_api = keycloak_realm_import_api(client, KEYCLOAK_NAMESPACE);
    let params = PatchParams::apply(crate::MANAGER).force();

    for realm in realm_values()? {
        let realm_name = realm
            .get("realm")
            .and_then(Value::as_str)
            .unwrap_or("realm")
            .to_string();
        let resource_name = format!("keycloak-realm-{}", realm_name);

        let realm_resource = json!({
            "apiVersion": "keycloak.org/v2alpha1",
            "kind": "KeycloakRealmImport",
            "metadata": {
                "name": resource_name,
                "namespace": KEYCLOAK_NAMESPACE,
            },
            "spec": {
                "keycloakCRName": KEYCLOAK_NAME,
                "realm": realm
            }
        });

        match realm_api
            .patch(&resource_name, &params, &Patch::Apply(realm_resource))
            .await
        {
            Ok(_) => {}
            Err(kube::Error::Api(err)) if err.code == 404 => {
                return Err(Error::DependencyMissing(KEYCLOAK_INSTALL_HINT));
            }
            Err(err) => return Err(err.into()),
        }
    }

    Ok(())
}

fn keycloak_api(client: Client, namespace: &str) -> Api<DynamicObject> {
    let gvk = GroupVersionKind::gvk("keycloak.org", "v2alpha1", "Keycloak");
    let resource = ApiResource::from_gvk(&gvk);
    Api::namespaced_with(client, namespace, &resource)
}

fn keycloak_realm_import_api(client: Client, namespace: &str) -> Api<DynamicObject> {
    let gvk = GroupVersionKind::gvk("keycloak.org", "v2alpha1", "KeycloakRealmImport");
    let resource = ApiResource::from_gvk(&gvk);
    Api::namespaced_with(client, namespace, &resource)
}

fn realm_values() -> Result<Vec<Value>, Error> {
    let value: Value = serde_json::from_str(CONFIG_JSON)?;
    let realms = match value {
        Value::Array(items) => items,
        other => vec![other],
    };
    Ok(realms)
}

fn extract_password(secret: Secret) -> Option<String> {
    secret
        .data
        .and_then(|mut data| data.remove("admin-password"))
        .and_then(|byte_string: ByteString| String::from_utf8(byte_string.0).ok())
}
