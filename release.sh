#!/bin/bash
echo "Updating files..."
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.fi.md
sed -i "s/repo_version = .*$/repo_version =  \"Github $1\"/" ./rust-on-nails.com/content/_index.md
sed -i "s/purtontech\/rust-on-nails-devcontainer-amd64:.*$/purtontech\/rust-on-nails-devcontainer-amd64:$1 AS development/" ./nails-devcontainer/.devcontainer/Dockerfile

git add .
git commit -am "chore(deployment): Update files with new version $1"
git push