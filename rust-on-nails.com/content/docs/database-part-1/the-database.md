+++
title = "The Database"
description = "The Database"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 40
sort_by = "weight"


[extra]
toc = true
top = false
+++

## Why Postgres?

* PostgreSQL supports most of the major features of SQL:2016. Out of 177 mandatory features required for full Core conformance, PostgreSQL conforms to at least 170. In addition, there is a long list of supported optional features. It might be worth noting that at the time of writing, no current version of any database management system claims full conformance to Core SQL:2016.
* Scales Vertically and Horizontally with tools such as Citus (https://www.citusdata.com/)
* Postgres supports RLS (Row Level Security) allowing you to implement authorization at the database level.
* It can support many 1000's of transaction per second running on commodity hardware.
* NoSQL support. Postgres can store and search JSON and other types of unstructured data.
* Postgres has earned a strong reputation for its proven architecture, reliability, data integrity, robust feature set, extensibility, and the dedication of the open source community behind the software to consistently deliver performant and innovative solutions.

## Test out your Postgres installation

Postgres is pre-installed in your `devcontainer`. To try it out run the below.

```sh
psql $DATABASE_URL
```

You should see something like the below. (\q to Quit)

```sh
psql (14.2 (Debian 14.2-1.pgdg110+1), server 14.1 (Debian 14.1-1.pgdg110+1))
Type "help" for help.

postgres=# \dt
Did not find any relations.
postgres=# \q
```