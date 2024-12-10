+++
title = "Forms and Actions"
description = "Forms and Actions"
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

We can write this same form using Dioxus. Update `crates/web-pages/src/root.rs` with a form to add users.

```rust
use crate::{layout::Layout, render};
use db::User;
use dioxus::prelude::*;
use web_assets::files::favicon_svg;

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
                                img {
                                    src: favicon_svg.name,
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

            // 👇 this is our new form
            form {
                action: "/sign_up",
                method: "POST",
                label { r#for: "user_email", "Email:" }
                input { id: "user_email", name: "email", r#type: "email", required: "true" }
                button { "Submit" }
            }
        }
    };

    render(page)
}
```

_Note:_ `for` and `type` are Rust keywords. We must prefix them with `r#` so Rust knows that we want the raw string literal of "for" and "type".

## Handling form submission with Actions


[Axum](https://github.com/tokio-rs/axum) has support for [Handlers](https://docs.rs/axum/latest/axum/handler/index.html). We can use those in a lot of different ways and one way is for form implementations. We are going to create a `create_form` handler to save new users to our database.

Create a new file `crates/web-server/src/new_user.rs`

```rust
use serde::Deserialize;

use crate::errors::CustomError;
use axum::{
    extract::Extension,
    response::Redirect,
    Form,
};

// 👇 create new SignUp struct
#[derive(Deserialize )]
pub struct SignUp {
    email: String,
}

// 👇 handle form submission
pub async fn process_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Redirect, CustomError> {
    let client = pool.get().await?;

    let email = form.email;
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/"))
}
```

## Add the form handling to our routes

In `crates/web-server/main.rs` add a the new module to the top of the file:

```rust
mod new_user;
```

Add `post` to our `use` section.

```rust
use axum::{extract::Extension, routing::{get, post}, Router};
```

And add another route like the following to the list of routes to catch the post of the form so that the Router now looks like:

```rust
    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .route("/static/*path", get(static_files::static_path))
        .route("/sign_up", post(new_user::process_form))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));
```

In `crates/web-server/Cargo.toml` we also need to update the Axum dependency to add the form feature:

```rust
axum = { version = "0.7", default-features = false, features = ["json","http1","tokio","form"] }
```

The compiler will complain because we haven't added the database code to handle form submission.

## Create the database code

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
INSERT INTO 
    users (email)
VALUES
    (:email);
```

You should get results like the screenshot below.

![Users Form](/form-screenshot.png)

If you add an email to the form and press submit, the server should handle that request and update the users table.

## Server Side Validation

Our web form validates that the user input is an email. We should also check that the user input is an email on the server. We can use [Validator](https://github.com/Keats/validator) which will allow us to add validation to the `SignUp` struct.


Install the `Validator` crate.

```bash
cd crates/web-server
cargo add validator@0.15 --features derive
```

Update `crates/web-server/src/new_user.rs` and add validation.

```rust
use serde::Deserialize;
use validator::Validate;

use crate::errors::CustomError;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Form,
};

#[derive(Deserialize, Validate)]
pub struct SignUp {
    #[validate(email)] // 👈 add validate annotation
    email: String,
}

// 👇 handle form submission
pub async fn process_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Response, CustomError> {
    
    // 👇 add our error handling
    if form.validate().is_err() {
        return Ok((StatusCode::BAD_REQUEST, "Bad request").into_response());
    }

    let client = pool.get().await?;

    let email = form.email;
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/").into_response())
}
```

And we can test that our validation works by sending a request directly to the server (bypassing the browser form):

```bash
curl http://localhost:3000/sign_up --data-raw 'email=bad-data'
```
