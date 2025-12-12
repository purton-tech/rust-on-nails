# Local Kubernetes

Run Kubernetes inside the devcontainer with [k3d](https://k3d.io/) so your local environment matches how we deploy to real clusters.

## Bootstrap the cluster

Open a terminal inside the devcontainer and recreate the local cluster with the bundled recipe:

```sh
just dev-init
```

This deletes any previous `k3d-nails` cluster, creates a fresh one with a single agent node, maps the app/Postgres NodePorts to the host (`30000-30001`), and patches your kubeconfig via `just get-config` so `kubectl` works from inside the container.

## Install the Nails stack

With the cluster up, install the platform operators and sample StackApp:

```sh
just dev-setup
```

The Justfile in `nails-devcontainer/Justfile` runs `stack init` followed by `stack install --manifest stack.dev.yaml`, giving you a ready-to-use namespace plus database credentials.

## Inspect the cluster

Use `k9s` (preinstalled in the devcontainer) to browse pods, services, and logs after the install completes:

```sh
k9s
```

You can also spot-check with `kubectl get pods -A`. Re-run `just dev-init` any time you want to reset the cluster to a clean state.
