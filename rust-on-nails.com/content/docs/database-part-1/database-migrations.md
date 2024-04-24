+++
title = "Database Migrations"
description = "Database Migrations"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 50
sort_by = "weight"


[extra]
toc = true
top = false
+++

[DBMate](https://github.com/amacneil/dbmate) is a database migration tool, to keep your database schema in sync across multiple developers and your production servers. We have pre-installed it in your `devcontainer`

After that we can setup our migrations folder and the create a users migration.

## Create a Migration

```sh
dbmate new user_tables
# Creating migration: crates/db/migrations/20220330110026_user_tables.sql
```

Edit the SQL file that was generated for you and add the following.

```sql
-- migrate:up
CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO users(email) VALUES('test1@test1.com');
INSERT INTO users(email) VALUES('test2@test1.com');
INSERT INTO users(email) VALUES('test3@test1.com');

-- migrate:down
DROP TABLE users;
```

## Run the Migrations

List the migrations so we can see which have run.

```sh
dbmate status
#[ ] 20220330110026_user_tables.sql
#
#Applied: 0
#Pending: 1
```

Run our new migration.

```sh
dbmate up
#Applying: 20220330110026_user_tables.sql
```

And check that it worked.

```sh
psql $DATABASE_URL -c 'SELECT count(*) FROM users;'
```

And you should see

```sh
 count 
-------
      3
(1 row)
```

Your project folders should now look like this.

```sh
├── .devcontainer/
│   └── ...
└── crates/
│         web-server/
│         │  └── main.rs
│         └── Cargo.toml
│         db/
│         ├── migrations
│         │   └── 20220330110026_user_tables.sql
│         └── schema.sql
├── Cargo.toml
└── Cargo.lock
```
