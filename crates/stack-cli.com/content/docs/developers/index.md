# Developers: Local Workflow

Use [k3d](https://k3d.io) to spin up a lightweight Kubernetes cluster, then point the Stack CLI at it. The commands below assume you are running inside the repo’s dev container, but they work anywhere k3d is installed.

## 1. Create (or reset) the cluster

```bash
k3d cluster delete stack-demo || true
k3d cluster create stack-demo --agents 1 -p "30000-30001:30000-30001@agent:0"
```

This recreates a clean cluster with NodePorts mapped so Stack’s helper services are reachable from your machine.

## 2. Prepare kubeconfig access

```bash
sudo apt-get update -qq && sudo apt-get install -y -qq iproute2
k3d kubeconfig write stack-demo --kubeconfig-merge-default
DEFAULT_GW=$(ip route | awk '/default/ {print $3}')
sed -i "s/127\.0\.0\.1/${DEFAULT_GW}/g; s/0\.0\.0\.0/${DEFAULT_GW}/g" "$HOME/.kube/config"
sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
```

Those steps copy the cluster credentials into your default kubeconfig, rewrite the server address so tools inside the dev container can reach it, and relax TLS checks for local use.

## 3. Bootstrap Stack into the cluster

```bash
stack init --no-operator
stack install --manifest demo-stack-app.yaml
stack operator --once
```

- `stack init --no-operator` installs CloudNativePG, Keycloak, nginx, and the StackApp CRD without leaving a controller running in the cluster.
- `stack install ...` applies the demo workload so you have something to inspect.
- `stack operator --once` reconciles locally for a single tick; remove `--once` to keep it running.

## 4. Iterate

- `stack status --manifest demo-stack-app.yaml` prints credentials and Cloudflare URLs.
- `stack cloudflare --manifest demo-stack-app.yaml --name local` starts a tunnel (add `--token` for authenticated tunnels).
- `stack operator --once` becomes your go-to after editing manifests so you can watch databases, secrets, and Deployments update in real time.

## Tips

- `k3d cluster delete stack-demo` is handy when switching branches that alter CRDs.
- Keep `kubectl get pods --all-namespaces --watch` open in another terminal to see reconciliations as they happen.
- Port-forward services you care about (for example, `kubectl port-forward svc/stack-db-cluster-rw 5455:5432 -n stack-demo`) to connect local tooling to the namespace-specific resources Stack provisions.
