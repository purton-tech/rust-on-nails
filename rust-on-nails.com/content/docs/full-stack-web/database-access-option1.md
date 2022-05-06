+++
title = "Database Access (Option 1)"
description = "Database Access (Option 1)"
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

Add the following to your `app/Cargo.toml`

```toml
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

In a folder called `queries` a file called `users.sql` and add the following content.

```sql
--! get_users(id) { id, email }
SELECT 
    id, email
FROM 
    users
WHERE
    id < $1
```

This will generate a function called `get_user` which will run the SQL query. Note cornucopia checks the query at code generation time against postgres.

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

Add a `create_pool` function to `app/src/config.rs` will we use this to convert our `DATABASE_URL` env var into something cornucopia can use for connection pooling.

```rust
pub fn create_pool(&self) -> deadpool_postgres::Pool {

    // Example to parse
    // APP_DATABASE_URL=postgresql://cloak:testpassword@db:5432/cloak?sslmode=disable
    let mut cfg = deadpool_postgres::Config::new();
    let url: Vec<&str> = if self.app_database_url.starts_with("postgresql://") {
        self.app_database_url.split("postgresql://").collect()
    } else {
        self.app_database_url.split("postgres://").collect()
    };
    let split_on_at: Vec<&str> = url[1].split("@").collect();
    let user_and_pass: Vec<&str> = split_on_at[0].split(":").collect();

    let split_on_slash: Vec<&str> = split_on_at[1].split("/").collect();
    let host_and_port: Vec<&str> = split_on_slash[0].split(":").collect();
    let dbname_and_params: Vec<&str> = split_on_slash[1].split("?").collect();

    // we need to repalce %40 with @ so this works on Azure Postgres
    cfg.user = Some(String::from(user_and_pass[0].replace("%40", "@")));
    cfg.password = Some(String::from(user_and_pass[1]));
    cfg.host = Some(String::from(host_and_port[0]));
    cfg.port = Some(host_and_port[1].parse::<u16>().unwrap());
    cfg.dbname = Some(String::from(dbname_and_params[0]));

    if self.app_database_url.contains("sslmode=require") {
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

        let tls_config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let tls = MakeRustlsConnect::new(tls_config);
        return cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), tls).unwrap();
    } else {
        return cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls).unwrap();
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
mod error;

use axum::extract::Extension;
use axum::{response::Html, routing::get, Router};
use deadpool_postgres::Pool;
use std::net::SocketAddr;
use crate::cornucopia::queries;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(pool.clone()))
        .layer(Extension(config));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Extension(pool): Extension<Pool>) -> Result<Html<String>, error::CustomError> {
    let users = queries::users::get_users(&pool, 10).await?;

    let html = format!("<h1>Hello, World! We Have {} Users</h1>", users.len());

    Ok(Html(html))
}

// Include the generated source code
pub mod cornucopia {
    include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
}
```