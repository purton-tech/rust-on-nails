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

We'll turn our `crates/db` folder into a crate so we can keep all our database logic in one place.

Run the following

```sh
cargo init --lib crates/db
```

## Installation

Install `cornucopia` into your project `cd` into your `crates/db` folder.

```sh
cd crates/db
cargo add cornucopia_async@0.6
```

## Creating a SQL definition

In a folder called `db/queries` create a file called `users.sql` and add the following content.

```sql
--: User()

--! get_users : User
SELECT 
    id, 
    email
FROM users;
```

Cornucopia will use the above definition to generate a Rust function called `get_users` to access the database. Note cornucopia checks the query at code generation time against Postgres.

## Updating build.rs

Create a `crates/db/build.rs` file and add the following content. This file we compile our .sql files into rust code whenever they change.

```rust
use std::env;
use std::path::Path;

fn main() {
    // Compile our SQL
    cornucopia();
}

fn cornucopia() {
    // For the sake of simplicity, this example uses the defaults.
    let queries_path = "queries";

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    let db_url = env::var_os("DATABASE_URL").unwrap();

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    // Call cornucopia. Use whatever CLI command you need.
    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()
        .unwrap();

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }
}
```

## Add a function to do connection pooling

First add some more crates, make sure you are in the `crates/db` folder.


```sh
cargo add tokio-postgres@0.7
cargo add deadpool-postgres@0.12
cargo add postgres-types@0.2
cargo add tokio@1 --features macros,rt-multi-thread
cargo add futures@0.3
cargo add serde@1 --features derive
```

Add the following code to `crates/db/src/lib.rs` will we use this to convert our `DATABASE_URL` env var into something cornucopia can use for connection pooling.

```rust
use std::str::FromStr;

pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
pub use tokio_postgres::Error as TokioPostgresError;
pub use queries::users::User;

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);
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
└── crates/
│         web-server/
│         │  └── main.rs
│         └── Cargo.toml
│         db/
│         ├── migrations
│         │   └── 20220330110026_user_tables.sql
│         ├── queries
│         │   └── users.sql
│         ├── src
│         │   └── lib.rs
│         └── build.rs
├── Cargo.toml
└── Cargo.lock
```

## Testing our database crate

Make sure everything builds.

```sh
cargo build
```

Add the following code to the bottom of your `crates/db/src/lib.rs`.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn load_users() {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = create_pool(&db_url);

        let client = pool.get().await.unwrap();
        //let transaction = client.transaction().await.unwrap();

        let users = crate::queries::users::get_users()
            .bind(&client)
            .all()
            .await
            .unwrap();

        dbg!(users);
    }
}
```

## Run the test

Run the below to trigger the unit test

```sh
cargo test -- --nocapture
```

### The Results

```sh
Running unittests src/lib.rs (/workspace/target/debug/deps/db-1a59f4c51c8578ce)

running 1 test
[crates/db/src/lib.rs:56] users = [
    User {
        id: 1,
        email: "test1@test1.com",
    },
    User {
        id: 2,
        email: "test2@test1.com",
    },
    User {
        id: 3,
        email: "test3@test1.com",
    },
]

test tests::load_users ... ok
```

Great, from now on adding code to access the database is as simple as creating new `sql` files. The code generator will take care of the details.