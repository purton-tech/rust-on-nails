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

## Add an image

Add the following to `crates/web-assets/images/avatar.svg`

```svg
<?xml version="1.0" encoding="utf-8"?>
<svg width="800px" height="800px" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
  <g id="avatar" transform="translate(-1407 -182)">
    <circle id="Ellipse_16" data-name="Ellipse 16" cx="15" cy="15" r="15" transform="translate(1408 183)" fill="#e8f7f9" stroke="#333" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"/>
    <g id="Group_49" data-name="Group 49">
      <circle id="Ellipse_17" data-name="Ellipse 17" cx="4.565" cy="4.565" r="4.565" transform="translate(1418.435 192.13)" fill="#fff1b6" stroke="#333" stroke-miterlimit="10" stroke-width="2"/>
      <path id="Path_53" data-name="Path 53" d="M1423,213a14.928,14.928,0,0,0,9.4-3.323,9.773,9.773,0,0,0-18.808,0A14.928,14.928,0,0,0,1423,213Z" fill="#fff1b6" stroke="#333" stroke-miterlimit="10" stroke-width="2"/>
    </g>
  </g>
</svg>
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
cargo add --build ructe@0.17 --no-default-features -F mime03
```

Ructe will now take our assets and turn them into rust functions. It handles creating a hash for the assets so we get good browser cache busting.

## Export the Assets

We needs to export our assets from our crate overwrite the `crates/web-assets/src/lib.rs`

```rust
include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub use templates::statics as files;
```

## Configuring a route for our assets

Back to our `web-server` crate.

create a new file `crates/web-server/src/static_files.rs` and add the following function.

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

And add the following route in `crates/web-server/src/main.rs` in the section where we defined our routes.

```rust
.route("/static/*path", get(static_files::static_path))
```

And change the `mod` section so it includes the following.

```rust
mod static_files;
```

## Using our image

Let's add the image to the page.

In our `crates/web-pages` directory run...

```sh
cargo add --path ../web-assets
```

And update the `crates/web-pages/src/users.rs` so it includes our image.

```rust
use crate::layout::Layout;
use db::User;
use dioxus::prelude::*;
use web_assets::files::avatar_svg;

// Take a Vec<User> and create an HTML table.
#[component]
pub fn IndexPage(users: Vec<User>) -> Element {
    rsx! {
        Layout {    // <-- Use our layout
            title: "Users Table",
            table {
                thead {
                    tr {
                        th { "ID" }
                        th { "Email" }
                    }
                }
                tbody {
                    for user in users {
                        tr {
                            td {
                                img {
                                    src: format!("/static/{}", avatar_svg.name),
                                    width: "16",
                                    height: "16"
                                }
                                strong {
                                    "{user.id}"
                                }
                            }
                            td {
                                "{user.email}"
                            }
                        }
                    }
                }
            }
        }
    }
}
```

## The Finished Result

That was a lot of work to put images on the screen but don't forget we now have a typesafe way to access images. 

![Screenshot](../screenshot-with-images.png)