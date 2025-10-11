use crate::error::Error;
use k8s_openapi::api::core::v1::{Secret, Service};
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
pub const KEYCLOAK_SERVICE_NAME: &str = "keycloak-service";
pub const KEYCLOAK_INTERNAL_URL: &str = "http://keycloak-service.keycloak.svc.cluster.local:8080";
pub const KEYCLOAK_REALM_BASE_PATH: &str = "/oidc/realms";

const KEYCLOAK_INSTALL_HINT: &str =
    "Keycloak operator is not installed. Run `nails-cli init` or apply the manifests in `crates/nails-cli/config` before reconciling.";

#[derive(Clone, Debug)]
pub struct RealmConfig {
    pub namespace: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub allow_registration: bool,
}

pub async fn bootstrap(client: Client) -> Result<(), Error> {
    cleanup_bootstrap_conflicts(client.clone()).await?;
    apply_keycloak_cr(client.clone()).await?;
    apply_static_realms(client.clone()).await?;

    Ok(())
}

pub async fn ensure_realm(client: Client, config: &RealmConfig) -> Result<(), Error> {
    ensure_namespace_service(client.clone(), &config.namespace).await?;
    let realm_api = keycloak_realm_import_api(client, KEYCLOAK_NAMESPACE);
    let resource_name = format!("keycloak-realm-{}", config.realm);

    let realm_resource = json!({
        "apiVersion": format!("{}/{}", KEYCLOAK_API_GROUP, "v2alpha1"),
        "kind": "KeycloakRealmImport",
        "metadata": {
            "name": resource_name,
            "namespace": KEYCLOAK_NAMESPACE,
        },
        "spec": {
            "keycloakCRName": KEYCLOAK_NAME,
            "realm": {
                "realm": config.realm,
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

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    cleanup_namespace_service(client.clone(), namespace).await?;
    let realm_api = keycloak_realm_import_api(client, KEYCLOAK_NAMESPACE);
    let resource_name = format!("keycloak-realm-{}", namespace);
    if realm_api.get(&resource_name).await.is_ok() {
        realm_api
            .delete(&resource_name, &DeleteParams::default())
            .await?;
    }
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

async fn ensure_namespace_service(client: Client, namespace: &str) -> Result<(), Error> {
    let services: Api<Service> = Api::namespaced(client.clone(), namespace);
    let service = json!({
        "apiVersion": "v1",
        "kind": "Service",
        "metadata": {
            "name": KEYCLOAK_SERVICE_NAME,
            "namespace": namespace,
        },
        "spec": {
            "type": "ExternalName",
            "externalName": format!(
                "{}.{}.svc.cluster.local",
                KEYCLOAK_SERVICE_NAME, KEYCLOAK_NAMESPACE
            )
        }
    });

    services
        .patch(
            KEYCLOAK_SERVICE_NAME,
            &PatchParams::apply(crate::MANAGER).force(),
            &Patch::Apply(service),
        )
        .await?;

    Ok(())
}

async fn cleanup_namespace_service(client: Client, namespace: &str) -> Result<(), Error> {
    let services: Api<Service> = Api::namespaced(client, namespace);
    if services
        .get(KEYCLOAK_SERVICE_NAME)
        .await
        .map(|_| ())
        .is_ok()
    {
        let _ = services
            .delete(KEYCLOAK_SERVICE_NAME, &DeleteParams::default())
            .await;
    }
    Ok(())
}
