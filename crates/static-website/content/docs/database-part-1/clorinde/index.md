# Database Access

[Clorinde](https://github.com/halcyonnouveau/clorinde) is a code generator that takes small snippets of SQL and turns them into Rust functions.

We'll turn our `crates/db` folder into a crate so we can keep all our database logic in one place.

Run the following

```sh
cargo init --lib crates/db
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
```

## Testing our database crate

```rust
/// Connection pool configuration.
///
/// This is just a simple example config, please look at
/// `tokio_postgres` and `deadpool_postgres` for details.
use clorinde::deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use clorinde::tokio_postgres::NoTls;

pub async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("db-owner"));
    cfg.password = Some(String::from("testpassword"));
    cfg.host = Some(String::from("host.docker.internal"));
    cfg.port = Some(30021);
    cfg.dbname = Some(String::from("stack-app"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clorinde::queries::users::get_users;
    #[tokio::test]
    async fn load_users() {
        // You can learn which database connection types are compatible with Clorinde in the book
        // https://halcyonnouveau.github.io/clorinde/using_queries/db_connections.html
        let pool = create_pool().await.unwrap();
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