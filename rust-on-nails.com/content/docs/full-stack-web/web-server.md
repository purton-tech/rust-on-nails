+++
title = "The Web Server and Routing"
description = "The Web Server and Routing"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 30
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

We looked at [Actix Web](https://actix.rs/), [Tokio Axum](https://github.com/tokio-rs/axum) and [Rocket](https://rocket.rs/). Axum was chosen as it's very actively maintained and has the fastest incremental build times. 

Most rust web server project operate in a similar way. That is you configure a route and a function that will respond to that route.

The functions that respond to routes can have parameters. These parameters which might be `structs`, database pools or form data are passed to the function by the framework. 

## Handling Errors

Now is a good time to think about how we will handle errros so we don't have to `unwrap` all the time.

Create a file called `app/src/errors.rs` and add the following code.

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

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

impl From<tokio_postgres::Error> for CustomError {
    fn from(err: tokio_postgres::Error) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

impl From<deadpool_postgres::PoolError> for CustomError {
    fn from(err: deadpool_postgres::PoolError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

```

## Install Axum

Add the following to your `app/Cargo.toml`.

```toml
[dependencies]
axum = "0"
```

And replace your `app/src/main.rs` with the following

```rust
mod config;
mod errors;

use crate::errors::CustomError;
use axum::{extract::Extension, response::Html, routing::get, Router};
use deadpool_postgres::Pool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    // build our application with a route
    let app = Router::new()
        .route("/", get(fortunes))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn fortunes(Extension(pool): Extension<Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let fortunes = queries::fortunes::fortunes(&client).await?;

    let fortunes = format!("{:?}", fortunes);

    Ok(Html(fortunes))
}

// Include the generated source code
include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
```

## Watch the Server

We could use `cargo run` to start our server but Rust on Nails comes with a built in alias that will watch your code for chnages and restart your server.

It also uses a very fast linker called (Mold)[https://github.com/rui314/mold] to speed up our incremental build times.

Iss ue the following command in your `app` folder.

```sh
cw
```

And you should be able to point your browser at `http://localhost:3000` and see the web server deliver a plain text list of fortunes.

![Fortunes](/plain-text.png)
