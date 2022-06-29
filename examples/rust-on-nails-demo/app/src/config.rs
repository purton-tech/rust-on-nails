use std::env;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        Config {
            database_url,
        }
    }

    pub fn create_pool(&self) -> deadpool_postgres::Pool {
    
        let config = tokio_postgres::Config::from_str(&self.database_url).unwrap();
    
        let manager = if self.database_url.contains("sslmode=require") {
            let mut root_store = rustls::RootCertStore::empty();
            root_store.add_server_trust_anchors(
                webpki_roots::TLS_SERVER_ROOTS
                    .0
                    .iter()
                    .map(|ta| {
                        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                            ta.subject,
                            ta.spki,
                            ta.name_constraints,
                        )
                    })
            );
    
            let tls_config = rustls::ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(root_store)
                .with_no_client_auth();
            let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);
            deadpool_postgres::Manager::new(config, tls)
        } else {
            deadpool_postgres::Manager::new(config, tokio_postgres::NoTls)
        };
    
        deadpool_postgres::Pool::builder(manager).build().unwrap()
    }
}