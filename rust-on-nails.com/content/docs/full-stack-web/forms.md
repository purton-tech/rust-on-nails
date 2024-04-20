+++
title = "Forms"
description = "Forms"
date = 2021-05-01T08:00:00+00:00
updated = 2023-08-12T08:00:00+00:00
draft = false
weight = 80
sort_by = "weight"


[extra]
toc = true
top = false
+++

Browsers have support for client side [validation of form data built in](https://developer.mozilla.org/en-US/docs/Learn/Forms/Form_validation). We can use this along with server side validation to give the user a nice experience and ensure security on the back end.


## Browser Validation

In the following form we use an email type and a required attribute. The browser will now block form submission until the field is filled in with a valid email address and password.

```html
<form>
  <label for="user_email">Email:</label>
  <input id="user_email" name="email" type="email" required>
  <button>Submit</button>
</form>
```

We can write this same form using Dioxus. Update `crates/ui-components/src/users.rs` with a form to add users.

```rust
use crate::layout::Layout;
use db::User;
use dioxus::prelude::*;

struct Props {
    users: Vec<User>,
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

                // 👇 this is our new form
                form {
                    action: "/sign_up",
                    method: "POST",
                    label { r#for: "user_email", "Email:" }
                    input { id: "user_email", name: "email", r#type: "email", required: "true" }
                    button { "Submit" }
                }
            }
        })
    }

    // Construct our component and render it to a string.
    let mut app = VirtualDom::new_with_props(app, Props { users });
    let _ = app.rebuild();
    format!("<!DOCTYPE html><html lang='en'>{}</html>", dioxus_ssr::render(&app))
}
```

_Note:_ `for` and `type` are Rust keywords. We must prefix them with `r#` so Rust knows that we want the raw string literal of "for" and "type".

## Handling form submission

We need to install [serde](https://serde.rs/) to transform the HTTP body into a Rust struct.

```bash
cd crates/web-ui
cargo add serde@1.0 --features derive
```

[Axum](https://github.com/tokio-rs/axum) has support for [Handlers](https://docs.rs/axum/latest/axum/handler/index.html). We can use those in a lot of different ways and one way is for form implementations. We are going to create a `create_form` handler to save new users to our database.

Update `crates/web-ui/src/main.rs`

```rust
mod config;
mod errors;

use crate::errors::CustomError;
// 👇 update axum imports
use axum::{
    extract::Extension,
    response::Html,
    response::Redirect,
    routing::get,
    routing::post,
    Form,
    Router,
};
// 👇 new import
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .route("/sign_up", post(accept_form)) // 👈 add new route
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    // We now return HTML
    Ok(Html(ui_components::users::users(users)))
}

// 👇 create new SignUp struct
#[derive(Deserialize )]
struct SignUp {
    email: String,
}

// 👇 handle form submission
async fn accept_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Redirect, CustomError> {
    let client = pool.get().await?;

    let email = form.email;
    // TODO - accept a password and hash it
    let hashed_password = String::from("aaaa");
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str(), &hashed_password.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/"))
}
```

We are using `db::queries::users::create_user()` in our `accept_form` handler. We must also update `crates/db/queries/users.sql` to include our actual SQL query

```sql
--: User()

--! get_users : User
SELECT 
    id, 
    email
FROM users;

-- 👇 add `create_user` query
--! create_user
INSERT INTO users (email, hashed_password)
VALUES(:email, :hashed_password);
```

You should get results like the screenshot below.

![Users Form](/form-screenshot.png)

If you add an email to the form and press submit, the server should handle that request and update the users table.

## Server Side Validation

Our web form validates that the user input is an email. We should also check that the user input is an email on the server. We can use [Validator](https://github.com/Keats/validator) which will allow us to add validation to the `SignUp` struct.


Install the `Validator` crate.

```bash
cd crates/web-ui
cargo add validator@0.15 --features derive
```

Update `crates/web-ui/src/main.rs`

```rust
mod config;
mod errors;

use crate::errors::CustomError;
// 👇 update axum imports
use axum::{
    extract::Extension,
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    response::Redirect,
    response::Response,
    routing::get,
    routing::post,
    Form,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
// 👇 new import
use validator::Validate;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .route("/sign_up", post(accept_form))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    // We now return HTML
    Ok(Html(ui_components::users::users(users)))
}

#[derive(Deserialize, Validate)]
struct SignUp {
    #[validate(email)] // 👈 add validate annotation
    email: String,
}

async fn accept_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,

    // 👇 change `Redirect` to `Response`
) -> Result<Response, CustomError> {
    // 👇 add our error handling
    if form.validate().is_err() {
        return Ok((StatusCode::BAD_REQUEST, "Bad request").into_response());
    }

    let client = pool.get().await?;

    let email = form.email;
    // TODO - accept a password and hash it
    let hashed_password = String::from("aaaa");
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str(), &hashed_password.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/").into_response()) // 👈 add `.into_response()`
}
```

And we can test that our validation works by sending a request directly to the server (bypassing the browser form):

```bash
curl http://localhost:3000/sign_up --data-raw 'email=bad-data'
```
