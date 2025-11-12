use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Stack application custom resource specification.
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "stack-cli.dev",
    version = "v1",
    kind = "StackApp",
    plural = "stackapps",
    derive = "PartialEq",
    namespaced
)]
pub struct StackAppSpec {
    pub web: WebContainer,
    pub auth: Option<AuthConfig>,
}

/// Web application container reference.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct WebContainer {
    /// Fully-qualified container image reference (e.g. ghcr.io/org/app:tag)
    pub image: String,
    /// Container port exposed by the application (e.g. 7903)
    pub port: u16,
}

/// Optional authentication configuration.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct AuthConfig {
    /// Public hostname that Cloudflare/Keycloak should use for redirects.
    #[serde(rename = "hostname-url")]
    pub hostname_url: Option<String>,
    /// Static JWT token forwarded by nginx when OIDC is disabled.
    pub jwt: Option<String>,
}
