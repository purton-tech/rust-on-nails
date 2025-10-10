# Deploying to Kubernetes

## Why Kubernetes

- Learn one deployment model and reuse it across clouds and on-prem. Kubernetes keeps us **cloud agnostic**.
- Production workloads of every shape already run on Kubernetes, so we inherit mature primitives for scaling, rollout, and recovery.
- Matching dev and prod is easier when both run on Kubernetes. We use [k3d](https://k3d.io/) locally so we can reuse the same manifests we ship to real clusters.
- Operators (like our Nails developer portal) let us codify platform decisions once and apply them consistently for every application.

## Prerequisites

You need Docker (or another container runtime), `kubectl`, and `k3d`.

- **macOS**: `brew install k3d kubectl`
- **Linux**: `curl -s https://raw.githubusercontent.com/k3d-io/k3d/main/install.sh | bash` (or use your distro’s package manager) and install `kubectl` from [kubernetes.io](https://kubernetes.io/docs/tasks/tools/).
- **Windows**: `choco install k3d kubernetes-cli`

After installation, validate your tooling:

```sh
k3d version
kubectl version --client
```

## Create the local cluster with k3d

We wrap the recommended k3d invocation in `just dev-init`. It deletes any previous cluster and recreates one with ports mapped for the Nails app and Postgres.

```sh
just dev-init
```

Under the hood this runs:

```sh
k3d cluster delete k3s-nails
k3d cluster create k3s-nails --agents 1 -p "30000-30001:30000-30001@agent:0"
```

To inspect the kubeconfig without leaving the devcontainer:

```sh
k3d kubeconfig get k3s-nails > ~/.kube/config
kubectl get nodes
```

If you are in the Nails devcontainer, `just get-config` patches the API server address so `kubectl` can reach the cluster.

## The Nails Developer Portal

Our internal developer portal lives in the `nails-cli` crate. It installs the platform operators, creates namespaces from NailsApp manifests, and provisions databases plus credentials. You can run everything manually or use the bundled Just recipes.

### Install the platform operators

```sh
cargo run --bin nails-cli -- init
```

This command:

- Installs the CloudNativePG, Keycloak, and ingress operators.
- Ensures the application namespace (`--namespace`, default `nails`) and the operator namespace (`--operator-namespace`, default `nails-system`) exist.
- Registers the `NailsApp` CustomResourceDefinition.
- Deploys the Nails operator itself unless you pass `--no-operator`.

In development we typically let `just dev-setup` run the right flags for us:

```sh
just dev-setup
```

That sequence applies a sample NailsApp manifest, maps NodePorts for the app and Postgres, and starts the operator loop.

### Apply an application manifest

Each NailsApp describes one namespace. When you run:

```sh
cargo run --bin nails-cli -- install --manifest demo-nails-app.yaml
```

the CLI:

1. Reads the `metadata.namespace` field from the manifest (for example `nails-demo`).
2. Creates that namespace if it does not exist.
3. Applies the manifest plus supporting Kubernetes objects.

A minimal manifest needs the version, replica count, and disk sizing for the two CloudNativePG clusters. The hash fields are optional and only required when you want to pin images.

```yaml
apiVersion: nails-cli.dev/v1
kind: NailsApp
metadata:
  name: nails-app
  namespace: nails-demo
spec:
  replicas: 1
  version: 1.11.33
  primary_db_disk_size: 20
  keycloak_db_disk_size: 10
  hostname-url: https://localhost
```

### What the operator does for you

When the operator sees a NailsApp it:

- Provisions a CloudNativePG cluster dedicated to the namespace.
- Creates the `database-urls` and `db-owner` secrets with connection strings and credentials.
- Boots supporting services (Keycloak, Envoy, ingress, optional PgAdmin/observability) according to the manifest flags.

If you enable development mode, the operator exposes the application on `http://localhost:30000` and Postgres on `localhost:30001`—matching the `k3d` port mapping from `dev-init`.

You can re-run the operator any time:

```sh
cargo run --bin nails-cli -- operator
```

It will reconcile existing NailsApp resources, upgrade components when you change the version, and clean up databases and secrets on deletion.
