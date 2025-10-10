# Rust on Nails Agents Guide

This repository mixes several deliverables that move at different speeds. To keep things tidy, we split the work into three "agents" that each own one slice of the Rust on Nails experience. Use this guide to decide where changes belong and how to validate them before opening a PR.

## Documentation Agent (`crates/static-website`)
- **Purpose**: Publish the public site that explains how to build Rust on Nails applications. The site is deployed to Cloudflare Pages.
- **Primary tasks**: Add or update docs in `content/` and assets in `assets/`. When adding tutorials, prefer runnable snippets that match the default project template.
- **Working locally**: Run `just wts` for Tailwind and `just ws` for the Rust/Dioxus generator. Browse `http://localhost:8080` to preview the rendered pages under `dist/`.
- **Before shipping**: `DO_NOT_RUN_SERVER=1 cargo run --bin static-website` to ensure the generator succeeds, then run the Cloudflare preview workflow if build or deployment config changed.

## Dev Environment Agent (`nails-devcontainer`)
- **Purpose**: Maintain the reusable development environment used by community members and contributors.
- **Primary tasks**: Keep `devcontainer-template.json` and wrapper scripts in sync with the CLI and website. Document required tooling in `README.md`.
- **Working locally**: Use `devcontainer up` (VS Code) or `devcontainer build --workspace-folder .` to verify changes. Ensure the container exposes port 8080 for the docs server and ships the CLI binaries or aliases referenced in the guides.
- **Before shipping**: Bump version tags when you change the base image or toolchain. Run at least one full `cargo test` inside the container to make sure the toolchain works.

## Platform Agent (`crates/nails-cli`)
- **Purpose**: Provide the internal developer platform that installs operators into Kubernetes clusters for Rust on Nails applications.
- **Primary tasks**: Extend the `k8s-operator` binary, manage Helm-like manifests under `config/`, and maintain integrations with Envoy and Keycloak.
- **Working locally**: `cargo run --bin k8s-operator -- -h` to inspect commands. Use `cargo run --bin k8s-operator -- operator` to run the controller loop, and the `init`/`install` subcommands to configure a cluster. The dev container already maps your kubeconfig—fall back to `tmp/kubeconfig` if needed.
- **Before shipping**: `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test`. When changing cluster assets, test against a local K3s install (`curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC='server --write-kubeconfig-mode="644"' sh -`). Document any new flags in `README.md`.

## Cross-Cutting Expectations
- Follow `CONTRIBUTING.md` for code review and branching conventions.
- If a change touches more than one agent area, coordinate early so reviewers from each area can weigh in.
- Prefer ASCII in docs and comments unless you have a compelling reason otherwise.
- Surface follow-up work with TODO comments (`// TODO(username):`) or GitHub issues so the right agent can pick them up.
