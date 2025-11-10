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
    #[serde(rename = "hostname-url")]
    pub hostname_url: String,
    pub web: WebContainer,
}

/// Web application container reference.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct WebContainer {
    /// Fully-qualified container image reference (e.g. ghcr.io/org/app:tag)
    pub image: String,
    /// Container port exposed by the application (e.g. 7903)
    pub port: u16,
}
