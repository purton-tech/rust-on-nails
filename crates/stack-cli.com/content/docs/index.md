# Stack Developer Platform

Stack is a self-hosted deployment platform that layers identity, networking, databases, and tooling on top of any Kubernetes cluster. Instead of wiring together operators, CRDs, and manifests by hand, you install Stack once and immediately get a PaaS-like workflow for your applications.

![Stack architecture](./architecture-diagram.svg)

Looking for a deeper dive? Read the [Stack architecture guide](./architecture/) to see how the operator, CRDs, and supporting services interact.

## Install Stack

1. **Grab the CLI.**

   ```bash
   export STACK_VERSION=v1.3.31
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

## Operate and debug

- `stack operator --once` runs a single reconciliation loop locally so you can watch what the controller does without staying connected forever. Drop `--once` to keep it running.
- `stack status --manifest demo-stack-app.yaml` prints the Keycloak admin credentials and inspects the `cloudflared` pods inside the namespace defined by your manifest. When a quick tunnel is running you will see the temporary HTTPS URL here.
- Need ingress from the wider internet? Follow the [Cloudflare quick-tunnel guide](./cloudflare/) to create either temporary or authenticated tunnels straight from your StackApp manifest.
- Curious about what `stack init` installs? The [Keycloak operator](./keycloak-operator/) and [PostgreSQL operator](./postgres-operator/) guides explain the shared services Stack keeps healthy for you.
- Ready to ship an existing framework? See the [Rails on Kubernetes](./framework/) and [Flask on Kubernetes](./framework/flask/) guides for concrete end-to-end examples.
- Want a local env? The [Developers workflow](./developers/) page shows how to spin up k3d, patch your kubeconfig, and run the Stack CLI manually.

## What's next?

- `stack operator` lets you run the reconciliation loop locally for rapid debugging.
- `stack status --manifest demo-stack-app.yaml` shows Cloudflare URLs, Keycloak credentials, and other platform details for that namespace.
- Use the generated CRDs (`stackapps.stack-cli.dev`) from your own automation pipelines to manage namespaces and applications at scale.
