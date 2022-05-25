+++
title = "Database Access"
description = "Database Access"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 55
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

[Cornucopia](https://github.com/LouisGariepy/cornucopia) is a code generator that takes small snippets of SQL and turns them into Rust functions.

## Installation

Add the following to your `app/Cargo.toml` below the `[dependencies]` 

```toml
tokio = { version = "1", default-features = false, features = ["macros", "rt-multi-thread"] }

# Used by cornucopia and the main app
futures = "0.3"

# Access to the database https://github.com/LouisGariepy/cornucopia
deadpool-postgres = { version = "0", features = ["serde"] }
#postgres-types = { version = "0", features = ["derive"] }
tokio-postgres = { version = "0.7", features = [
    "with-time-0_3",
] }
tokio-postgres-rustls = "0"
time = { version = "0", default-features = false,  features = ["formatting"] }
cornucopia_client = "0"
rustls = "0"
webpki-roots = "0"
```

## Creating a SQL definition

In a folder called `queries` a file called `fortunes.sql` and add the following content.

```sql
--! fortunes() { id, message } *
SELECT 
    id, message
FROM 
    Fortune
```

This will generate a function called `fortunes` which will run the SQL query. Note cornucopia checks the query at code generation time against postgres.

## Updating build.rs

Create a `app/build.rs` file and add the following content. This file we compile our .sql files into rust code whenever they chnage.

```rust
use std::env;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {

    cornucopia()?;

    Ok(())
}

fn cornucopia() -> Result<(), std::io::Error> {
    // For the sake of simplicity, this example uses the defaults.
    let queries_path = "queries";

    // Again, for simplicity, we generate the module in our project, but
    // we could've also generated it elsewhere if we wanted to.
    // For example, you could make the destination the `target` folder
    // and include the generated file with a `include_str` statement in your project.

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    let db_url = env::var_os("DATABASE_URL").unwrap();

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    // Call cornucopia. Use whatever CLI command you need.
    let output = std::process::Command::new("cornucopia")
        .arg("generate")
        .arg("-d")
        .arg(file_path)
        .arg("live")
        .arg("--url")
        .arg(db_url)
        .output()?;

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    Ok(())
}
```

## Updating our config handling

Add the following code to `app/src/config.rs` will we use this to convert our `DATABASE_URL` env var into something cornucopia can use for connection pooling.

```rust
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
```

## Folder Structure

You should now have a folder structure something like this.

```sh
.
├── .devcontainer/
│   └── ...
├── app
│   ├── queries/
│   │   └── users.sql
│   ├── src/
│   │   ├── main.rs
│   │   └── config.rs
│   ├── build.rs
├── db/
│   └── ...
├── Cargo.toml
└── Cargo.lock
```

## Calling the Database from main.rs

```rust
mod config;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    let client = pool.get().await.unwrap();

    let fortunes = queries::fortunes::fortunes(&client).await.unwrap();

    dbg!(fortunes);
}

// Include the generated source code
include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));

```