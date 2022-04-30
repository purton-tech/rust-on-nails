# Do all the cargo install stuff
FROM rust:slim as builder

# Configure apt and install packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        xz-utils \
        unzip

RUN curl https://github.com/sagiegurari/cargo-make/releases/download/0.34.0/cargo-make-v0.34.0-x86_64-unknown-linux-musl.zip -L -o cargo-make.zip \
    && unzip cargo-make.zip \
    && mv cargo-make-v0.34.0-x86_64-unknown-linux-musl/cargo-make /home

RUN curl https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-musl.tar.xz -L -o cargo-watch.tar.xz \
    && tar -xf cargo-watch.tar.xz \
    && mv cargo-watch-v7.8.0-x86_64-unknown-linux-musl/cargo-watch /home

RUN cargo install cornucopia
RUN cargo install cargo-chef --locked


FROM rust:slim
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
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        wget \
        git \
        ssh \
        sudo \
        # required by parcel or you can't npm install 
        build-essential \
        # For musl builds
        musl-dev \
        musl-tools \
        musl \
        # Docker in Docker for Earthly
        apt-transport-https \
        ca-certificates \
        gnupg-agent \
        gnupg \
        software-properties-common \
    #
    #
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -r /var/cache/* /var/lib/apt/lists/*
    

# Install psql 14
RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list' \
   && wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - \
   && apt-get -y update \
   && apt-get -y install postgresql-client \
   && apt-get autoremove -y && apt-get clean -y

# Install node.
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
RUN sudo apt-get install -y nodejs

# Create a non-root user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME\
    && chmod 0440 /etc/sudoers.d/$USERNAME 
    
USER $USERNAME

COPY --chown=$USERNAME --from=builder /home/cargo-make $CARGO_HOME/bin
COPY --chown=$USERNAME --from=builder /home/cargo-watch $CARGO_HOME/bin
COPY --chown=$USERNAME --from=builder /usr/local/cargo/bin/cargo-chef $CARGO_HOME/bin
COPY --chown=$USERNAME --from=builder /usr/local/cargo/bin/cornucopia $CARGO_HOME/bin

# Essential tools
RUN rustup component add rustfmt clippy

# For musl builds
RUN rustup target add x86_64-unknown-linux-musl


# Install secrets management
ARG CLOAK_VERSION=1.10.0
RUN sudo /bin/sh -c "wget https://github.com/purton-tech/cloak/releases/download/v$CLOAK_VERSION/cloak-linux -O /usr/local/bin/cloak && chmod +x /usr/local/bin/cloak"

# Database migrations   
ARG DBMATE_VERSION=1.15.0
RUN sudo curl -OL https://github.com/amacneil/dbmate/releases/download/v$DBMATE_VERSION/dbmate-linux-amd64 \
    && sudo mv ./dbmate-linux-amd64 /usr/bin/dbmate \
    && sudo chmod +x /usr/bin/dbmate

# mold
ARG MOLD_VERSION=1.1.1
RUN sudo curl -OL https://github.com/rui314/mold/releases/download/v$MOLD_VERSION/mold-$MOLD_VERSION-x86_64-linux.tar.gz \
    && sudo tar -xf mold-$MOLD_VERSION-x86_64-linux.tar.gz \
    && sudo mv ./mold-$MOLD_VERSION-x86_64-linux/bin/mold /usr/bin/ \
    && sudo mv ./mold-$MOLD_VERSION-x86_64-linux/lib/mold/mold-wrapper.so /usr/bin/ \
    && sudo chmod +x /usr/bin/mold

# Docker Engine for Earthly. 
RUN  curl -fsSL "https://download.docker.com/linux/debian/gpg" | sudo apt-key add -
RUN sudo add-apt-repository \
        "deb [arch=$(dpkg --print-architecture)] https://download.docker.com/linux/debian \
        $(lsb_release -cs) \
        stable" \
    && sudo apt-get update \
    && sudo apt-get install -y docker-ce docker-ce-cli containerd.io \
    #
    #
    # Clean up
    && sudo apt-get autoremove -y \
    && sudo apt-get clean -y \
    && sudo rm -r /var/cache/* /var/lib/apt/lists/*

# Earthly
RUN sudo /bin/sh -c 'wget https://github.com/earthly/earthly/releases/latest/download/earthly-linux-amd64 -O /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly'
