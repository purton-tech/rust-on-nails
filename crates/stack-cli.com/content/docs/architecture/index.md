# Stack Architecture

Stack is delivered as a Kubernetes operator plus a set of curated operators that your clusters rarely ship out of the box. Understanding how those pieces fit together helps you diagnose issues or extend the platform.

## Controller-first workflow

`stack install --manifest <file>` creates or updates a `StackApp` custom resource. The Stack operator watches these resources and reconciles them into:

- A dedicated CloudNativePG database cluster with app, readonly, and migration credentials.
- Secrets (`database-urls`, `db-owner`) wired into your container’s environment.
- An nginx deployment configured either for static JWT auth or Keycloak/OAuth2 Proxy, depending on the manifest.
- Optional helpers like MailHog, pgAdmin, and NetworkPolicy objects that keep the namespace isolated.

You can run the controller indefinitely inside the cluster, or locally with `stack operator --once` while iterating on changes.

## What `stack init` installs

Running `stack init` bootstraps all shared dependencies the operator expects:

1. **CloudNativePG** for managed PostgreSQL clusters.
2. **Keycloak CRDs plus the Keycloak operator** for identity and OAuth flows.
3. **Ingress (NGINX)** unless you pass `--disable-ingress`.
4. The `stackapps.stack-cli.dev` CRD, controller deployment, and associated RBAC.

`stack init --no-operator` is handy when you want to run the controller locally but still install supporting CRDs and cluster roles.

## Namespace lifecycle

Each manifest defines exactly one namespace. The install command ensures that namespace exists before applying the resource. Deleting a `StackApp` removes the database, secrets, and deployments but leaves the namespace so you can recover logs or additional Kubernetes objects if needed.

## Observability and debugging

- `stack operator --once` processes a single reconciliation tick for rapid feedback.
- `stack status --manifest ...` inspects Cloudflare tunnels and prints Keycloak credentials for the namespace in your manifest.
- Because Stack relies on standard Kubernetes objects, you can always drop to `kubectl` to inspect pods, services, and secrets that the operator created.
