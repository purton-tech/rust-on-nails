# Database Access

[Clorinde](https://github.com/halcyonnouveau/clorinde) is a code generator that takes small snippets of SQL and turns them into Rust functions.

We'll turn our `crates/db` folder into a crate so we can keep all our database logic in one place.

Run the following

```sh
cargo init --lib --vcs none crates/db
```

## Creating a SQL definition

In a folder called `db/queries` create a file called `users.sql` and add the following content.

```sql
--: User()

--! get_users : User
SELECT 
    id, 
    email
FROM auth.users;
```

Clorinde will use the above definition to generate a Rust function called `get_users` to access the database. Note cornucopia checks the query at code generation time against Postgres.

## Run Clorinde

```
clorinde live -q ./crates/db/queries/ -d crates/clorinde
```

## Use Clorinde

Update our `crates/db/Cargo.toml`

```toml
[package]
name = "db"
version = "0.1.0"
edition = "2024"

[dependencies]
clorinde = { version = "0.0.0", path = "../clorinde" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Testing our database crate

```rust
use clorinde::{deadpool_postgres, tokio_postgres};
use std::str::FromStr;

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clorinde::queries::users::get_users;
    #[tokio::test]
    async fn load_users() {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = create_pool(&db_url);
        let client = pool.get().await.unwrap();

        // The `all` method returns queried rows collected into a `Vec`
        let users = get_users().bind(&client).all().await.unwrap();
        dbg!(users);
    }
}
```

## Run the Test

```sh
cargo test -- --nocapture
```

```sh

running 1 test
[crates/db/src/lib.rs:31:9] users = [
    User {
        id: 1,
        email: "test1@test1.com",
    },
    User {
        id: 2,
        email: "test2@test1.com",
    },
    User {
        id: 3,
        email: "test3@test1.com",
    },
]
test tests::load_users ... ok
```