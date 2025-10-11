list:
    just --list

dev-init:
    k3d cluster delete k3d-nails
    k3d cluster create k3d-nails --agents 1 -p "30000-30001:30000-30001@agent:0"

dev-setup:
    cargo run --bin nails-cli -- init --no-operator
    cargo run --bin nails-cli -- install --manifest demo-nails-app.yaml --development
    cargo run --bin nails-cli -- operator

codex: 
    sudo npm install -g @openai/codex

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    sudo apt-get update -qq && sudo apt-get install -y -qq iproute2
    k3d kubeconfig write k3d-nails --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/$(ip route | awk '/default/ {print $3}')/g; s/0\.0\.0\.0/$(ip route | awk '/default/ {print $3}')/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"
    
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
