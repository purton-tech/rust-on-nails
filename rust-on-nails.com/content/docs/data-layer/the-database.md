+++
title = "The Database"
description = "The Database"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 40
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

> If you're not sure which database to use then use Postgres.

The architecture doesn't stop you using MySQL (MariaDB?) or other relational databases. However a relational database is the recommendation.

We already installed Postgres when we installed our *devcontainer*, however we didn't install the Postgres command line client. To do that, add the following to your `.devcontainer/Dockerfile`.

```
# Install psql 14
RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list' \
   && wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - \
   && apt-get -y update \
   && apt-get -y install postgresql-client \
   && apt-get autoremove -y && apt-get clean -y
```

Add the following to your `.devcontainer/.env`

```
DATABASE_URL=postgresql://postgres:postgres@db:5432/postgres?sslmode=disable
```

Restart your devcontainer and you should now have access to Postgres. i.e.

```
psql $DATABASE_URL
psql (14.2 (Debian 14.2-1.pgdg110+1), server 14.1 (Debian 14.1-1.pgdg110+1))
Type "help" for help.

postgres=# \dt
Did not find any relations.
postgres=# \q
```

We will use this pattern over and over. When we add a tool to our solution we add it to the devcontainer this ensures we can always reproduce our development environment. 

## Configuration

To configure our application we pull in environment variables a create a Rust struct. Create a `app/src/config.rs`.

```rust
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    // Initialise form oiur environment
    pub fn new() -> Config {
        let database_url = 
            std::env::var("DATABASE_URL")
            .expect("DATABASE_URL not set");

        Config {
            database_url,
        }
    }
}
```

```sh
.
├── .devcontainer/
│   └── ...
├── app/
│   ├──src/
│   │  ├── main.rs
│   │  └── config.rs <- Our configuration
│   └── Cargo.toml
├── Cargo.toml
└── Cargo.lock
```
