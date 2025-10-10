VERSION 0.7

target +nails-cli:
    FROM docker.io/library/rust:1.76-bullseye
    WORKDIR /workspace

    RUN apt-get update && \
        apt-get install -y --no-install-recommends pkg-config libssl-dev && \
        rm -rf /var/lib/apt/lists/*

    COPY Cargo.toml Cargo.lock ./
    RUN mkdir -p crates/nails-cli
    COPY crates/nails-cli/Cargo.toml crates/nails-cli/Cargo.toml

    RUN cargo fetch --locked

    COPY . .
    RUN cargo build --release --locked -p nails-cli

    RUN install -d /out
    RUN install -m 0755 target/release/nails-cli /out/nails

    SAVE ARTIFACT /out/nails AS LOCAL nails

target +nails-operator-image:
    ARG IMAGE=ghcr.io/nails/manager:dev

    FROM docker.io/library/rust:1.76-bullseye AS build
    WORKDIR /workspace

    RUN apt-get update && \
        apt-get install -y --no-install-recommends pkg-config libssl-dev && \
        rm -rf /var/lib/apt/lists/*

    COPY Cargo.toml Cargo.lock ./
    RUN mkdir -p crates/nails-cli
    COPY crates/nails-cli/Cargo.toml crates/nails-cli/Cargo.toml

    RUN cargo fetch --locked

    COPY . .
    RUN cargo build --release --locked -p nails-cli

    FROM docker.io/debian:bookworm-slim
    RUN apt-get update && \
        apt-get install -y --no-install-recommends ca-certificates && \
        rm -rf /var/lib/apt/lists/*

    COPY --from=build /workspace/target/release/nails-cli /usr/local/bin/nails
    RUN useradd --system --home /nonexistent --shell /usr/sbin/nologin nails
    USER nails
    ENTRYPOINT ["/usr/local/bin/nails"]
    CMD ["operator"]

    SAVE IMAGE --push $IMAGE
