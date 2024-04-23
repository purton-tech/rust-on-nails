+++
title = "Assets and Cache Busting"
description = "Assets and Cache Busting"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 80
sort_by = "weight"


[extra]
toc = true
top = false
+++

We'll want to add assets to our project such as images, css and perhaps javascript (but hopefully not javascript).

Cache busting is where we invalidate a cached file and force the browser to retrieve the file from the server. We can instruct the browser to bypass the cache by simply changing the filename. To the browser, this is a completely new resource so it will fetch the resource from the server. The most common way to do this is to add the hash of the file to the URL.

We can also generate some code so the assets are available in our Rust pages and then we get the added benefit that if the files are deleted or names are changed we get compiler errors.

## Create an Asset Pipeline

```sh
cargo init --lib crates/web-assets
```

## Using Ructe for Cache Busting

We'll use `Ructe` to generate code that allows to access assets in a typesafe way. Ructe also handles hashing so that we never have to worry about the browser deploying the wrong CSS or Images.


Create a  `crates/web-assets/build.rs` so that the `main` method looks like the following.

```rust
use ructe::{Result, Ructe};

fn main() -> Result<()>  {

    let mut ructe = Ructe::from_env().unwrap();
    let mut statics = ructe.statics().unwrap();
    statics.add_files("images").unwrap();
    ructe.compile_templates("images").unwrap();

    Ok(())
}
```

Setup our dependencies

```sh
cd crates/web-assets
cargo add mime@0.3
cargo add --build ructe@0.17 --no-default-features -F mime
```

Ructe will now take our assets and turn them into rust functions. It handles creating a hash for the assets so we get good browser cache busting.

## Export the Assets

We needs to export our assets from our crate overwrite the `crates/web-assets/src/lib.rs`

```rust
include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub use templates::statics as files;
```

## Configuring a route for our assets

Back to our `web-ui` crate.

create a new file `crates/web-ui/src/static_files.rs` and add the following function.

```rust
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::body::Body;
use axum::http::{header, HeaderValue, Response, StatusCode};
use web_assets::templates::statics::StaticFile;

pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(Body::from(data.content))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
```

And add the following route also in `crates/web-ui/src/main.rs`

```rust
.route("/static/*path", get(static_path))
```

And change the `mod` section so it includes the following.

```rust
mod static_files;
```