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
cargo add dioxus@0.6 --no-default-features -F macro,html,signals
cargo add dioxus-ssr@0.6 --no-default-features
cargo add --path ../db
```

## Creating a Layout Component

A layout defines the surroundings of an HTML page. It's the place to define a common look and feel of your final output. 

create a file called `crates/web-pages/src/layout.rs`.

It's a big one as I've added some styling which we won't use until later.

```rust
#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    title: String,
    fav_icon_src: String,
    stylesheets: Vec<String>,
    js_href: Option<String>,
    header: Element,
    children: Element,
    sidebar: Element,
    sidebar_footer: Element,
    sidebar_header: Element,
}

pub fn Layout(props: LayoutProps) -> Element {
    rsx!(
        head {
            title {
                "{props.title}"
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
            for href in &props.stylesheets {
                link {
                    rel: "stylesheet",
                    href: "{href}",
                    "type": "text/css"
                }
            }
            if let Some(js_href) = props.js_href {
                script {
                    "type": "module",
                    src: "{js_href}"
                }
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "{props.fav_icon_src}"
            }
        }
        body {
            div {
                class: "flex h-screen overflow-hidden",
                nav {
                    id: "sidebar",
                    class: "
                        border-r border-base-300
                        fixed
                        bg-base-200
                        inset-y-0
                        left-0
                        w-64
                        transform
                        -translate-x-full
                        transition-transform
                        duration-200
                        ease-in-out
                        flex
                        flex-col
                        lg:translate-x-0
                        lg:static
                        lg:inset-auto
                        lg:transform-none
                        z-20",
                    div {
                        class: "flex items-center p-4",
                        {props.sidebar_header}
                    }
                    div {
                        class: "flex-1 overflow-y-auto",
                        {props.sidebar}
                    }
                    div {
                        class: "p-4",
                        {props.sidebar_footer}
                    }
                }
                main {
                    id: "main-content",
                    class: "flex-1 flex flex-col",
                    header {
                        class: "flex items-center p-4 border-b border-base-300",
                        button {
                            id: "toggleButton",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "lucide lucide-panel-left",
                                rect {
                                    width: "18",
                                    height: "18",
                                    x: "3",
                                    y: "3",
                                    rx: "2",
                                }
                                path {
                                    d: "M9 3v18",
                                }
                            }
                        }
                        {props.header}
                    }
                    section {
                        class: "flex-1 overflow-y-auto",
                        {props.children}
                    }
                }
            }
        }
    )
}
```

Let's use this layout to create a very simple users screen that will show a table of users.

Create a file `crates/web-pages/src/root.rs`. we call it `root.rs` because it's the root of our routes. If we had a route such as `customers` we would call it `customers.rs`.

```rust
use crate::{layout::Layout, render};
use db::User;
use dioxus::prelude::*;

pub fn index(users: Vec<User>) -> String {
    let page = rsx! {
        Layout {    // <-- Use our layout
            title: "Users Table",
            stylesheets: vec![],
            header: rsx!(),
            sidebar: rsx!(),
            sidebar_header: rsx!(),
            sidebar_footer: rsx!(),
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
    };

    render(page)
}
```

If we update our `crates/web-pages/src/lib.rs` to look like the following...

```rust
mod layout;
pub mod root;
use dioxus::prelude::*;

pub fn render_page(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
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
use std::net::SocketAddr;
use web_pages::root;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(loader))
        .layer(Extension(config))
        .layer(LiveReloadLayer::new())
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on... {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn loader(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    let html = root::index(users);

    Ok(Html(html))
}
```

You should get results like the screenshot below.

![Users](/layout-screenshot.png)

