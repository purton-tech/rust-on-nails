use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Nails application custom resource specification.
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "nails-cli.dev",
    version = "v1",
    kind = "NailsApp",
    plural = "nailsapps",
    derive = "PartialEq",
    namespaced
)]
pub struct NailsAppSpec {
    pub replicas: i32,
    pub version: String,
    pub gpu: Option<bool>,
    pub saas: Option<bool>,
    pub disable_ingress: Option<bool>,
    pub pgadmin: Option<bool>,
    pub observability: Option<bool>,
    pub development: Option<bool>,
    pub testing: Option<bool>,
    pub primary_db_disk_size: i32,
    pub keycloak_db_disk_size: i32,
    #[serde(rename = "hostname-url")]
    pub hostname_url: String,
    #[serde(rename = "hash-app")]
    pub hash_app: String,
    #[serde(rename = "hash-app-pipeline")]
    pub hash_app_pipeline_job: String,
    #[serde(rename = "hash-app-db-migrations")]
    pub hash_app_db_migrations: String,
}
