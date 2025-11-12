use crate::error::Error;
use crate::{cli::apply, operator::crd::StackAppSpec};
use k8s_openapi::api::{apps::v1::Deployment, core::v1::ConfigMap};
use kube::{
    api::{DeleteParams, Patch, PatchParams},
    Api, Client,
};

const GRAFANA_YAML: &str = include_str!("../../config/grafana.yaml");
const APPLICATION_DASHBOARD_JSON: &str =
    include_str!("../../config/dashboards/application-dashboard.json");
const CONFIG_NAME: &str = "grafana-dashboards";

// Large Language Model
pub async fn deploy(
    client: Client,
    password: Option<String>,
    spec: StackAppSpec,
    namespace: &str,
) -> Result<(), Error> {
    let config_map = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": CONFIG_NAME,
            "namespace": namespace
        },
        "data": {
            "application-dashboard.json": APPLICATION_DASHBOARD_JSON
        }
    });

    let api: Api<ConfigMap> = Api::namespaced(client.clone(), namespace);
    api.patch(
        CONFIG_NAME,
        &PatchParams::apply(crate::MANAGER).force(),
        &Patch::Apply(config_map),
    )
    .await?;

    // If we have the passwords then extract them.
    let password = if let Some(password) = password {
        password
    } else {
        "".to_string()
    };

    let hostname_url = spec
        .auth
        .as_ref()
        .and_then(|auth| auth.hostname_url.clone())
        .unwrap_or_else(|| "http://localhost".to_string());

    let yaml = GRAFANA_YAML.replace("$APPLICATION_PASSWORD", &password);
    let yaml = yaml.replace("$HOSTNAME_URL", &hostname_url);
    let mut yaml = yaml.replace("$ADMIN_PASSWORD", &super::database::rand_hex());

    if let Ok(url) = url::Url::parse(&hostname_url) {
        if let Some(domain) = url.host_str() {
            yaml = yaml.replace("$HOSTNAME_DOMAIN", domain);
        } else {
            yaml = yaml.replace("$HOSTNAME_DOMAIN", "localhost");
        }
    }

    apply::apply(&client, &yaml, Some(namespace)).await.unwrap();

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get(GRAFANA_YAML).await.is_ok() {
        api.delete(GRAFANA_YAML, &DeleteParams::default()).await?;
    }

    Ok(())
}
