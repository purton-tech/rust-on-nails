+++
title = "Database Migrations"
description = "AdiDoks is a Zola theme helping you build modern documentation websites, which is a port of the Hugo theme Doks for Zola."
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 50
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = 'AdiDoks is a Zola theme helping you build modern documentation websites, which is a port of the Hugo theme <a href="https://github.com/h-enk/doks">Doks</a> for Zola.'
toc = true
top = false
+++

## Database Migrations

[Dbmate](https://github.com/amacneil/dbmate) is a database migration tool, to keep your database schema in sync across multiple developers and your production servers.

Add the following to your `.devcontainer/Dockerfile` and rebuild your devcontainer

```
RUN sudo curl -fsSL -o /usr/local/bin/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-amd64 \
   && sudo chmod +x /usr/local/bin/dbmate
```

After that we can setup our migrations folder and the create a users migration.

```
$ dbmate new create_users_table
Creating migration: db/migrations/20220330110026_create_users_table.sql
```

Edit the SQL file that was generated for you and add the following.

```sql
-- migrate:up
CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    hashed_password VARCHAR NOT NULL, 
    reset_password_selector VARCHAR,
    reset_password_verifier_hash VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY, 
    session_verifier VARCHAR NOT NULL, 
    user_id INT NOT NULL, 
    otp_code_encrypted VARCHAR NOT NULL,
    otp_code_attempts INTEGER NOT NULL DEFAULT 0,
    otp_code_confirmed BOOLEAN NOT NULL DEFAULT false,
    otp_code_sent BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- migrate:down
DROP TABLE sessions;
DROP TABLE users;
```

List the migrations so we can see which have run.

```
$ dbmate status
[ ] 20220330110026_create_users_table.sql

Applied: 0
Pending: 1
```

Run our new migration.

```
$ dbmate up
Applying: 20220330110026_create_users_table.sql
```

And check that it worked.

```
$ psql $DATABASE_URL -c 'SELECT count(*) FROM users;'
 count 
-------
     0
(1 row)
```

Your project folders should now look like this.

```sh
.
├── .devcontainer/
│   └── ...
├── app/
│   └── ...
├── db/
│   ├── migrations
│   │   └── 20220330110026_create_users_table.sql
│   └── schema.sql
├── Cargo.toml
└── Cargo.lock
```