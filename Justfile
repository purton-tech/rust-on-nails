list:
    just --list

dev-init:
    k3d cluster delete
    k3d cluster create --agents 1 -p "30000-30001:30000-30001@agent:0"

dev-setup:
    cargo run --bin k8s-operator -- install --no-operator --testing --development --hostname-url http://localhost:30000
    cargo run --bin k8s-operator -- operator

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    k3d kubeconfig write k3s-default --kubeconfig-merge-default
    
watch:
    mold -run cargo watch --workdir /workspace/ -w crates/web-server -w crates/web-pages -w crates/web-assets -w crates/db --no-gitignore -x "run --bin web-server"

tailwind:
    cd /workspace/crates/web-assets && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

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