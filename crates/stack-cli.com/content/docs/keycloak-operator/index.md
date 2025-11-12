# Keycloak Operator

Stack relies on Keycloak for OAuth2 and OpenID Connect flows. When you run `stack init`, the CLI installs everything required to run a shared Keycloak control plane inside your cluster.

## What gets installed

1. **CustomResourceDefinitions** – `keycloaks.k8s.keycloak.org` and `keycloakrealmimports.k8s.keycloak.org` enable the operator to watch realms and servers.
2. **Keycloak Operator** – A deployment that reconciles `Keycloak` and `KeycloakRealmImport` resources.
3. **Dedicated namespace** – Stack creates (or reuses) the `keycloak` namespace so the identity stack stays isolated.
4. **Backing database** – The Keycloak operator provisions a CloudNativePG cluster for Keycloak itself; Stack wires credentials automatically.

## How Stack uses Keycloak

- Each `StackApp` with `spec.auth.hostname-url` defined triggers the Stack controller to ensure a Keycloak realm and OAuth2 Proxy configuration exist.
- The CLI creates an initial admin secret named `keycloak-initial-admin` in the Keycloak namespace. `stack status --manifest …` reads this secret so you can log in instantly.
- OAuth2 Proxy is configured to trust Keycloak and inject the right upstream headers toward your app.

## Verifying the installation

```bash
kubectl get pods -n keycloak
kubectl get keycloaks.k8s.keycloak.org -n keycloak
kubectl get secret keycloak-initial-admin -n keycloak -o yaml
```

If you ever need to reinstall Keycloak components (for example after manually deleting the namespace), re-run `stack init`. The CLI reapplies the CRDs, operator deployment, and database manifests idempotently.

## Customising Keycloak

- Change the namespace with `stack init --operator-namespace <ns>` and update your manifests accordingly.
- Create additional realms by applying `KeycloakRealmImport` resources. Stack uses the same API to configure per-application realms automatically.
- Externalise Keycloak by pointing the operator at an existing PostgreSQL database; Stack’s defaults are tailored for quick starts but the CRDs are flexible.
