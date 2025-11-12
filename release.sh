#!/bin/bash

# Get the version and container name from Semantic Release
VERSION=$1
CONTAINER_NAME="purtontech/rust-on-nails-devcontainer"
OPERATOR_IMAGE="purtontech/stack-operator"

echo "Updating files..."
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.fi.md
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.md
sed -i "s/purtontech\/rust-on-nails-devcontainer:.*$/purtontech\/rust-on-nails-devcontainer:$1 AS development/" ./nails-devcontainer/.devcontainer/Dockerfile
sed -i "s#purtontech/stack-operator:.*#purtontech/stack-operator:$1#" ./crates/stack-cli/config/operator.yaml

echo "Building and pushing Stack operator image..."
earthly --ci --push +stack-operator-image --IMAGE ${OPERATOR_IMAGE}:${VERSION}

# Create a multi-platform tag for the specified version by referencing the `latest` tag
docker buildx imagetools create -t $CONTAINER_NAME:$VERSION $CONTAINER_NAME:latest
docker buildx imagetools create -t $OPERATOR_IMAGE:latest $OPERATOR_IMAGE:$VERSION

git add .
git commit -am "chore(deployment): Update files with new version $1"
git push
