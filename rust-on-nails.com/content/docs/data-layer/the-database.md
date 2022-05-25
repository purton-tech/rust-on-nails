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

> If you're not sure which database to use then, use Postgres.

The architecture doesn't stop you using MySQL (MariaDB?) or other relational databases. However a relational database is the recommendation.

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