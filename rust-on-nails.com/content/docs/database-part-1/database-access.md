+++
title = "Database Access"
description = "Database Access"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 55
sort_by = "weight"


[extra]
toc = true
top = false
+++

[Cornucopia](https://github.com/cornucopia-rs/cornucopia) is a code generator that takes small snippets of SQL and turns them into Rust functions.

## Installation

Install `cornucopia` into your project.

```sh
cargo add cornucopia_async
```

## Creating a SQL definition

In a folder called `db/queries` create a file called `organisations.sql` and add the following content.

```sql
--: User(first_name?, last_name?)

--! get_users : User
SELECT 
    u.id, 
    ou.organisation_id, 
    u.email, 
    u.first_name,
    u.last_name,
    u.ecdh_public_key, 
    ou.roles
FROM 
    organisation_users ou
LEFT JOIN users u ON u.id = ou.user_id
WHERE
    ou.organisation_id = :organisation_id;
```

Cornucopia will use the above definition to generata a Rust function called `get_users` to access the database. Note cornucopia checks the query at code generation time against Postgres.

## Turning our `db` folder into a crate.

We can turn out `db` folder into a rust crate. This is a separate compilation unit which will help both spee up compile times and also enforce a nice separation of copncerns.

From within the db folder run

```
cargo init --lib --force
```

## Updating build.rs

Create a `db/build.rs` file and add the following content. This file we compile our .sql files into rust code whenever they change.

```rust
use std::env;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {

    cornucopia()?;

    Ok(())
}

fn cornucopia() -> Result<(), std::io::Error> {

    let queries_path = "queries";
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");
    let db_url = env::var_os("DATABASE_URL").unwrap();

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("--derive_ser")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
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

Add the following code to `db/lib.rs` will we use this to convert our `DATABASE_URL` env var into something cornucopia can use for connection pooling.

```rust
use std::str::FromStr;

pub use deadpool_postgres::{Pool, Transaction, PoolError};
pub use tokio_postgres::Error as TokioPostgresError;
pub use cornucopia_async::Params;

pub use queries::organisations::Organisation;

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();

    let manager = if database_url.contains("sslmode=require") {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(
            |ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            },
        ));

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

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
```

## Folder Structure

You should now have a folder structure something like this.

```sh
.
├── .devcontainer/
│   └── ...
├── app
│   ├── src/
│   │   ├── main.rs
│   │   └── config.rs
│   ├── build.rs
├── db/
│   ├── migrations/
│   │   └── organisations.sql
│   ├── queries/
│   │   └── fortunes.sql
│   ├── build.rs
│   └── lib.rs
├── Cargo.toml
└── Cargo.lock
```

## Calling the Database from main.rs

First add the client side dependencies to our project

```sh
cargo add tokio_postgres
cargo add deadpool_postgres
cargo add tokio_postgres_rustls
cargo add postgres_types
cargo add tokio --features macros,rt-multi-thread
cargo add rustls
cargo add webpki_roots
cargo add futures
cargo add serde --features derive
```

And then update the `main.rs` so it looks like the following.

```rust
mod config;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    let client = pool.get().await.unwrap();

    let fortunes = queries::fortunes::fortunes()
        .bind(&client)
        .all()
        .await
        .unwrap();

    dbg!(fortunes);
}

// Include the generated source code
include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));

```

Call `cargo run` and you should see

```sh
[src/main.rs:13] fortunes = [
    Fortunes {
        id: 1,
        message: "fortune: No such file or directory",
    },
    Fortunes {
        id: 2,
        message: "A computer scientist is someone who fixes things that aren't broken.",
    },
    Fortunes {
        id: 3,
        message: "After enough decimal places, nobody gives a damn.",
    },
    Fortunes {
        id: 4,
        message: "A bad random number generator: 1, 1, 1, 1, 1, 4.33e+67, 1, 1, 1",
    },
    Fortunes {
        id: 5,
        message: "A computer program does what you tell it to do, not what you want it to do.",
    },
    Fortunes {
        id: 6,
        message: "Emacs is a nice operating system, but I prefer UNIX. — Tom Christaensen",
    },
    Fortunes {
        id: 7,
        message: "Any program that runs right is obsolete.",
    },
    Fortunes {
        id: 8,
        message: "A list is only as strong as its weakest link. — Donald Knuth",
    },
    Fortunes {
        id: 9,
        message: "Feature: A bug with seniority.",
    },
    Fortunes {
        id: 10,
        message: "Computers make very fast, very accurate mistakes.",
    },
    Fortunes {
        id: 11,
        message: "<script>alert(\"This should not be displayed in a browser alert box.\");</script>",
    },
    Fortunes {
        id: 12,
        message: "フレームワークのベンチマーク",
    },
]
```