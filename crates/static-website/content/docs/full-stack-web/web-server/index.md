# The Web Server and Routing

We looked at [Actix Web](https://actix.rs/), [Tokio Axum](https://github.com/tokio-rs/axum) and [Rocket](https://rocket.rs/). Axum was chosen as it's very actively maintained and has the fastest incremental build times. 

Most rust web server projects operate in a similar way. That is you configure a route and a function that will respond to that route.

The functions that respond to routes can have parameters. These parameters which might be `structs`, database pools or form data are passed to the function by the framework. 

## Handling Configuration

We'll separate our configuration into its own file. create `crates/web-server/src/config.rs`

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
use clorinde::deadpool_postgres::PoolError;
use clorinde::tokio_postgres::Error as TokioPostgresError;

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
cargo add axum-extra@0.9 -F form,typed-routing
cargo add tokio@1 --no-default-features -F macros,rt-multi-thread
cargo add tokio-util@0.7 --no-default-features -F io
cargo add tower-livereload@0.9
cargo add serde@1 -F "derive"
cargo add --path ../clorinde
```

And replace your `crates/web-server/src/main.rs` with the following

```rust
mod config;
mod errors;
mod root;

use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};
use clorinde::deadpool_postgres::Manager;
use clorinde::tokio_postgres::NoTls;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pg_config: clorinde::tokio_postgres::Config = config
        .database_url
        .parse()
        .expect("DATABASE_URL is invalid");
    let manager = Manager::new(pg_config, NoTls);
    let pool = clorinde::deadpool_postgres::Pool::builder(manager)
        .build()
        .expect("Failed to build database pool");

    // build our application with a route
    let app = Router::new()
        .route("/", get(root::loader))
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
```

## Loaders

create a file called `root.rs`. This is where we will have the logic that will load data from the database and pass it to the pages so they can render.

In this example we will just return JSON for now.

```rust
use crate::errors::CustomError;
use axum::{Extension, Json};
use clorinde::{deadpool_postgres::Pool, queries::users::User};

pub async fn loader(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;

    let users = clorinde::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    Ok(Json(users))
}
```

## Watching for changes and Hot Reload

We could use `cargo run` to start our server but ideally we'd like our server to re-start when we make changes and for the browser to reload itself.

Our development environment (container) comes preinstalled with [Just](https://github.com/casey/just) which is a command runner.

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

![Users](./axum-screenshot.png)
