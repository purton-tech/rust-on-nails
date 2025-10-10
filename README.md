## Rust on Nails

A full stack architecture for web development with Rust.

![Rust on Nails](crates/static-website/content/docs/yay-your-on-nails.png)

## Building the static site

The marketing and documentation site now lives in `crates/static-website`. It is a self‑contained Rust application that renders pages with Dioxus SSR, writes them into `dist/`, and serves them over Axum for local development. The same binary is used in Cloudflare Pages deploys.

### Running locally (inside the devcontainer)

1. `just wts` — build Tailwind assets on change (`tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch`).
1. `just ws` — run the generator in watch mode (`cargo watch ... -x "run --bin static-website"`).

Then browse to http://localhost:8080. The Axum server injects live reload and serves the freshly rendered files from `dist/`.

### Building for Cloudflare

Set `DO_NOT_RUN_SERVER=1` so the binary generates the site without starting the dev server, then run:

```bash
cd crates/static-website
DO_NOT_RUN_SERVER=1 cargo run --bin static-website
```

Cloudflare Pages uses `crates/static-website/cloudflare-build.sh` to perform this same build step during deployment.
