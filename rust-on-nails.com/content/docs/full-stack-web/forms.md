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

Browsers have support for client side [validation of form data built in](https://developer.mozilla.org/en-US/docs/Learn/Forms/Form_validation). We can use this along with server side validation to give the user a nice experience and ensure security on the backend.


## Browser Validation

In the following form we use a pattern and a required attribute. The browser will now block form submission until the field is filled in with Banana or Cherry.

```html
<form>
  <label for="choose">Would you prefer a banana or a cherry?</label>
  <input id="choose" name="i_like" required pattern="[Bb]anana|[Cc]herry">
  <button>Submit</button>
</form>
```

## Handling form submission

[Axum](https://github.com/tokio-rs/axum) has support for [Handlers](https://docs.rs/axum/latest/axum/handler/index.html). We can use those in a lot of different ways and one way is for form implementations. The example below shows how we use a struct to handle the form data passed in to the `accept_form` function.

```rust
use axum::{
    extract::Form,
    handler::post,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct FruitPreferences {
    i_like: String,
}

async fn accept_form(form: Form<FruitPreferences>) {
    let fruit_preferences: FruitPreferences = form.0;

    // ...
}

let app = Router::new().route("/sign_up", post(accept_form));
```

## Handling validation server side

We can use [Validator](https://github.com/Keats/validator) which will allow us to add validation to the strjuct that we use to process form requests.

In your app/Cargo.toml add the following

```toml
validator = { version = "0", features = ["derive"] }
```

And then we can chnage our FruitPreferences to

```rust
#[derive(Deserialize, Validate)]
struct FruitPreferences {
    #[validate(length(min = 1, message = "The i_like field is mandatory"))]
    i_like: String,
}
```

Then we can use logic like

```rust
if form.validate().is_ok() {
    ...
}
```

There are many validations that can be performed with this pattern.