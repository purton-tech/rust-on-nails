+++
title = "Database Access (Option 1)"
description = "Database Access (Option 1)"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 60
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

[SQLx](https://github.com/launchbadge/sqlx) is amazing. It's An async, pure Rust SQL crate featuring compile-time checked queries without a DSL. Supports PostgreSQL, MySQL, SQLite, and MSSQL.

Add the following to your `app/Cargo.toml`

```
sqlx = { version = "0", default-features = false,  features = [ "runtime-tokio-rustls", "postgres", "macros", "chrono" ] }
```

Add the following code to a file called `app/src/models/user.rs`

```rust
use sqlx::{Error, PgPool};

pub struct User {
    pub id: i32,
    pub email: String,
}

impl User {
    pub async fn get_users(pool: &PgPool, user_id: u32) -> Result<Vec<User>, Error> {
        Ok(sqlx::query_as!(
            User,
            "
                SELECT 
                    id, email
                FROM 
                    users
                WHERE
                    id < $1
            ",
            user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }
}
```

You'll also need to create a file called `app/src/models/mod.rs` and add the following

```
pub mod user;
```

We recommend creating a folder called models and adding a file for each type of entity you will manipulate with SQLx.

```sh
.
├── .devcontainer/
│   └── ...
├── app
│   ├── src/
│   │   ├── models/
│   │   │   ├── main.rs
│   │   │   └── mod.rs
│   │   ├── main.rs
│   │   └── config.rs
├── db/
│   └── ...
├── Cargo.toml
└── Cargo.lock
```

Now we can tie our database and front end together. Update your `app/src/main.rs` to become

```rust
mod config;
mod error;
mod models;

use axum::extract::Extension;
use axum::{response::Html, routing::get, Router};
use sqlx::PgPool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let db_pool = PgPool::connect(&config.database_url)
        .await
        .expect("Problem connecting to the database");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(db_pool))
        .layer(Extension(config));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Extension(pool): Extension<PgPool>) -> Result<Html<String>, error::CustomError> {
    let users = models::user::User::get_users(&pool, 10).await?;

    let html = format!("<h1>Hello, World! We Have {} Users</h1>", users.len());

    Ok(Html(html))
}
```

The main thing to note here is that our handler gets passed in a database connection auto magically by the framework.

We also need to create a place to hold our application errors. Create a file called `app/src/error.rs` like so.

```rust
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

#[derive(Debug)]
pub enum CustomError {
    Database(String),
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

// Any errors from sqlx get converted to CustomError
impl From<sqlx::Error> for CustomError {
    fn from(err: sqlx::Error) -> CustomError {
        CustomError::Database(err.to_string())
    }
}
```

Execute `cargo run` and you can access the application from `http://localhost:3000` in your browser.
