+++
title = "The Web Server and Routing"
description = "The Web Server and Routing"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 30
sort_by = "weight"


[extra]
toc = true
top = false
+++

We looked at [Actix Web](https://actix.rs/), [Tokio Axum](https://github.com/tokio-rs/axum) and [Rocket](https://rocket.rs/). Axum was chosen as it's very actively maintained and has the fastest incremental build times. 

Most rust web server projects operate in a similar way. That is you configure a route and a function that will respond to that route.

The functions that respond to routes can have parameters. These parameters which might be `structs`, database pools or form data are passed to the function by the framework. 

## Handling Configuration

We'll separate our configuration into it's own file. create `crates/web-server/src/config.rs`

```rust
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

        Config {
            database_url,
        }
    }
}
```

## Handling Errors

Now is a good time to think about how we will handle errors so we don't have to `unwrap` all the time.

Create a file called `crates/web-server/src/errors.rs` and add the following code.

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;
use db::{TokioPostgresError, PoolError};

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            //CustomError::Unauthorized(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Database(ref cause) => {
                write!(f, "Database Error: {}", cause)
            }
        }
    }
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<TokioPostgresError> for CustomError {
    fn from(err: TokioPostgresError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

impl From<PoolError> for CustomError {
    fn from(err: PoolError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}
```

## Install Axum

Make sure you're in the `crates/web-server` folder and add Axum to your `Cargo.toml` using the following command.

```sh
cargo add axum@0.7 --no-default-features -F json
cargo add tokio@1 --no-default-features -F macros,fs,rt-multi-thread
cargo add --path ../db
```

And replace your `crates/web-server/src/main.rs` with the following

```rust
mod config;
mod errors;

use crate::errors::CustomError;
use axum::{extract::Extension, response::Json, routing::get, Router};
use std::net::SocketAddr;
use db::User;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    Ok(Json(users))
}
```

## Watch the Server

We could use `cargo run` to start our server but Rust on Nails comes with a built in alias that will watch your code for changes and restart your server.

It also uses a very fast linker called [Mold](https://github.com/rui314/mold) to speed up our incremental build times.

Issue the following command in your `app` folder.

```sh
cw
```

And you should be able to point your browser at `http://localhost:3000` and see the web server deliver a plain text list of users.

![Users](/axum-screenshot.png)
