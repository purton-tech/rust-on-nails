#!/bin/bash
echo "Updating files..."
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.fi.md
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.md
sed -i "s/purtontech\/rust-on-nails-devcontainer:.*$/purtontech\/rust-on-nails-devcontainer:$1 AS development/" ./dev-env-as-code/Dockerfile.devcontainer
sed -i "s/purtontech\/rust-on-nails-devcontainer:.*$/purtontech\/rust-on-nails-devcontainer:$1 AS development/" ./nails-devcontainer/Dockerfile

git add .
git commit -am "chore(deployment): Update files with new version $1"
git push