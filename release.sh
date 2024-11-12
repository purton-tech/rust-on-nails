#!/bin/bash

# Get the version and container name from Semantic Release
VERSION=$1
CONTAINER_NAME="purtontech/rust-on-nails-devcontainer"

echo "Updating files..."
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.fi.md
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.md
sed -i "s/purtontech\/rust-on-nails-devcontainer:.*$/purtontech\/rust-on-nails-devcontainer:$1 AS development/" ./nails-devcontainer/.devcontainer/Dockerfile

# Create a multi-platform tag for the specified version by referencing the `latest` tag
docker buildx imagetools create -t $CONTAINER_NAME:$VERSION $CONTAINER_NAME:latest

git add .
git commit -am "chore(deployment): Update files with new version $1"
git push