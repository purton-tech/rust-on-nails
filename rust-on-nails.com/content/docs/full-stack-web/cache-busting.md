+++
title = "Cache Busting and Images"
description = "Cache Busting and Images"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 100
sort_by = "weight"


[extra]
toc = true
top = false
+++

[NOTE - This section needs to be tested since we migrated to using crates]

Cache busting is where we invalidate a cached file and force the browser to retrieve the file from the server. We can instruct the browser to bypass the cache by simply changing the filename. To the browser, this is a completely new resource so it will fetch the resource from the server. The most common way to do this is to add the hash of the file to the URL.

## Using Ructe for Cache Busting

Let's turn our `asset-pipeline` folder into a Rust crate. We do this because we can use `Ructe` to generate code that allows to access assets in  a typesafe way. Ructe also handles hashing so that we never have to worry about the browser deploying the wrong CSS or Javascript.

Run the following...


```sh
$ cargo init --lib crates/asset-pipeline
Created library package
```

Create a  `crates/asset-pipeline/build.rs` so that the `main` method looks like the following.

```rust
fn main() -> Result<()>  {

    cornucopia()?;

    let mut ructe = Ructe::from_env().unwrap();
    let mut statics = ructe.statics().unwrap();
    statics.add_files("dist").unwrap();
    statics.add_files("images").unwrap();
    ructe.compile_templates("templates").unwrap();

    Ok(())
}
```

And add the following to `crates/asset-pipeline/Cargo.toml` in the dependencies section.

```toml
# Used by ructe for image mime type detection
mime = "0.3.0"
```

Ructe will now take our assets and turn them into rust functions. It handles creating a hash for the assets so we get good browser cache busting.

## Using the Assets

```
use super::statics::*;

dbg!(index_css.name) -> index.234532455.css
```

## Configuring a route for our assets

In `crates/axum-server/main.rs` create the following function.

```rust
async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(body::boxed(Body::from(data.content)))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap()
    }
}
```

And add the following route also in `crates/axum-server/main.rs`

```rust
.route("/static/*path", get(static_path))
```

And change the `use` section so it looks like the following.

```rust
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::{response::Html, response::IntoResponse, routing::get, Router};
use deadpool_postgres::Pool;
use std::net::SocketAddr;
use axum::body::{self, Body, Empty};
use axum::http::{header, HeaderValue, Response, StatusCode};
use assets::templates::statics::StaticFile;
```