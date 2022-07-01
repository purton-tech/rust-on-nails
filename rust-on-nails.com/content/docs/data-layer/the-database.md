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

## Why Postgres?

* Postgres supports RLS (Row Level Security) this allows us to implement authorization over our data at the lowest level.
* Support for unstructured data. Postgres can store and search JSON and other types of data.
* PostgreSQL has earned a strong reputation for its proven architecture, reliability, data integrity, robust feature set, extensibility, and the dedication of the open source community behind the software to consistently deliver performant and innovative solutions.

## Test out your Postgres installation

Postgres is pre-installed in your `devcontainer`. To try it out run the below.

```sh
> psql $DATABASE_URL

psql (14.2 (Debian 14.2-1.pgdg110+1), server 14.1 (Debian 14.1-1.pgdg110+1))
Type "help" for help.

postgres=# \dt
Did not find any relations.
postgres=# \q
```