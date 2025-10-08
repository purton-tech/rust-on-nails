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
