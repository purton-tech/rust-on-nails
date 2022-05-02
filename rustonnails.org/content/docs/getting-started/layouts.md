+++
title = "Layouts"
description = "AdiDoks is a Zola theme helping you build modern documentation websites, which is a port of the Hugo theme Doks for Zola."
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 110
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = 'AdiDoks is a Zola theme helping you build modern documentation websites, which is a port of the Hugo theme <a href="https://github.com/h-enk/doks">Doks</a> for Zola.'
toc = true
top = false
+++

## Layouts

Layouts are pieces that fit together (for example header, footer, menus, etc) to make a complete view. An application may have as many layouts as needed. 

In Nails a layout is just a function that takes HTML content and returns more HTML content. Let's put together our cache busting strategy with our asset pipeline into a Layout we can use.

Create `app/src/layout.rs` with the following

```rust
use axum::response::Html;

pub fn layout(title: &str, content: &str) -> Html<String> {
    let layout = Layout {
        title,
        content,
    };
    Html(layout.to_string())
}

markup::define! {

    Layout<'a>(
        title: &'a str,
        content: &'a str,
    )
    {
        @markup::doctype()

        html[lang="en"] {

            head {
                meta [ charset="utf-8" ] {}
                meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
                meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
                title { {title} }

                script [ src = crate::statics::get_index_js(), type="text/javascript", async=""] {}

                link [ rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()] {}
            }
            body {
                main {
                    {markup::raw(content)}
                }
            }
        }
    }
}

```

And we need to change our `app/src/main.rs` to include the routes for our asset pipeline and to call out to the new layout.

```rust
mod config;
mod error;
mod models;
mod layout;

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
        .merge(statics::asset_pipeline_routes())
        .merge(statics::image_routes())
        .layer(Extension(db_pool))
        .layer(Extension(config));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Extension(pool): Extension<PgPool>) -> Result<Html<String>, error::CustomError> {
    let users = models::user::User::get_users(&pool, 10).await?;

    let html = format!("<h1>Hello, World! We Have {} Users</h1>", users.len());

    Ok(layout::layout("Test", &html))
}

// Error here disabled with "rust-analyzer.diagnostics.disabled": ["macro-error"]
// in .vscode/settings.json
pub mod statics {
    include!(concat!(env!("OUT_DIR"), "/statics.rs"));
}
```