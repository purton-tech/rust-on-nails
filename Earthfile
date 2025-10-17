VERSION 0.8

FROM purtontech/rust-on-nails-devcontainer:1.3.18

WORKDIR /workspace

USER vscode

nails-cli:


    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .

    RUN cargo build --release -p nails-cli

    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/nails-cli

nails-operator-image:
    ARG IMAGE=purtontech/nails-operator:dev

    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .

    RUN cargo build --release -p nails-cli

    FROM docker.io/debian:bookworm-slim
    RUN apt-get update && \
        apt-get install -y --no-install-recommends ca-certificates && \
        rm -rf /var/lib/apt/lists/*

    COPY --chown=1001:1001 +build/nails-cli /usr/local/bin/nails

    RUN useradd --system --home /nonexistent --shell /usr/sbin/nologin nails
    USER nails
    ENTRYPOINT ["/usr/local/bin/nails"]
    CMD ["operator"]

    SAVE IMAGE --push $IMAGE
