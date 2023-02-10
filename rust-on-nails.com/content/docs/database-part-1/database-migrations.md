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

```
$ dbmate new user_tables
Creating migration: crates/db/migrations/20220330110026_user_tables.sql
```

Edit the SQL file that was generated for you and add the following.

```sql
-- migrate:up

CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    hashed_password VARCHAR NOT NULL, 
    reset_password_selector VARCHAR,
    reset_password_sent_at TIMESTAMP,
    reset_password_validator_hash VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO users(email, hashed_password) VALUES('test1@test1.com', 'aasdsaddasad');
INSERT INTO users(email, hashed_password) VALUES('test2@test1.com', 'aasdsaddasad');
INSERT INTO users(email, hashed_password) VALUES('test3@test1.com', 'aasdsaddasad');

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY, 
    session_verifier VARCHAR NOT NULL, 
    user_id INT NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otp_code_encrypted VARCHAR NOT NULL,
    otp_code_attempts INTEGER NOT NULL DEFAULT 0,
    otp_code_confirmed BOOLEAN NOT NULL DEFAULT false,
    otp_code_sent BOOLEAN NOT NULL DEFAULT false
);

COMMENT ON TABLE sessions IS 'The users login sessions';
COMMENT ON COLUMN sessions.session_verifier IS ' The session is a 32 byte random number stored in their cookie. This is the sha256 hash of that value.';
COMMENT ON COLUMN sessions.otp_code_encrypted IS 'A 6 digit code that is encrypted here to prevent attackers with read access to the database being able to use it.';
COMMENT ON COLUMN sessions.otp_code_attempts IS 'We count OTP attempts to prevent brute forcing.';
COMMENT ON COLUMN sessions.otp_code_confirmed IS 'Once the user enters the correct value this gets set to true.';
COMMENT ON COLUMN sessions.otp_code_sent IS 'Have we sent the OTP code?';


-- migrate:down
DROP TABLE users;
DROP TABLE sessions;
```

## Run the Migrations

List the migrations so we can see which have run.

```
$ dbmate status
[ ] 20220330110026_user_tables.sql

Applied: 0
Pending: 1
```

Run our new migration.

```
$ dbmate up
Applying: 20220330110026_user_tables.sql
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
├── .devcontainer/
│   └── ...
└── crates/
│         axum-server/
│         │  └── main.rs
│         └── Cargo.toml
│         db/
│         ├── migrations
│         │   └── 20220330110026_user_tables.sql
│         └── schema.sql
├── Cargo.toml
└── Cargo.lock
```