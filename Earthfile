VERSION 0.8

FROM purtontech/rust-on-nails-devcontainer:1.3.18

WORKDIR /workspace

USER vscode

stack-cli:


    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .

    RUN cargo build --release -p stack-cli

    SAVE ARTIFACT target/release/stack-cli AS LOCAL ./stack-cli

stack-operator-image:
    ARG IMAGE=purtontech/stack-operator:dev

    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .

    RUN cargo build --release -p stack-cli

    FROM docker.io/debian:bookworm-slim
    RUN apt-get update && \
        apt-get install -y --no-install-recommends ca-certificates && \
        rm -rf /var/lib/apt/lists/*

    COPY --chown=1001:1001 +stack-cli/stack-cli /usr/local/bin/stack

    RUN useradd --system --home /nonexistent --shell /usr/sbin/nologin stack
    USER stack
    ENTRYPOINT ["/usr/local/bin/stack"]
    CMD ["operator"]

    SAVE IMAGE --push $IMAGE
