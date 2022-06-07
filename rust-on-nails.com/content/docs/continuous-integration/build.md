+++
title = "Build our Containers"
description = "Build our Containers"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

## Introduction

The ideal output from any CI/CD pipeline is one or more Docker containers. This allows us to separate deployment from build ina clean way. Once we have our containers we are free to choose how we deply them wether that is Kubernetes in the cloud or on or with any other deployment service that support containers.

## Earthly

Earthly uses Dockerfile syntax for creating builds. So we can leverage our existing Dockerfile knowledge. 

Create an Earthfile with the below contents.

```Dockerfile
FROM purtontech/rust-on-nails-devcontainer:1.0.5

ARG APP_NAME=app
ARG APP_FOLDER=app
ARG IMAGE_PREFIX=rustonnails

# This file builds the following containers
ARG APP_IMAGE_NAME=$IMAGE_PREFIX/$APP_NAME:latest
ARG MIGRATIONS_IMAGE_NAME=$IMAGE_PREFIX/$APP_NAME-migrations:latest

WORKDIR /build

USER root

# Set up for docker in docker https://github.com/earthly/earthly/issues/1225
DO github.com/earthly/lib+INSTALL_DIND

USER vscode

all:
    BUILD +build

npm-deps:
    COPY $APP_FOLDER/package.json $APP_FOLDER/package.json
    COPY $APP_FOLDER/package-lock.json $APP_FOLDER/package-lock.json
    RUN cd $APP_FOLDER && npm install
    SAVE ARTIFACT $APP_FOLDER/node_modules

npm-build:
    FROM +npm-deps
    COPY $APP_FOLDER/asset-pipeline $APP_FOLDER/asset-pipeline
    COPY --if-exists protos protos
    COPY $APP_FOLDER/templates $APP_FOLDER/templates
    COPY +npm-deps/node_modules $APP_FOLDER/node_modules
    RUN cd $APP_FOLDER && npm run release
    SAVE ARTIFACT $APP_FOLDER/dist

prepare-cache:
    COPY --dir $APP_FOLDER/src $APP_FOLDER/Cargo.toml $APP_FOLDER/build.rs $APP_FOLDER/asset-pipeline $APP_FOLDER
    COPY Cargo.lock Cargo.toml .
    RUN cargo chef prepare --recipe-path recipe.json
    SAVE ARTIFACT recipe.json

build-cache:
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release --target x86_64-unknown-linux-musl 
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home
    SAVE IMAGE --cache-hint

build:
    COPY --dir $APP_FOLDER/src $APP_FOLDER/Cargo.toml $APP_FOLDER/build.rs $APP_FOLDER/templates $APP_FOLDER/queries $APP_FOLDER/asset-pipeline $APP_FOLDER
    COPY --dir db Cargo.lock Cargo.toml protos .
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    RUN mkdir asset-pipeline
    COPY --dir +npm-build/dist $APP_FOLDER/
    COPY --dir $APP_FOLDER/asset-pipeline/images $APP_FOLDER/asset-pipeline
    # We need to run inside docker as we need postgres running for SQLX
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable
    USER root
    WITH DOCKER \
        --pull postgres:alpine
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword postgres:alpine \
            && while ! pg_isready --host=localhost --port=5432 --username=postgres; do sleep 1; done ;\
                dbmate up \
            && cargo build --release --target x86_64-unknown-linux-musl
    END
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$APP_EXE_NAME AS LOCAL ./tmp/$APP_EXE_NAME
```

## Running the Build

From the command line run

```sh
earthly -P +all
```

The build will happend and you should see the executable being created.

## Building Containers

The above build will create our rust executable, to turn that into a docker container add the following to the end of your `Earthfile`.

```Dockerfile
migration-container:
    FROM debian:bullseye-slim
    RUN apt-get update -y \  
        && apt-get install -y --no-install-recommends ca-certificates curl libpq-dev \
        && rm -rf /var/lib/apt/lists/*
    RUN curl -OL https://github.com/amacneil/dbmate/releases/download/v$DBMATE_VERSION/dbmate-linux-amd64 \
        && mv ./dbmate-linux-amd64 /usr/bin/dbmate \
        && chmod +x /usr/bin/dbmate
    COPY --dir db .
    CMD dbmate up
    SAVE IMAGE $INIT_IMAGE_NAME

app-container:
    FROM scratch
    COPY +build/$APP_EXE_NAME rust-exe
    COPY --dir +npm-build/dist dist
    COPY --dir $APP_FOLDER/asset-pipeline/images asset-pipeline/images
    ENTRYPOINT ["./rust-exe"]
    SAVE IMAGE $APP_IMAGE_NAME
```

We actually create 2 containers. One for the executable and another that will run migrations.