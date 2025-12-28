# Releases on Github

We cut releases with semantic commits so the version bumps are automatic. Conventional commit types drive the semver change: `feat:` raises the minor version, `fix:` bumps the patch, and adding a breaking change marker (`!`) turns that into a major release. This keeps release automation predictable while still letting humans read the history.

## Workflow overview

Create `.github/workflows/release.yml` alongside the CI workflow. It has two jobs: `get-version` determines the next semantic version without publishing anything, and `release` builds/tag/pushes the containers. Everything lives in a single file for easy copy/paste:

```yaml
name: Release

on:
  workflow_dispatch:
  push:
    branches:
      - main

jobs:
  get-version:
    name: Determine semantic version
    runs-on: ubuntu-latest
    outputs:
      version: ${{ steps.semver.outputs.version }}
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install semantic-release
        run: npm install -g semantic-release @semantic-release/commit-analyzer @semantic-release/release-notes-generator

      - name: Create .releaserc.json
        run: |
          cat > .releaserc.json <<'EOF'
          {
            "branches": [{ "name": "main" }],
            "tagFormat": "v${version}",
            "plugins": [
              "@semantic-release/commit-analyzer",
              "@semantic-release/release-notes-generator"
            ]
          }
          EOF

      - name: Calculate next version (dry-run)
        id: semver
        run: |
          set -euo pipefail
          npx semantic-release --dry-run | tee sr.log
          VER=$(grep -Eo 'next release version is [0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z\.-]+)?' sr.log | awk '{print $5}' || true)
          echo "version=${VER}" >> $GITHUB_OUTPUT

      - name: Print calculated version
        run: echo "Next version is ${{ steps.semver.outputs.version }}"

  release:
    name: Build and publish containers
    needs: get-version
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write
    env:
      FORCE_COLOR: 1
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Log in to GitHub Container Registry
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Build and push versioned images
        env:
          VERSION: ${{ needs.get-version.outputs.version }}
          WEB_REPO: ghcr.io/${{ github.repository_owner }}/${{ github.event.repository.name }}
          MIGRATIONS_REPO: ghcr.io/${{ github.repository_owner }}/${{ github.event.repository.name }}-migrations
        run: |
          if [[ -z "$VERSION" ]]; then
            echo "No release-worthy commits detected. Skipping publish."
            exit 0
          fi
          cargo run --release -p infrastructure -- build \
            --web-tag "$WEB_REPO:$VERSION" \
            --migrations-tag "$MIGRATIONS_REPO:$VERSION"
```

- The release job reuses the same Rust/Dagger pipeline as CI, but now it supplies explicit tags so the resulting containers are published with the semantic version.
- If semantic-release does not find a new version (for example, only docs changes landed), the script exits early and nothing is published.
- Once the images exist, you can follow up with `semantic-release` (without `--dry-run`) or a changelog workflow if you want GitHub Releases as well.
