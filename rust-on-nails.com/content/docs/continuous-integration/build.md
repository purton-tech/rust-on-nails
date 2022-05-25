+++
title = "Build"
description = "Build"
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

## Earthly

```
FROM purtontech/rust-on-nails-devcontainer:1.0.5

# Proto compiler and web grpc.
RUN sudo apt update \
    && sudo apt install -y protobuf-compiler \
    && sudo curl -OL https://github.com/grpc/grpc-web/releases/download/1.3.0/protoc-gen-grpc-web-1.3.0-linux-x86_64  \
    && sudo mv protoc-gen-grpc-web* /usr/local/bin/protoc-gen-grpc-web \
    && sudo chmod +x /usr/local/bin/protoc-gen-grpc-web

ARG APP_EXE_NAME=app
ARG APP_FOLDER=app
ARG CLI_FOLDER=cli
ARG CLI_EXE_NAME=cli
ARG CLI_LINUX_EXE_NAME=cloak-linux
ARG CLI_MACOS_EXE_NAME=cloak-macos
ARG DBMATE_VERSION=1.15.0

# Base images
ARG ENVOY_PROXY=envoyproxy/envoy:v1.17-latest
ARG NGINX=nginx:1.21.5

# This file builds the following containers
ARG APP_IMAGE_NAME=purtontech/cloak-server:latest
ARG INIT_IMAGE_NAME=purtontech/cloak-db-migrations:latest
ARG ENVOY_IMAGE_NAME=purtontech/cloak-envoy:latest
ARG WWW_IMAGE_NAME=purtontech/cloak-website:latest


WORKDIR /build

USER root

# Set up for docker in docker https://github.com/earthly/earthly/issues/1225
DO github.com/earthly/lib+INSTALL_DIND

USER vscode

pull-request:
    BUILD +init-container
    BUILD +app-container
    BUILD +envoy-container
    BUILD +www-container
    BUILD +integration-test

all:
    BUILD +init-container
    BUILD +app-container
    BUILD +envoy-container
    BUILD +www-container
    BUILD +build-cli-osx

npm-deps:
    COPY $APP_FOLDER/package.json $APP_FOLDER/package.json
    COPY $APP_FOLDER/package-lock.json $APP_FOLDER/package-lock.json
    RUN cd $APP_FOLDER && npm install
    SAVE ARTIFACT $APP_FOLDER/node_modules

npm-build:
    FROM +npm-deps
    COPY $APP_FOLDER/asset-pipeline $APP_FOLDER/asset-pipeline
    COPY $APP_FOLDER/templates $APP_FOLDER/templates
    # Protos needed for typescript web gRPC.
    COPY protos protos
    COPY +npm-deps/node_modules $APP_FOLDER/node_modules
    RUN cd $APP_FOLDER && npm run release
    SAVE ARTIFACT $APP_FOLDER/dist

prepare-cache:
    COPY --dir $APP_FOLDER/src $APP_FOLDER/Cargo.toml $APP_FOLDER/build.rs $APP_FOLDER/asset-pipeline $APP_FOLDER
    COPY --dir $CLI_FOLDER/src $CLI_FOLDER/Cargo.toml $CLI_FOLDER
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
    COPY --dir $CLI_FOLDER/src $CLI_FOLDER/Cargo.toml $CLI_FOLDER/build.rs $CLI_FOLDER
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
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$CLI_EXE_NAME AS LOCAL ./tmp/$CLI_LINUX_EXE_NAME

init-container:
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

envoy-container:
    FROM $ENVOY_PROXY
    COPY .devcontainer/envoy.yaml /etc/envoy/envoy.yaml
    # Update the first entry in our config to point at the marketing pages
    RUN sed -i '0,/development/{s/development/www/}' /etc/envoy/envoy.yaml
    RUN sed -i '0,/7104/{s/7104/80/}' /etc/envoy/envoy.yaml
    # The second development entry in our cluster list is the app
    RUN sed -i '0,/development/{s/development/app/}' /etc/envoy/envoy.yaml
    SAVE IMAGE $ENVOY_IMAGE_NAME

zola-generate:
    ARG ZOLA_VERSION=0.15.3
    RUN sudo curl -OL https://github.com/getzola/zola/releases/download/v$ZOLA_VERSION/zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
        && sudo tar -xvf zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
        && sudo mv zola /usr/bin/zola \
        && sudo chmod +x /usr/bin/zola
    COPY --dir www www
    RUN cd www && zola build
    SAVE ARTIFACT www/public public

# Test this with docker run --rm -p7180:80 ianpurton/vault:www
www-container:
    FROM $NGINX
    COPY +zola-generate/public /usr/share/nginx/html/
    SAVE IMAGE $WWW_IMAGE_NAME

integration-test:
    FROM +build
    COPY --dir $APP_FOLDER/tests $APP_FOLDER/
    COPY --dir db .
    COPY .devcontainer/docker-compose.yml ./ 
    COPY .devcontainer/docker-compose.earthly.yml ./ 
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/cloak?sslmode=disable
    ARG APP_DATABASE_URL=postgresql://cloak:testpassword@db:5432/cloak
    # We expose selenium to localhost
    ARG WEB_DRIVER_URL='http://localhost:4444' 
    # The selenium container will connect to the envoy container
    ARG WEB_DRIVER_DESTINATION_HOST='http://envoy:7100' 
    # How do we connect to mailhog
    ARG MAILHOG_URL=http://localhost:8025/api/v2/messages?limit=1
    USER root
    WITH DOCKER \
        --compose docker-compose.yml \
        --compose docker-compose.earthly.yml \
        --service db \
        --service auth \
        --service smtp \
        # Record our selenium session
        --service selenium \
        --pull selenium/video:ffmpeg-4.3.1-20220208 \
        # Bring up the containers we have built
        --load $APP_IMAGE_NAME=+app-container \
        --load $WWW_IMAGE_NAME=+www-container \
        --load $ENVOY_IMAGE_NAME=+envoy-container

        # Force to command to always be succesful so the artifact is saved. 
        # https://github.com/earthly/earthly/issues/988
        RUN dbmate up \
            && docker run -d -p 7103:7103 --rm --network=build_default \
                -e APP_DATABASE_URL=$APP_DATABASE_URL \
                -e INVITE_DOMAIN=http://envoy:7100 \
                -e INVITE_FROM_EMAIL_ADDRESS=support@cloak.com \
                -e SMTP_HOST=smtp \
                -e SMTP_PORT=1025 \
                -e SMTP_USERNAME=thisisnotused \
                -e SMTP_PASSWORD=thisisnotused \
                -e SMTP_TLS_OFF='true' \
                --name app $APP_IMAGE_NAME \
            && docker run -d --rm --network=build_default --name www $WWW_IMAGE_NAME \
            && docker run -d -p 7100:7100 -p 7101:7101 --rm --network=build_default --name envoy $ENVOY_IMAGE_NAME \
            && cargo test --no-run --release --target x86_64-unknown-linux-musl \
            && docker run -d --name video --network=build_default -e DISPLAY_CONTAINER_NAME=build_selenium_1 -e FILE_NAME=chrome-video.mp4 -v /build/tmp:/videos selenium/video:ffmpeg-4.3.1-20220208 \
            && (cargo test --release --target x86_64-unknown-linux-musl -- --nocapture || echo fail > ./tmp/fail) \
            && docker stop app www envoy video
    END
    SAVE ARTIFACT tmp AS LOCAL ./tmp/earthly

    # If we failed in selenium a fail file will have been created
    # Comment this out to get the build to pass and see the chrome video
    IF [ -f ./tmp/fail ]
        RUN echo "cargo test has failed." && exit 1
    END

build-cli-osx:
    FROM joseluisq/rust-linux-darwin-builder:1.59.0
    COPY --dir $APP_FOLDER/src $APP_FOLDER/Cargo.toml $APP_FOLDER/build.rs $APP_FOLDER/asset-pipeline $APP_FOLDER
    COPY --dir $CLI_FOLDER/src $CLI_FOLDER/Cargo.toml $CLI_FOLDER/build.rs $CLI_FOLDER
    COPY --dir db Cargo.lock Cargo.toml protos .
    RUN cd cli \ 
        && CC=o64-clang \
        CXX=o64-clang++ \
        cargo build --release --target x86_64-apple-darwin
    SAVE ARTIFACT target/x86_64-apple-darwin/release/$CLI_EXE_NAME AS LOCAL ./tmp/$CLI_MACOS_EXE_NAME
```