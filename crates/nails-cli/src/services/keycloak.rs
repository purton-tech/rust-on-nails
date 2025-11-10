use crate::error::Error;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::api::networking::v1::NetworkPolicy;
use kube::api::{DeleteParams, Patch, PatchParams};
use kube::core::dynamic::{ApiResource, DynamicObject};
use kube::core::gvk::GroupVersionKind;
use kube::{Api, Client};
use serde_json::{json, Value};

const CONFIG_JSON: &str = include_str!("../../keycloak/realm.json");
const KEYCLOAK_API_GROUP: &str = "k8s.keycloak.org";
pub const KEYCLOAK_NAMESPACE: &str = "keycloak";
pub const KEYCLOAK_NAME: &str = "keycloak";

const KEYCLOAK_INSTALL_HINT: &str =
    "Keycloak operator is not installed. Run `nails-cli init` or apply the manifests in `crates/nails-cli/config` before reconciling.";

pub async fn bootstrap(client: Client) -> Result<(), Error> {
    cleanup_bootstrap_conflicts(client.clone()).await?;
    apply_keycloak_cr(client.clone()).await?;
    apply_static_realms(client.clone()).await?;

    Ok(())
}

async fn apply_keycloak_cr(client: Client) -> Result<(), Error> {
    let keycloak_api = keycloak_api(client, KEYCLOAK_NAMESPACE);

    let keycloak_resource = json!({
        "apiVersion": format!("{}/{}", KEYCLOAK_API_GROUP, "v2alpha1"),
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
            "hostname": {
                "strict": false,
                "backchannelDynamic": false
            },
            "proxy": {
                "headers": "xforwarded"
            },
            "env": [
                {
                    "name": "KC_HTTP_ENABLED",
                    "value": "true"
                },
                {
                    "name": "KC_PROXY",
                    "value": "edge"
                }
            ]
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
            "apiVersion": format!("{}/{}", KEYCLOAK_API_GROUP, "v2alpha1"),
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
    let gvk = GroupVersionKind::gvk(KEYCLOAK_API_GROUP, "v2alpha1", "Keycloak");
    let resource = ApiResource::from_gvk(&gvk);
    Api::namespaced_with(client, namespace, &resource)
}

fn keycloak_realm_import_api(client: Client, namespace: &str) -> Api<DynamicObject> {
    let gvk = GroupVersionKind::gvk(KEYCLOAK_API_GROUP, "v2alpha1", "KeycloakRealmImport");
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

async fn cleanup_bootstrap_conflicts(client: Client) -> Result<(), Error> {
    let secret_api: Api<Secret> = Api::namespaced(client.clone(), KEYCLOAK_NAMESPACE);
    if secret_api
        .get("keycloak-initial-admin")
        .await
        .map(|_| ())
        .is_ok()
    {
        let _ = secret_api
            .delete("keycloak-initial-admin", &DeleteParams::default())
            .await;
    }

    let network_policy_api: Api<NetworkPolicy> = Api::namespaced(client, KEYCLOAK_NAMESPACE);
    if network_policy_api
        .get("keycloak-network-policy")
        .await
        .map(|_| ())
        .is_ok()
    {
        let _ = network_policy_api
            .delete("keycloak-network-policy", &DeleteParams::default())
            .await;
    }

    Ok(())
}
