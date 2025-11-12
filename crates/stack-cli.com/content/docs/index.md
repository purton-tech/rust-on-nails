# Stack Developer Platform

Stack is a developer platform that layers identity, networking, databases, and tooling on top of any Kubernetes cluster. Instead of wiring together operators, CRDs, and manifests by hand, you install Stack once and immediately get a coherent place to deploy applications.

![Stack architecture](./architecture-diagram.svg)

## Install Stack

1. **Grab the CLI.**

   ```bash
   export STACK_VERSION=v1.3.25
   curl -OL https://github.com/purton-tech/rust-on-nails/releases/download/${STACK_VERSION}/stack-cli \
     && chmod +x ./stack-cli \
     && sudo mv ./stack-cli /usr/local/bin/stack
   ```

2. **Bootstrap the platform operators into your cluster.**

   ```bash
   stack init
   ```

   This command installs CloudNativePG, Keycloak, ingress, the Stack controller, and custom resource definitions that describe your applications.

3. **Apply a StackApp manifest.**

   ```bash
   stack install --manifest demo-stack-app.yaml
   ```

   A minimal `StackApp` looks like this:

   ```yaml
   apiVersion: stack-cli.dev/v1
   kind: StackApp
   metadata:
     name: stack-app
     namespace: stack-demo
   spec:
     web:
       image: ghcr.io/stack/demo-app:latest
       port: 7903
     auth:
       jwt: "1"
   ```

   The controller provisions a dedicated CloudNativePG cluster, injects connection strings into secrets, deploys your container as `stack-app`, and keeps everything in sync with the manifest.

## Chapter: Expose traffic with Cloudflare

Stack can open your cluster to the internet through Cloudflare tunnels directly from your manifest:

- **Quick tunnel (no Cloudflare account).** Omit `--token` and Stack spins up a temporary tunnel that prints an accessible URL once `stack status --manifest demo-stack-app.yaml` runs.
- **Authenticated tunnel.** Generate a Cloudflare tunnel token and run:

  ```bash
  stack cloudflare \
    --manifest demo-stack-app.yaml \
    --token "$CLOUDFLARE_TUNNEL_TOKEN" \
    --name stack
  ```

Every tunnel points at the nginx instance Stack already deployed, so your app, Keycloak, and OAuth2 Proxy immediately become reachable. Because the command reads the `StackApp` manifest you pass in, the tunnel installs directly into the same namespace. Update your `StackApp` manifest with `auth.hostname-url` to enable Keycloak redirects over the new hostname.

## What's next?

- `stack operator` lets you run the reconciliation loop locally for rapid debugging.
- `stack status --manifest demo-stack-app.yaml` shows Cloudflare URLs, Keycloak credentials, and other platform details for that namespace.
- Use the generated CRDs (`stackapps.stack-cli.dev`) from your own automation pipelines to manage namespaces and applications at scale.
