list:
    just --list

dev-init:
    k3d cluster delete k3d-nails
    k3d cluster create k3d-nails --agents 1 -p "30000-30001:30000-30001@agent:0"

dev-setup:
    cargo run --bin nails-cli -- init --no-operator
    cargo run --bin nails-cli -- install --manifest demo-nails-app.yaml --development
    cargo run --bin nails-cli -- operator

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    mkdir -p ~/.kube
    GW_IP=`ip route | awk '/default/ {print $$3}'`
    k3d kubeconfig get k3d-k3d-nails | sed "s|server: https://0\.0\.0\.0:|server: https://$${GW_IP}:|g" > ~/.kube/config
    kubectl config use-context k3d-k3d-nails >/dev/null

watch:
    mold -run cargo watch --workdir /workspace/ -w crates/web-server -w crates/web-pages -w crates/web-assets -w crates/db --no-gitignore -x "run --bin web-server"

tailwind:
    cd /workspace/crates/web-assets && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

nails:
    cargo run --bin nails-cli

install-codex:
    sudo npm install -g @openai/codex

watch-static:
    cargo watch --workdir /workspace/crates/static-website -w ./content -w ./src --no-gitignore -x "run --bin static-website"

wasm:
    cd /workspace/crates/web-csr && wasm-pack build --target web --out-dir dist

tailwind-static:
    cd /workspace/crates/static-website && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

ws:
    cd /workspace/crates/static-website && cargo watch --workdir /workspace/crates/static-website -w ./content -w ./src --no-gitignore -x "run --bin static-website"

wts:
    cd /workspace/crates/static-website && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch
