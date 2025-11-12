# Cloudflare Tunnels

Stack includes an opinionated Cloudflare deployment so you can expose namespaces without writing additional manifests. Both commands only need the same `StackApp` manifest you already apply for your workload, which keeps namespace selection consistent everywhere.

## Quick tunnels (no Cloudflare account)

```bash
stack cloudflare \
  --manifest demo-stack-app.yaml \
  --name stack
```

- Omitting `--token` tells Stack to start a temporary tunnel.  
- The CLI reads `metadata.namespace` from your manifest, installs `cloudflared` into that namespace, and points it at the nginx service Stack created earlier.  
- Run `stack status --manifest demo-stack-app.yaml` to print the generated HTTPS URL.

Temporary tunnels are great for demos, development sessions, and any workflow where you just need to share access for a few minutes.

## Authenticated tunnels (bring your Cloudflare account)

When you want a long-lived hostname, create a Cloudflare tunnel token and run:

```bash
stack cloudflare \
  --manifest demo-stack-app.yaml \
  --token "$CLOUDFLARE_TUNNEL_TOKEN" \
  --name stack
```

The CLI injects the token into the bundled deployment and reuses the same nginx target as the quick tunnel. Because everything comes from your manifest:

- The namespace always matches your application.
- Switching environments (dev/staging/prod) is as simple as pointing to a different manifest.

## Verifying the tunnel

Use the status command any time you need credentials or the public URL:

```bash
stack status --manifest demo-stack-app.yaml
```

You will see:

- Keycloak admin username/password read from the `keycloak-initial-admin` secret in the shared Keycloak namespace.
- The latest Cloudflare URL scraped from the `cloudflared` pod logs in your manifest namespace.
- Helpful hints if the tunnel pod is not running.

Update your `StackApp` manifest with `spec.auth.hostname-url` once you have a stable domain so Keycloak and OAuth2 Proxy can enforce proper redirects.
