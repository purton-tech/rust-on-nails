+++
title = "HTML Templating"
description = "HTML Templating"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 70
sort_by = "weight"


[extra]
toc = true
top = false
+++

We will use [Ructe](https://github.com/kaj/ructe) for templating. It compiles HTML templates directly into rust code at build time not at run time. It's also pretty fast so we can keep our fast incremental build times.

## Install Ructe

Create a folder called `templates` in `app/`. You should end up with a folder structure like the following.


```sh
.
├── .devcontainer/
│   └── ...
├── app
│   ├── queries/
│   │   └── ...
│   ├── src/
│   │   └── ...
│   ├── templates/
│   │   └── ...
│   ├── Cargo.toml
│   ├── package-lock.json
│   ├── package.json
├── db/
│   └── ...
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

Install Ructe as a build dependency, add the following to the end of `app/Cargo.toml`

```toml
[build-dependencies]
ructe = { version="0.14.0", features = ["mime03"] }
```

Finally replace your `build.rs` with this one.

```rust
use std::env;
use std::path::Path;
use ructe::{Result, Ructe};
use std::path::PathBuf;

fn main() -> Result<()> {

    cornucopia()?;

    let mut ructe = Ructe::from_env().unwrap();
    ructe.compile_templates("templates").unwrap();

    Ok(())
}

fn cornucopia() -> Result<()> {
    // For the sake of simplicity, this example uses the defaults.
    let queries_path = "queries";

    // Again, for simplicity, we generate the module in our project, but
    // we could've also generated it elsewhere if we wanted to.
    // For example, you could make the destination the `target` folder
    // and include the generated file with a `include_str` statement in your project.

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    let db_url = env::var_os("DATABASE_URL").unwrap();

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    // Call cornucopia. Use whatever CLI command you need.
    let output = std::process::Command::new("cornucopia")
        .arg("generate")
        .arg("-d")
        .arg(file_path)
        .arg("live")
        .arg("--url")
        .arg(db_url)
        .output()?;

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    Ok(())
}
```

## Creating a template

Create a file called `app/templates/fortunes/index.rs.html` with the following content.

```html
@use crate::queries::fortunes::Fortunes;

@(title: &str, fortunes: Vec<Fortunes>)

<!DOCTYPE html>
<html>
    <head><title>@title</title></head>
    <body>
        <table>
            <tr><th>id</th><th>message</th></tr>
            @for fortune in fortunes {
                <tr><td>@fortune.id</td><td>@fortune.message</td></tr>
            }
        </table>
    </body>
</html>
```

## Loading the template from the database

We can change our `app/src/main.rs` so that it uses the template to render the page.

```rust
mod config;
mod errors;

use crate::errors::CustomError;
use axum::{extract::Extension, response::Html, routing::get, Router};
use deadpool_postgres::Pool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    // build our application with a route
    let app = Router::new()
        .route("/", get(fortunes))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fortunes(Extension(pool): Extension<Pool>) -> Result<Html<&'static str>, CustomError> {
    let client = pool.get().await?;

    let fortunes = queries::fortunes::fortunes(&client).await?;

    Ok(crate::render(|buf| {
        crate::templates::fortunes::index_html(buf, "Fortunes", fortunes)
    }))
}

pub fn render<F>(f: F) -> Html<&'static str>
where
    F: FnOnce(&mut Vec<u8>) -> Result<(), std::io::Error>,
{
    let mut buf = Vec::new();
    f(&mut buf).expect("Error rendering template");
    let html: String = String::from_utf8_lossy(&buf).into();

    Html(Box::leak(html.into_boxed_str()))
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
```

## View the Results


![Fortunes](/rendered-template.png)