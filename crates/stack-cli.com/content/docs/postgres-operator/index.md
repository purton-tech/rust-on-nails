# PostgreSQL Operator

Every Stack namespace receives its own managed PostgreSQL cluster powered by [CloudNativePG](https://cloudnative-pg.io/). The operator is installed automatically when you run `stack init`, and the Stack controller drives it for each `StackApp`.

## Bootstrapping CloudNativePG

`stack init` applies:

1. The CloudNativePG CRDs (`clusters.postgresql.cnpg.io`, etc.).
2. The CloudNativePG operator deployment in the `stack-system` namespace (or whatever you pass with `--operator-namespace`).
3. Supporting RBAC so the Stack controller can create `Cluster` resources.

You can verify the installation with:

```bash
kubectl get pods -n stack-system -l app.kubernetes.io/name=cloudnative-pg
kubectl get crd | grep postgresql.cnpg.io
```

## What happens per StackApp

When you run `stack install --manifest app.yaml`, the controller:

1. Creates a `Cluster` (e.g. `stack-db-cluster`) inside your application namespace.
2. Generates unique credentials for application, readonly, migrations, and owner roles.
3. Stores connection strings in `database-urls` and `db-owner` secrets.
4. Wires those secrets into your Deployment’s environment variables so the app, migrations, and helper services can connect securely.

The cluster includes streaming replicas and can enable extensions like `pgvector` so you’re ready for advanced search or embedding workloads when you need them.

## Development shortcuts

- During local testing you can expose the database via NodePort by applying the `postgres-service-dev.yaml` manifest included in the repo.
- If you need deterministic credentials, the controller accepts overrides via the manifest’s `spec.database.insecurePasswords` field (intended only for tests).

## Maintenance tips

- Use `kubectl cnpg status <cluster>` (from the CloudNativePG plugin) to view backups, replicas, and failover readiness.
- `stack operator --once` is useful after editing a manifest so you can confirm the database reconciliation succeeds.
- For disaster recovery, pair CloudNativePG’s backup CRDs with your preferred object storage bucket; Stack’s defaults focus on day-one experience but the operator supports full PITR configurations.
