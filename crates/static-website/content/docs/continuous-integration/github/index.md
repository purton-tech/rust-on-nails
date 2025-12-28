# Running CI on Github

We describe our container build in Rust with [Dagger](../dagger/), so GitHub only needs to run the pipeline binary (`cargo run -p infrastructure`) to exercise the full stack in CI. Create `.github/workflows/ci.yml` with the workflow below.

## Github Action

```yaml
name: CI

on:
  push: # build verification for commits landing on main
    branches:
      - main
  pull_request: # build verification for PRs that target main
    branches:
      - main

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
    env:
      FORCE_COLOR: 1
    steps:
    - name: Checkout
      uses: actions/checkout@v4

    - name: Run build pipeline # reuses the Dagger build code from the infrastructure crate
      run: cargo run --release -p infrastructure -- build
```

- The workflow checks the repo out and runs on both pushes to `main` and pull requests targeting `main`.
- It runs the Rust pipeline from [Build Our Containers (Dagger)](../dagger/) directly with `cargo run -- build` so every CI run compiles the web server, WASM artifacts, and Tailwind output.
- Because this workflow is purely for verification, it never authenticates to registries or publishes artifacts—those steps live in the dedicated release workflow.

## Packages

The screenshot below shows how your Github repo should look after the pipeline has run.

![Github Repo](./github-repo.png)
