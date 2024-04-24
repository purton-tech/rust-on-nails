+++
title = "Building Pages"
description = "Building Pages"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 70
sort_by = "weight"


[extra]
toc = true
top = false
+++

One way to build up an application is to think of it in terms of pages. 
Later we can split the pages into re-usable components but let's start with a simple page.

The [Dioxus](https://dioxuslabs.com/) framework gives us the capability to build user interfaces out of components which can be rendered server side. It's worth looking at their [components documentation](https://dioxuslabs.com/guide/components/index.html).

I looked at lots of Rust markup and component libraries. Dioxus stood out as having a lightweight syntax and the ability to create components in a similar way to React.

## Creating a web-pages crate

```sh
cargo init --lib crates/web-pages
```

## Install Dioxus

```sh
cd crates/web-pages
cargo add dioxus@0.5
cargo add dioxus-ssr@0.5
```

## Creating a Layout Component

A layout defines the surroundings of an HTML page. It's the place to define a common look and feel of your final output. 

create a file called `crates/web-pages/src/layout.rs`.

```rust
#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Layout(title: String, children: Element) -> Element {
    rsx!(
        head {
            title {
                "{title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
        }
        body {
            {children}
        }
    )
}

```

Let's use this layout to create a very simple users screen that will show a table of users.

Make sure you're in the `crates/ui-components` folder and add the `db` crate to your `Cargo.toml` using the following command:

```sh
cargo add --path ../db
```

Create a file `crates/web-pages/src/users.rs`.

```rust
use db::User;
use dioxus::prelude::*;

// Take a Vec<User> and create an HTML table.
#[component]
pub fn users(users: Vec<User>) -> Element {
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

If we update our `crates/web-pages/src/lib.rs` to look like the following...

```rust
mod layout;
pub mod users;
use dioxus::prelude::*;

// Method to render components to HTML
pub fn render(mut virtual_dom: VirtualDom) -> String {
    virtual_dom.rebuild_in_place();
    let html = dioxus_ssr::render(&virtual_dom);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

```

Then finally we can change our `web-server` code to generate HTML rather than JSON.

Make sure you're in the `crates/web-server` folder and add the `web-pages` crate to your `Cargo.toml` using the following command:

```sh
cargo add --path ../web-pages
```

Update `crates/web-server/src/main.rs`

```rust
mod config;
mod errors;
use crate::errors::CustomError;
use axum::response::Html;
use axum::{extract::Extension, routing::get, Router};
use dioxus::dioxus_core::VirtualDom;
use std::net::SocketAddr;
use web_pages::{
    render,
    users::{IndexPage, IndexPageProps},
};

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
    println!("listening on... {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    let html = render(VirtualDom::new_with_props(
        IndexPage,
        IndexPageProps { users },
    ));

    Ok(Html(html))
}

```

You should get results like the screenshot below.

![Users](/layout-screenshot.png)

