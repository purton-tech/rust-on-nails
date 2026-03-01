# The Database

## Test out your Postgres installation

First we'll need to create a `.env` file with our databse secrets.

```sh
stack secrets --manifest infra-as-code/stack.yaml --db-host host.docker.internal --db-port 30001 >> .env
```

Postgres is pre-installed in your `devcontainer`. To try it out run the below.

```sh
db
```

You should see something like the below. (\q to Quit)

```sh
psql (14.2 (Debian 14.2-1.pgdg110+1), server 14.1 (Debian 14.1-1.pgdg110+1))
Type "help" for help.

postgres=# \dt
Did not find any relations.
postgres=# \q
```