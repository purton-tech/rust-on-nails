pub mod application;
pub mod cloudflare;
pub mod database;
pub mod deployment;
pub mod ingress;
pub mod keycloak;
pub mod keycloak_db;
pub mod llm;
pub mod mailhog;
pub mod network_policy;
pub mod nginx;
pub mod oauth2_proxy;
pub mod observability;
pub mod pgadmin;

const OAUTH2_PROXY_IMAGE: &str = "quay.io/oauth2-proxy/oauth2-proxy:v7.5.1";
const PGADMIN_IMAGE: &str = "dpage/pgadmin4:8";
const LLM_API_IMAGE: &str = "ghcr.io/nails/llm-api:1.1.1";
