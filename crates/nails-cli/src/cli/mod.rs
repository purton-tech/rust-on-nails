pub mod apply;
pub mod install;
pub mod licence;

use clap::{Parser, Subcommand};

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
    /// Run a cut down version for integration testing
    #[arg(long, default_value_t = false)]
    testing: bool,
    /// Install ingress
    #[arg(long, default_value_t = false)]
    disable_ingress: bool,
    /// The setup needed for development. See CONTRIBUTING.md in the main project.
    #[arg(long, default_value_t = false)]
    development: bool,
    /// Namespace for the application deployment
    #[arg(long, default_value = "nails")]
    namespace: String,
    /// Namespace for the operator resources
    #[arg(long, default_value = "nails-system")]
    operator_namespace: String,
    /// The number of application replicas
    #[arg(long, default_value_t = 1)]
    replicas: i32,
    /// Install A GPU based inference engine?
    #[arg(long, default_value_t = false)]
    gpu: bool,
    /// Are we a saas
    #[arg(long, default_value_t = false)]
    saas: bool,
    /// Install pgAdmin?
    #[arg(long, default_value_t = false)]
    pgadmin: bool,
    /// Install Observability?
    #[arg(long, default_value_t = false)]
    observability: bool,
    /// The hostname we are deploying on. By default use the local ip address
    #[arg(long)]
    hostname_url: Option<String>,
    /// Disk size for the primary Postgres database (in GB)
    #[arg(long, default_value_t = 1)]
    primary_db_disk_size: i32,
    /// Disk size for the Keycloak Postgres database (in GB)
    #[arg(long, default_value_t = 1)]
    keycloak_db_disk_size: i32,
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
    /// Sign a licence JSON using a private key
    SignLicence(licence::SignerOpts),
}
