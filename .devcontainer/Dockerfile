FROM purtontech/rust-on-nails-devcontainer:1.0.3

COPY .bash_aliases /home/vscode/.bash_aliases

ARG ZOLA_VERSION=0.15.3
RUN sudo curl -OL https://github.com/getzola/zola/releases/download/v$ZOLA_VERSION/zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo tar -xvf zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo mv zola /usr/bin/zola \
    && sudo chmod +x /usr/bin/zola