# Flask on Kubernetes

Python apps deploy just as easily as Rails ones. Here is how to package a Flask service and run it with Stack.

## Dockerfile

```dockerfile
FROM python:3.12-slim
WORKDIR /app

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    PORT=5000

RUN apt-get update && apt-get install -y build-essential && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .
CMD ["gunicorn", "--bind", "0.0.0.0:5000", "app:app"]
```

`requirements.txt`:

```
flask==3.0.3
gunicorn==23.0.0
```

## StackApp manifest

```yaml
# flask-stack-app.yaml
apiVersion: stack-cli.dev/v1
kind: StackApp
metadata:
  name: flask-app
  namespace: flask-demo
spec:
  web:
    image: ghcr.io/acme/flask-app:latest
    port: 5000
  auth:
    jwt: "local-dev-token"
```

Apply it with:

```bash
stack install --manifest flask-stack-app.yaml
stack cloudflare --manifest flask-stack-app.yaml --name flask-demo
```

Once you have a stable Cloudflare hostname, set `auth.hostname-url` so Keycloak/OAuth2 Proxy can enforce proper redirects.
