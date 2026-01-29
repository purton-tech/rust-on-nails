# Do all the cargo install stuff
FROM rust:slim-bookworm as builder

ARG TARGETOS
ARG TARGETARCH
ARG TARGETPLATFORM

ARG CARGO_BINSTALL_VERSION=0.8.0
ARG CARGO_WATCH_VERSION=7.8.0

# Configure apt and install packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libpq-dev \
        curl \
        wget \
        xz-utils \
        build-essential \
        unzip

# Install cargo-watch
RUN case "${TARGETARCH}" in \
        amd64) ARCH_WATCH="x86_64-unknown-linux-musl" ;; \
        arm64) ARCH_WATCH="aarch64-unknown-linux-gnu" ;; \
        *) echo "Unsupported architecture for cargo-watch: ${TARGETARCH}" && exit 1 ;; \
    esac \
    && curl -L https://github.com/watchexec/cargo-watch/releases/download/v${CARGO_WATCH_VERSION}/cargo-watch-v${CARGO_WATCH_VERSION}-${ARCH_WATCH}.tar.xz -o cargo-watch.tar.xz \
    && tar -xf cargo-watch.tar.xz \
    && mv cargo-watch-v${CARGO_WATCH_VERSION}-${ARCH_WATCH}/cargo-watch /home/ \
    && chmod +x /home/cargo-watch \
    && rm -rf cargo-watch.tar.xz cargo-watch-v${CARGO_WATCH_VERSION}-${ARCH_WATCH}

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
# Determine architecture and install cargo-binstall
RUN case "${TARGETARCH}" in \
        amd64) ARCH="x86_64-unknown-linux-musl" ;; \
        arm64) ARCH="aarch64-unknown-linux-musl" ;; \
        *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac \
    && wget https://github.com/cargo-bins/cargo-binstall/releases/download/v${CARGO_BINSTALL_VERSION}/cargo-binstall-${ARCH}.tgz \
    && tar -xvf cargo-binstall-${ARCH}.tgz \
    && cp cargo-binstall /usr/local/cargo/bin/ \
    && rm cargo-binstall-${ARCH}.tgz

FROM rust:slim-bookworm

ARG TARGETOS
ARG TARGETARCH
ARG TARGETPLATFORM

ARG CLOAK_VERSION=1.19.4
ARG ZOLA_VERSION=0.17.2
ARG DBMATE_VERSION=2.28.0
ARG MOLD_VERSION=2.34.1
ARG EARTHLY_VERSION=0.8.15
ARG DOCKER_COMPOSE_VERSION=2.23.0
ARG POSTGRES_CLIENT_VERSION=16
ARG K9S_VERSION=v0.50.16
ARG STACK_VERSION=v1.1.14


