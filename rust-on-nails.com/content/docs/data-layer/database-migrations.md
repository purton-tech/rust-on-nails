+++
title = "Database Migrations"
description = "Database Migrations"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 50
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

[Dbmate](https://github.com/amacneil/dbmate) is a database migration tool, to keep your database schema in sync across multiple developers and your production servers. We have pre-installed it in your `devcontainer`

After that we can setup our migrations folder and the create a users migration.

## Create a Migration

```
$ dbmate new initial_tables
Creating migration: db/migrations/20220330110026_initial_tables.sql
```

Edit the SQL file that was generated for you and add the following.

```sql
-- migrate:up
CREATE TABLE  World (
  id integer NOT NULL,
  randomNumber integer NOT NULL default 0,
  PRIMARY KEY  (id)
);
GRANT ALL PRIVILEGES ON World to benchmarkdbuser;

INSERT INTO World (id, randomnumber)
SELECT x.id, least(floor(random() * 10000 + 1), 10000) FROM generate_series(1,10000) as x(id);

CREATE TABLE Fortune (
  id integer NOT NULL,
  message varchar(2048) NOT NULL,
  PRIMARY KEY  (id)
);
GRANT ALL PRIVILEGES ON Fortune to benchmarkdbuser;

INSERT INTO Fortune (id, message) VALUES (1, 'fortune: No such file or directory');
INSERT INTO Fortune (id, message) VALUES (2, 'A computer scientist is someone who fixes things that aren''t broken.');
INSERT INTO Fortune (id, message) VALUES (3, 'After enough decimal places, nobody gives a damn.');
INSERT INTO Fortune (id, message) VALUES (4, 'A bad random number generator: 1, 1, 1, 1, 1, 4.33e+67, 1, 1, 1');
INSERT INTO Fortune (id, message) VALUES (5, 'A computer program does what you tell it to do, not what you want it to do.');
INSERT INTO Fortune (id, message) VALUES (6, 'Emacs is a nice operating system, but I prefer UNIX. — Tom Christaensen');
INSERT INTO Fortune (id, message) VALUES (7, 'Any program that runs right is obsolete.');
INSERT INTO Fortune (id, message) VALUES (8, 'A list is only as strong as its weakest link. — Donald Knuth');
INSERT INTO Fortune (id, message) VALUES (9, 'Feature: A bug with seniority.');
INSERT INTO Fortune (id, message) VALUES (10, 'Computers make very fast, very accurate mistakes.');
INSERT INTO Fortune (id, message) VALUES (11, '<script>alert("This should not be displayed in a browser alert box.");</script>');
INSERT INTO Fortune (id, message) VALUES (12, 'フレームワークのベンチマーク');

-- migrate:down
DROP TABLE World;
DROP TABLE Fortune;
```

## Run the Migrations

List the migrations so we can see which have run.

```
$ dbmate status
[ ] 20220330110026_initial_tables.sql

Applied: 0
Pending: 1
```

Run our new migration.

```
$ dbmate up
Applying: 20220330110026_initial_tables.sql
```

And check that it worked.

```
$ psql $DATABASE_URL -c 'SELECT count(*) FROM Fortune;'
 count 
-------
     12
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
│   │   └── 20220330110026_initial_tables.sql
│   └── schema.sql
├── Cargo.toml
└── Cargo.lock
```