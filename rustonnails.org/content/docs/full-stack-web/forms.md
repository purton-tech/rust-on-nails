+++
title = "Forms"
description = "Forms"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 80
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

[Axum](https://github.com/tokio-rs/axum) has support for [Handlers](https://docs.rs/axum/latest/axum/handler/index.html). We can use those in a lot of different ways and one way is for form implementations. The example below shows how we use a struct to handle the form data passed in to the `accept_form` function.

```rust
use axum::{
    extract::Form,
    handler::post,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct SignUp {
    username: String,
    password: String,
}

async fn accept_form(form: Form<SignUp>) {
    let sign_up: SignUp = form.0;

    // ...
}

let app = Router::new().route("/sign_up", post(accept_form));
```