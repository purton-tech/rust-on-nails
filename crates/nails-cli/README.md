## The Nails CLI and Kubernetes Operator

Run and see the CLI help

```sh
cargo run --bin nails-cli -- -h
```

## Run as an Operator

```sh
cargo run --bin nails-cli -- operator
```

## (Re-)install K3's

```sh
# Uninstall
sudo /usr/local/bin/k3s-uninstall.sh
```

```sh
curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -
```

## Install the application into a cluster

The `.kube/config` is already mapped in by `devcontainer.json`
c

If that one doesn't work copy `~/.kube/config` to `tmp/kubeconfig` then

```
export KUBECONFIG=/workspace/tmp/kubeconfig 
```

Then run

```sh
cargo run --bin nails-cli -- init
cargo run --bin nails-cli -- install --manifest ../../demo-nails-app.yaml
```

## Testing the Operator

Install the manifests without the in-cluster controller so you can iterate on the binary locally and observe the resources it creates:

```sh
cargo run --bin nails-cli -- init --no-operator
cargo run --bin nails-cli -- install --manifest ../../demo-nails-app.yaml --development
```

Then run the operator locally and confirm it reconciles `NailsApplication` objects:

```sh
cargo run --bin nails-cli -- operator
kubectl get nailsapplications --all-namespaces --watch
```

Try it
