pub mod apply;
pub mod install;
pub mod licence;
pub mod status;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub struct CloudflareInstaller {
    /// The cloudflare tuneel token
    #[arg(long)]
    pub token: String,
    /// The tunnel name
    #[arg(long, default_value = "nails")]
    pub name: String,
    /// Namespace for the tunnel deployment
    #[arg(long, default_value = "nails")]
    pub namespace: String,
}

#[derive(Parser)]
pub struct Initializer {
    /// Install ingress
    #[arg(long, default_value_t = false)]
    pub disable_ingress: bool,
    /// Namespace for application components
    #[arg(long, default_value = "nails")]
    pub namespace: String,
    /// Namespace for the operator
    #[arg(long, default_value = "nails-system")]
    pub operator_namespace: String,
    /// Skip installing the operator
    #[arg(long, default_value_t = false)]
    pub no_operator: bool,
}

#[derive(Parser)]
pub struct Installer {
    /// Path to a NailsApp manifest to apply
    #[arg(long)]
    manifest: PathBuf,
    /// The setup needed for development. See CONTRIBUTING.md in the main project.
    #[arg(long, default_value_t = false)]
    development: bool,
    /// Deploy a Cloudflare tunnel during installation
    #[arg(long)]
    cloudflare_token: Option<String>,
    /// Name for the Cloudflare tunnel (defaults to manifest namespace)
    #[arg(long)]
    cloudflare_tunnel_name: Option<String>,
    /// Namespace for the Cloudflare tunnel (defaults to manifest namespace)
    #[arg(long)]
    cloudflare_namespace: Option<String>,
}

#[derive(Parser)]
pub struct StatusArgs {
    /// Namespace where the application components (including cloudflared) are installed
    #[arg(long, default_value = "nails")]
    pub namespace: String,
    /// Namespace where the shared Keycloak installation lives
    #[arg(long, default_value = "keycloak")]
    pub keycloak_namespace: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install the application into Kubernetes
    Install(Installer),
    /// Install the required operators into Kubernetes
    Init(Initializer),
    /// Run the Nails Kubernetes Operator
    Operator {},
    /// Run the Nails Kubernetes Operator
    Cloudflare(CloudflareInstaller),
    /// Show platform connection details (Keycloak credentials, Cloudflare URL)
    Status(StatusArgs),
    /// Sign a licence JSON using a private key
    SignLicence(licence::SignerOpts),
}
