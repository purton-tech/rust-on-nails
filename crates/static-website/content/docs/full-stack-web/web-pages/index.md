# Building Pages

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
cargo add --path ../clorinde
```

## Creating a Layout Component

A layout defines the surroundings of an HTML page. It's the place to define a common look and feel of your final output. 

create a file called `crates/web-pages/src/layout.rs`.

It's a big one as I've added some styling which we won't use until later.

```rust
#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use web_assets::files::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SideBar {
    Users,
}

impl std::fmt::Display for SideBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[component]
pub fn Layout(title: String, children: Element, selected_item: SideBar) -> Element {
    rsx! {
        BaseLayout {
            title,
            //web_assembly: (
            //    web_assets::statics::web_islands_js.name.into(),
            //    web_assets::statics::web_islands_bg_wasm.name.into()
            //),
            stylesheets: vec![tailwind_css.name.to_string()],
            header: rsx!(
                nav {
                    aria_label: "breadcrumb",
                    ol {
                        class: "flex flex-wrap items-center gap-1.5 break-words text-sm sm:gap-2.5",
                        li {
                            class: "ml-3 items-center gap-1.5 hidden md:block",
                            "Your Application"
                        }
                        li {
                            ">"
                        }
                        li {
                            "Users"
                        }
                    }
                }
            ),
            sidebar: rsx!(
                NavGroup {
                    heading: "Your Menu",
                    content:  rsx!(
                        NavItem {
                            id: SideBar::Users.to_string(),
                            selected_item_id: selected_item.to_string(),
                            href: "/",
                            icon: favicon_svg.name,
                            title: "Users"
                        }
                    )
                }
            ),
            sidebar_header: rsx!(
                div {
                    class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-neutral text-neutral-content",
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
                        class: "lucide lucide-gallery-vertical-end size-4",
                        path {
                            d: "M7 2h10",
                        }
                        path {
                            d: "M5 6h14",
                        }
                        rect {
                            width: "18",
                            height: "12",
                            x: "3",
                            y: "10",
                            rx: "2",
                        }
                    }
                }
                div {
                    class: "ml-3 flex flex-col gap-0.5 leading-none",
                    span {
                        class: "font-semibold uppercase",
                        "Your Application"
                    }
                    span {
                        class: "",
                        "v1.0.1"
                    }
                }
            ),
            sidebar_footer: rsx!(
                div {
                    class: "text-center text-sm",
                    "data-alert": "",
                    "You can place items at the bottom"
                }
            ),
            div {
                class: "px-4 h-full",
                {children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BaseLayoutProps {
    title: String,
    fav_icon_src: Option<String>,
    stylesheets: Vec<String>,
    js_href: Option<String>,
    web_assembly: Option<(String, String)>,
    header: Element,
    children: Element,
    sidebar: Element,
    sidebar_footer: Element,
    sidebar_header: Element,
}

pub fn BaseLayout(props: BaseLayoutProps) -> Element {
    // Do we need to handle web assembly?
    let hydrate: Option<String> = if let Some((js, wasm)) = props.web_assembly {
        Some(format!(
            "import init, {{ hydrate }} from '{}'; await init('{}');hydrate();",
            js, wasm
        ))
    } else {
        None
    };

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
            if let Some(fav_icon_src) = props.fav_icon_src {
                link {
                    rel: "icon",
                    "type": "image/svg+xml",
                    href: "{fav_icon_src}"
                }
            }
            if let Some(hydrate) = hydrate {
                script {
                    "type": "module",
                    dangerous_inner_html: hydrate
                }
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
use clorinde::queries::users::User;
use dioxus::prelude::*;

pub fn index(users: Vec<User>) -> String {
    let page = rsx! {
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
    };

    render(page)
}
```

If we update our `crates/web-pages/src/lib.rs` to look like the following...

```rust
mod layout;
pub mod root;
use dioxus::prelude::*;

pub fn render(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}
```

Then finally we can change our `web-server` code to generate HTML rather than JSON.

Make sure you're in the `crates/web-server` folder and add the `web-pages` crate to your `Cargo.toml` using the following command:

```sh
cargo add --path ../web-pages
```

Update `crates/web-server/src/root.rs`

```rust
use crate::errors::CustomError;
use axum::{response::Html, Extension};
use web_pages::root;
use clorinde::deadpool_postgres::Pool;

pub async fn loader(
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = clorinde::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    let html = root::index(users);

    Ok(Html(html))
}
```

You should get results like the screenshot below.

![Users](./layout-screenshot.png)

