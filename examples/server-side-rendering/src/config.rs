use std::env;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    // Configure SMTP for email.
    pub smtp_config: Option<SmtpConfig>,
}

#[derive(Clone, Debug)]
pub struct SmtpConfig {
    // Configure SMTP for email.
    pub host: String,
    pub port: u16,
    pub tls_off: bool,
    pub username: String,
    pub password: String,
}

impl SmtpConfig {
    pub fn new() -> Option<SmtpConfig> {
        let host = env::var("SMTP_HOST");
        let username = env::var("SMTP_USERNAME");
        let password = env::var("SMTP_PASSWORD");
        let smtp_port = env::var("SMTP_PORT");

        if let (Ok(host), Ok(username), Ok(password), Ok(smtp_port)) =
            (host, username, password, smtp_port)
        {
            Some(SmtpConfig {
                host,
                port: smtp_port.parse::<u16>().unwrap(),
                tls_off: env::var("SMTP_TLS_OFF").is_ok(),
                username,
                password,
            })
        } else {
            None
        }
    }
}

impl Config {
    // Initialise form oiur environment
    pub fn new() -> Config {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        Config {
            database_url,
            smtp_config: SmtpConfig::new(),
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
