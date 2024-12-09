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
cargo add axum@0.7 --no-default-features -F json,http1,tokio
cargo add axum-extra@0.9 --F form
cargo add tokio@1 --no-default-features -F macros,rt-multi-thread
cargo add tokio-util@0.7 --no-default-features
cargo add tower-livereload@0.9
cargo add serde@1 -F "derive"
cargo add --path ../db
```

And replace your `crates/web-server/src/main.rs` with the following

```rust
mod config;
mod errors;

use std::net::SocketAddr;

use crate::errors::CustomError;
use axum::{routing::get, Extension, Json, Router};
use db::User;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    Ok(Json(users))
}
```

## Watching for changes and Hot Reload

We could use `cargo run` to start our server but ideally we'd like our server to re-start when we make changes and for the browser to reload itself.

We've installed [Just](https://github.com/casey/just) which is a command runner.

Issue the following command to create a justfile with an entry to run our server.

```sh
echo -e 'watch:\n    mold -run cargo watch --workdir /workspace/ -w crates/web-server -w crates/db --no-gitignore -x "run --bin web-server"' > Justfile
```

You should get a `Justfile`like the following.

```Justfile
watch:
    mold -run cargo watch --workdir /workspace/ -w crates/web-server -w crates/db --no-gitignore -x "run --bin web-server"
```
Run the server


```sh
just watch
```

The server will run up. It's setup for hot reload so when you change the code, the browser will automatically update.

It also uses a very fast linker called [Mold](https://github.com/rui314/mold) to speed up our incremental build times.

And you should be able to point your browser at `http://localhost:3000` and see the web server deliver a plain text list of users.

![Users](/axum-screenshot.png)
