+++
title = "Authentication"
description = "Authentication"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 140
sort_by = "weight"


[extra]
toc = true
top = false
+++

Probably the quickest way to add authentication to an application is with [Barricade](https://github.com/purton-tech/barricade). Barricade handles login and registration pages and connects to your Postgres database.

## Installing Barricade

We've already created the tables that Barricade needs in the migrations section. so we just need to add configuration `.devcontainer/docker-compose.yml`.

```yml
  barricade:
    image: purtontech/barricade:1.2.5
    env_file:
        - .env
    depends_on:
      db:
        condition: service_healthy
    ports:
      - "127.0.0.1:9090:9090"
```

We also need to add a health check to our db section so that we know when the database is ready.

```yml
  db:
    ...
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5
```

Add the following to you `.devcontainer/.env`

```sh
# Barricade config
DATABASE_URL=postgresql://postgres:testpassword@db:5432/postgres?sslmode=disable
SECRET_KEY=190a5bf4b3cbb6c0991967ab1c48ab30790af876720f1835cbbf3820f4f5d949

FORWARD_URL=development
FORWARD_PORT=3000
# Any requests that meet the following regular expressions
# with pass through. i.e. They don't require auth.
SKIP_AUTH_FOR=/static*
REDIRECT_URL='/'
```

## Testing Barricade

After rebuilding your *devcontainer* you will need to register as a user. Make sure you server is running again i.e. 

```sh
cd /workspace
cargo run
```

Expose port 9090 from your devcontainer then go to `http://localhost:9090` and sign up.

![Barricade](/login.png)

## Accessing the user from Axum

We need to create a file called `crates/web-server/src/authentication.rs` and add the following code.

```rust
// Extract the barricade 
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct Authentication {
    pub user_id: i32,
}

// From a request extract our authentication token.
#[async_trait]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_id) = parts.headers.get("x-user-id") {
            if let Ok(user_id) = user_id.to_str() {
                if let Ok(user_id) = user_id.parse::<i32>() {
                    return Ok(Authentication { user_id });
                }
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "x-user-id not found or unparseable as i32",
        )
            .into_response())
    }
}
```

Connect this into our `crates/web-server/src/main.rs` by adding a line `mod authentication` at the top of the `main.rs`.

## Using the Authentication Extractor

Form within a handler you can access the user id like so...

```rust

pub async fn index(
    Extension(pool): Extension<Pool>,
    current_user: authentication::Authentication,
) -> Result<Html<String>, CustomError> {
  ...
  dbg!(current_user.user_id);
  ...
}
```

The `user_id` is the database ID of the user in the `users` table.
