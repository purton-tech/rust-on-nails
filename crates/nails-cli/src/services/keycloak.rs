use std::collections::BTreeMap;

use crate::error::Error;
use crate::operator::crd::NailsAppSpec;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::ByteString;
use kube::api::{DeleteParams, Patch, PatchParams};
use kube::core::dynamic::{ApiResource, DynamicObject};
use kube::core::gvk::GroupVersionKind;
use kube::{Api, Client};
use serde_json::{json, Map, Value};
use url::Url;

const CONFIG_JSON: &str = include_str!("../../keycloak/realm.json");
pub const KEYCLOAK_NAME: &str = "keycloak";
const INITIAL_ADMIN_SECRET: &str = "keycloak-initial-admin";
const KEYCLOAK_SECRET: &str = "keycloak-secrets";

pub async fn deploy(client: Client, spec: NailsAppSpec, namespace: &str) -> Result<(), Error> {
    ensure_admin_secrets(client.clone(), namespace, super::database::rand_hex()).await?;

    apply_keycloak_cr(client.clone(), namespace, spec).await?;
    apply_realm_imports(client.clone(), namespace).await?;

    crate::services::network_policy::default_deny(client, KEYCLOAK_NAME, namespace).await?;

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    let keycloak_api = keycloak_api(client.clone(), namespace);
    if keycloak_api.get(KEYCLOAK_NAME).await.is_ok() {
        keycloak_api
            .delete(KEYCLOAK_NAME, &DeleteParams::default())
            .await?;
    }

    let realm_import_api = keycloak_realm_import_api(client.clone(), namespace);
    for realm_name in realm_names()? {
        let resource_name = format!("keycloak-realm-{}", realm_name);
        if realm_import_api.get(&resource_name).await.is_ok() {
            realm_import_api
                .delete(&resource_name, &DeleteParams::default())
                .await?;
        }
    }

    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    for secret_name in [KEYCLOAK_SECRET, INITIAL_ADMIN_SECRET] {
        if secret_api.get(secret_name).await.is_ok() {
            secret_api
                .delete(secret_name, &DeleteParams::default())
                .await?;
        }
    }

    Ok(())
}

async fn ensure_admin_secrets(
    client: Client,
    namespace: &str,
    fallback_password: String,
) -> Result<(), Error> {
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    let existing_password = match secret_api.get(KEYCLOAK_SECRET).await {
        Ok(secret) => extract_password(secret),
        Err(_) => None,
    };

    let admin_password = existing_password.unwrap_or(fallback_password);

    let mut legacy_secret_data = BTreeMap::new();
    legacy_secret_data.insert("admin-password".to_string(), admin_password.clone());

    let legacy_secret = json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": KEYCLOAK_SECRET,
            "namespace": namespace
        },
        "stringData": legacy_secret_data
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
            "namespace": namespace
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

async fn apply_keycloak_cr(
    client: Client,
    namespace: &str,
    spec: NailsAppSpec,
) -> Result<(), Error> {
    let keycloak_api = keycloak_api(client, namespace);

    let instances = if spec.replicas > 0 { spec.replicas } else { 1 };

    let mut keycloak_resource = json!({
        "apiVersion": "keycloak.org/v2alpha1",
        "kind": "Keycloak",
        "metadata": {
            "name": KEYCLOAK_NAME,
            "namespace": namespace,
            "labels": {
                "app": KEYCLOAK_NAME
            }
        },
        "spec": {
            "instances": instances,
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
            }
        }
    });

    if let Some(spec_obj) = keycloak_resource
        .get_mut("spec")
        .and_then(Value::as_object_mut)
    {
        let mut hostname_config = Map::new();
        hostname_config.insert("strict".into(), Value::Bool(false));
        if let Some(hostname) = parsed_hostname(&spec.hostname_url) {
            hostname_config.insert("hostname".into(), Value::String(hostname));
        }
        hostname_config.insert("backchannelDynamic".into(), Value::Bool(true));
        spec_obj.insert("hostname".into(), Value::Object(hostname_config));
        spec_obj.insert(
            "proxy".into(),
            json!({
                "headers": "xforwarded"
            }),
        );
    }

    keycloak_api
        .patch(
            KEYCLOAK_NAME,
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(keycloak_resource),
        )
        .await?;

    Ok(())
}

async fn apply_realm_imports(client: Client, namespace: &str) -> Result<(), Error> {
    let realm_api = keycloak_realm_import_api(client, namespace);
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
                "namespace": namespace,
            },
            "spec": {
                "keycloakCRName": KEYCLOAK_NAME,
                "realm": realm
            }
        });

        realm_api
            .patch(&resource_name, &params, &Patch::Apply(realm_resource))
            .await?;
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

fn realm_names() -> Result<Vec<String>, Error> {
    realm_values().map(|realms| {
        realms
            .into_iter()
            .filter_map(|realm| {
                realm
                    .get("realm")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })
            .collect()
    })
}

fn parsed_hostname(url: &str) -> Option<String> {
    Url::parse(url)
        .ok()
        .and_then(|parsed| parsed.host_str().map(|host| host.to_string()))
}

fn extract_password(secret: Secret) -> Option<String> {
    secret
        .data
        .and_then(|mut data| data.remove("admin-password"))
        .and_then(|byte_string: ByteString| String::from_utf8(byte_string.0).ok())
}
