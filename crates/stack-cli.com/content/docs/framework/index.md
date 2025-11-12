# Rails on Kubernetes

Stack does not force you to rewrite applications in a specific language. As long as you can produce a container image that exposes an HTTP port, the operator can deploy it. Here is a concrete example using Ruby on Rails.

## Package the Rails app

Create a multi-stage `Dockerfile` that installs your gems, builds assets, and ships a slim runtime image:

```dockerfile
# syntax=docker/dockerfile:1.5
FROM ruby:3.3 AS build
WORKDIR /app

# Install system packages and gems
RUN apt-get update && apt-get install -y build-essential nodejs yarn
COPY Gemfile Gemfile.lock ./
RUN bundle install --without development test

# Copy source and precompile assets
COPY . .
RUN RAILS_ENV=production bundle exec rake assets:precompile

FROM ruby:3.3-slim
WORKDIR /app
ENV RAILS_ENV=production \
    PORT=3000

RUN apt-get update && apt-get install -y libvips && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/bundle /usr/local/bundle
COPY --from=build /app /app
CMD ["bundle", "exec", "puma", "-C", "config/puma.rb"]
```

Build and push the image:

```bash
docker build -t ghcr.io/acme/rails-app:latest .
docker push ghcr.io/acme/rails-app:latest
```

## Describe the app with Stack

Point a `StackApp` manifest at your image and port. You can start with static JWT authentication and enable OIDC later by adding `hostname-url`.

```yaml
# rails-stack-app.yaml
apiVersion: stack-cli.dev/v1
kind: StackApp
metadata:
  name: rails-app
  namespace: rails-demo
spec:
  web:
    image: ghcr.io/acme/rails-app:latest
    port: 3000
  auth:
    jwt: "development-token"
```

Deploy it:

```bash
stack install --manifest rails-stack-app.yaml
```

Stack will ensure the namespace exists, provision a CloudNativePG database cluster, inject credentials into `rails-app`, and run any database migrations via the migration container specified in future versions of the manifest.

## Expose it to the world

Run a quick tunnel while testing:

```bash
stack cloudflare --manifest rails-stack-app.yaml --name rails-demo
stack status --manifest rails-stack-app.yaml
```

When you are ready for a permanent hostname, pass `--token` with a Cloudflare tunnel credential and update `auth.hostname-url` inside the manifest so Keycloak and OAuth2 Proxy enforce proper redirects.