# This Dockerfile adds a non-root 'vscode' user with sudo access. However, for Linux,
# this user's GID/UID must match your local user UID/GID to avoid permission issues
# with bind mounts. Update USER_UID / USER_GID if yours is not 1000. See
# https://aka.ms/vscode-remote/containers/non-root-user for details.
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Configure apt and install packages
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        ca-certificates \
        gnupg \
        gnupg-agent \
        curl && \
    echo "deb http://apt.postgresql.org/pub/repos/apt bookworm-pgdg main" > /etc/apt/sources.list.d/pgdg.list && \
    curl -sSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        apt-transport-https \
        build-essential \
        git \
        jq \
        musl \
        musl-dev \
        musl-tools \
        nodejs \
        npm \
        protobuf-compiler \
        software-properties-common \
        ssh \
        sudo \
        wget \
        postgresql-client-${POSTGRES_CLIENT_VERSION} && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/* /var/cache/* \

    # Docker Engine for Earthly. https://docs.docker.com/engine/install/debian/
    && install -m 0755 -d /etc/apt/keyrings \
    && curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg \
    && chmod a+r /etc/apt/keyrings/docker.gpg \
    && curl -fsSL "https://download.docker.com/linux/debian/gpg" | apt-key add - \
    && echo \
        "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
        "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
        tee /etc/apt/sources.list.d/docker.list > /dev/null \
    && apt-get -y update \
    && apt-get -y --no-install-recommends install docker-ce docker-ce-cli containerd.io \
    && apt-get autoremove -y && apt-get clean -y \

    # Tailwind binary + extra
    && curl -OL https://github.com/dobicinaitis/tailwind-cli-extra/releases/latest/download/tailwindcss-extra-linux-x64 \
    && chmod +x tailwindcss-extra-linux-x64 \
    && mv tailwindcss-extra-linux-x64 /usr/local/bin/tailwind-extra \
        
    # Rainfrog (latest)
    && curl -s https://api.github.com/repos/achristmascarl/rainfrog/releases/latest \
      | grep "browser_download_url.*x86_64-unknown-linux-musl.tar.gz" \
      | cut -d '"' -f 4 \
      | xargs -n1 -P1 curl -OL \
    && tar -xzf rainfrog-*-x86_64-unknown-linux-musl.tar.gz \
    && chmod +x rainfrog \
    && mv rainfrog /usr/local/bin/rainfrog \
    && rm rainfrog-*-x86_64-unknown-linux-musl.tar.gz \

    # Create a non-root user
    && groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME\
    && chmod 0440 /etc/sudoers.d/$USERNAME \
    # Rust tools
    && rustup component add rustfmt clippy \
    # Add the musl toolchain
    && rustup target add x86_64-unknown-linux-musl \
    && rustup target add aarch64-unknown-linux-musl \
    && rustup target add wasm32-unknown-unknown \
     # Database migrations   
    && curl -OL https://github.com/amacneil/dbmate/releases/download/v$DBMATE_VERSION/dbmate-linux-${TARGETARCH} \
    && mv ./dbmate-linux-${TARGETARCH} /usr/bin/dbmate \
    && chmod +x /usr/bin/dbmate \
    # Mold - Fast Rust Linker
    && curl -OL https://github.com/rui314/mold/releases/download/v$MOLD_VERSION/mold-$MOLD_VERSION-x86_64-linux.tar.gz \
    && tar -xf mold-$MOLD_VERSION-x86_64-linux.tar.gz \
    && mv ./mold-$MOLD_VERSION-x86_64-linux/bin/mold /usr/bin/ \
    && mv ./mold-$MOLD_VERSION-x86_64-linux/lib/mold/mold-wrapper.so /usr/bin/ \
    && rm mold-$MOLD_VERSION-x86_64-linux.tar.gz \
    && rm -rf ./mold-$MOLD_VERSION-x86_64-linux \
    && chmod +x /usr/bin/mold

RUN wget https://github.com/earthly/earthly/releases/download/v$EARTHLY_VERSION/earthly-linux-${TARGETARCH} -O /usr/local/bin/earthly \
    && chmod +x /usr/local/bin/earthly \
    # K9s
    && curl -L -s https://github.com/derailed/k9s/releases/download/${K9S_VERSION}/k9s_Linux_${TARGETARCH}.tar.gz | tar xvz -C /tmp \
    && mv /tmp/k9s /usr/bin \
    && rm -rf k9s_Linux_*.tar.gz \
    # k3d
    && curl -s https://raw.githubusercontent.com/k3d-io/k3d/main/install.sh | bash \
    # Kubectl
    && curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/${TARGETARCH}/kubectl" \
    && install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl \
    # Just
    && curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/bin \
    && mv ~/bin/just /usr/local/bin

# stack CLI (fixed release)
RUN curl -OL https://github.com/stack-cli/stack-cli/releases/download/${STACK_VERSION}/stack-cli-linux \
    && chmod +x ./stack-cli-linux \
    && sudo mv ./stack-cli-linux /usr/local/bin/stack
    
USER $USERNAME

# Copy the binaries we built in builder container
COPY --chown=$USERNAME --from=builder /home/cargo-watch $CARGO_HOME/bin
#COPY --chown=$USERNAME --from=builder /usr/local/cargo/bin/cargo-chef $CARGO_HOME/bin
COPY --chown=$USERNAME --from=builder /usr/local/cargo/bin/cargo-binstall $CARGO_HOME/bin


# Pulumi
RUN curl -fsSL https://get.pulumi.com | sudo -E bash - \
    && sudo chown -R $USERNAME:$USERNAME /home/$USERNAME/.pulumi

# Compile here as it requires musl.
RUN cargo binstall --no-confirm cornucopia

RUN if [ "$TARGETARCH" != "arm64" ]; then \
        cargo binstall --no-confirm clorinde; \
        cargo binstall --no-confirm wasm-bindgen-cli; \
    fi
