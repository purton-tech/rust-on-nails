+++
title = "Running CI on Github"
weight = 20
sort_by = "weight"
+++

We develop and test our CI pipeline with Earthly, then we can create a Github Action to trigger our earthly build.

Create a file `.github/workflows/ci.yml` and add the following

## Github Action

```yaml
name: CI

on:
  push:
    branches: 
      - main
  pull_request:
    branches:
      - main

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      packages: write
      contents: read
    env:
      FORCE_COLOR: 1
    steps:
    - uses: earthly/actions/setup-earthly@v1
      with:
        version: v0.6.0
    - uses: actions/checkout@v2

    - name: Put back the git branch into git (Earthly uses it for tagging)
      run: |
        branch=""
        if [ -n "$GITHUB_HEAD_REF" ]; then
          branch="$GITHUB_HEAD_REF"
        else
          branch="${GITHUB_REF##*/}"
        fi
        git checkout -b "$branch" || true

    - name: Log in to the Container registry
      uses: docker/login-action@f054a8b539a109f9f41c372932f1ae047eff08c9
      with:
        registry: ghcr.io
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}

    - name: Earthly version
      run: earthly --version

    - name: Run build
      # Allow privelaged is required to run docker in docker
      run: earthly --allow-privileged +all

    - name: Tag and Push Images
      run: |
        docker tag rustonnails/app ghcr.io/${{ github.repository_owner }}/${{ github.event.repository.name }}
        docker tag rustonnails/app-migrations ghcr.io/${{ github.repository_owner }}/${{ github.event.repository.name }}-migrations
        docker push ghcr.io/${{ github.repository_owner }}/${{ github.event.repository.name }}:latest
        docker push ghcr.io/${{ github.repository_owner }}/${{ github.event.repository.name }}-migrations:latest
```

This will run our earthly build and push our docker images to the Github Container Registry.

The images will be renamed to match your Github organisation and project.

## Packages

The screenshot below shows how your Github repo should look after the pipeline has run.

![Github Repo](../github-repo.png)