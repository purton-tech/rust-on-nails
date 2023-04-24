+++
title = "Server Side Components"
description = "Server Side Components"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 70
sort_by = "weight"


[extra]
toc = true
top = false
+++

The [Dioxus](https://dioxuslabs.com/) framework gives us the capability to build user interfaces out of components which can be rendered server side. It's worth looking at their [components documentation](https://dioxuslabs.com/guide/components/index.html).

## Creating a ui-components crate

```sh
cargo init --lib crates/ui-components
```

## Install Dioxus

```sh
cd crates/ui-components
cargo add dioxus@0.2 --features ssr
```

## Creating a Layout Component

A layout defines the surroundings of an HTML page. It's the place to define a common look and feel of your final output. 

create a file called `crates/ui-components/src/layout.rs`.

```rust
#![allow(non_snake_case)]

use dioxus::prelude::*;

// Remember: owned props must implement PartialEq!
#[derive(Props)]
pub struct AppLayoutProps<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn Layout<'a>(cx: Scope<'a, AppLayoutProps<'a>>) -> Element {
    cx.render(rsx!(
        {
            LazyNodes::new(|f| f.text(format_args!("<!DOCTYPE html><html lang='en'>")))
        }
        head {
            title {
                "{cx.props.title}"
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
            &cx.props.children
        }
    ))
}
```

Let's use this layout to create a very simple users screen that will show a table of users.

Make sure you're in the `crates/ui-components` folder and add the `db` crate to your `Cargo.toml` using the following command:

```sh
cargo add --path ../db
```

Create a file `crates/ui-components/src/users.rs`.

```rust
use crate::layout::Layout;
use db::User;
use dioxus::prelude::*;

struct Props {
    users: Vec<User>
}

// Take a Vec<User> and create an HTML table.
pub fn users(users: Vec<User>) -> String {

    // Inner function to create our rsx! component
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
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
                        cx.props.users.iter().map(|user| rsx!(
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
                        ))
                    }
                }
            }
        })
    }

    // Construct our component and render it to a string.
    let mut app = VirtualDom::new_with_props(
        app,
        Props {
            users
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
```

If we update our `crates/ui-components/src/lib.rs` to look like the following...

```rust
mod layout;
pub mod users;
```

Then finally we can change our `axum-server` code to generate HTML rather than JSON.

Make sure you're in the `crates/axum-server` folder and add the `ui_components` crate to your `Cargo.toml` using the following command:

```sh
cargo add --path ../ui_components
```

Update `crates/axum-server/src/main.rs`

```rust
mod config;
mod errors;

use crate::errors::CustomError;
use axum::{response::Html, extract::Extension, routing::get, Router};
use std::net::SocketAddr;

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
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;
    
    // We now return HTML
    Ok(Html(ui_components::users::users(
        users,
    )))
}
```

You should get results like the screenshot below.

![Users](/layout-screenshot.png)

