#!/bin/bash

# Get the version and container name from Semantic Release
VERSION=$1
CONTAINER_NAME="purtontech/rust-on-nails-devcontainer"

echo "Updating files..."
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.fi.md
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.md
sed -i "s/purtontech\/rust-on-nails-devcontainer:.*$/purtontech\/rust-on-nails-devcontainer:$1 AS development/" ./nails-devcontainer/.devcontainer/Dockerfile

docker pull $CONTAINER_NAME:latest
docker tag $CONTAINER_NAME:latest $CONTAINER_NAME:$VERSION
docker push $CONTAINER_NAME:$VERSION

git add .
git commit -am "chore(deployment): Update files with new version $1"
git push