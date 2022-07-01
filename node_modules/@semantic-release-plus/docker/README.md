# @semantic-release-plus/docker

[![npm](https://img.shields.io/npm/v/@semantic-release-plus/docker.svg)](https://www.npmjs.com/package/@semantic-release-plus/docker)
[![downloads](https://img.shields.io/npm/dt/@semantic-release-plus/docker.svg)](https://www.npmjs.com/package/@semantic-release-plus/docker)
[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg)](https://github.com/prettier/prettier)
[![semantic-release-plus](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release--plus-e10079.svg)](https://github.com/semantic-release/semantic-release)
[![license](https://img.shields.io/npm/l/@semantic-release-plus/docker.svg)](https://github.com/semantic-release-plus/semantic-release-plus/blob/beta/packages/plugins/docker/LICENSE)

A [semantic-release-plus](https://github.com/semantic-release-plus/semantic-release) or [semantic-release](https://github.com/semantic-release/semantic-release) plugin for publishing a docker images to a docker registry.

| Step               | Description                                                                                                                                                               |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `verifyConditions` | Verify that all needed configuration and `DOCKER_USERNAME` and `DOCKER_PASSWORD` environment variables are present and logs into the Docker registry unless skipped.      |
| `addChannel`       | Tag an existing image with a new channel. _Run only if there are releases that have been merged from a higher branch but not added on the channel of the current branch._ |
| `publish`          | Tag the image specified by `name` as `{registry}/{name}:{version}` and `{registry}/{name}:{channel}` (based on configuration) and push it to the Docker registry.         |

## Install

```bash
$ npm install @semantic-release-plus/docker -D
```

## Usage

1. Build your docker image and tag it with a known name that does not include a registry.
2. Configure the plugin with the same name and specify the registry, if publishing to a registry other than docker.io. The registry to publish can either be defined in the `name` property, the `registry` property or the `name.registry` property.

The plugin can be configured in the [**semantic-release-plus** configuration file](https://github.com/semantic-release-plus/semantic-release/blob/master/docs/usage/configuration.md#configuration):

## Configuration

| Option              | Description                                                                                                                                                                                                                                                                                                                                                                                                                                      | Type             | Default   |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------- | --------- |
| **`name`**          | Required config associated with the tag name assigned to the image during build `docker build -t name`. This can include registry, namespace, repo, and suffix (either tag or sha) in the following format `<registry>/<namespace>/<repo>:<tag>` or `<registry>/<namespace>/<repo>@<sha>`. If a registry is defined as part of the name it will override the registry option. This can also be provided as an object, see example for interface. | `string\|object` |           |
| `registry`          | The docker registry to login and push images to, this will be pre-pended to the name field when tagging                                                                                                                                                                                                                                                                                                                                          | `string`         | docker.io |
| `publishChannelTag` | Will publish a channel (dist) tag such as `latest`, `next`, `beta`, `alpha`, `1`, and `1.1`, that always points to the most recent release on the channel. `1`, `1.1` tags will only be created on maintenance branches. See [Publishing maintenance releases](https://github.com/semantic-release-plus/semantic-release/blob/master/docs/recipes/maintenance-releases.md)                                                                       | `boolean`        | true      |
| `skipLogin`         | Skips logging in to docker hub in the verifyConditions step, used if you log in separately in your CI job. Removes requirement for `DOCKER_USERNAME` and `DOCKER_PASSWORD` environment variables                                                                                                                                                                                                                                                 | `boolean`        | false     |

### Name

The name field is required and can be either a string or an object, if a string it will be converted into an object using this syntax `<registry>/<namespace>/<repo>:<tag>` or `<registry>/<namespace>/<repo>@<sha>`.

| Property         | Description                                                                            | Type     | Default     |
| ---------------- | -------------------------------------------------------------------------------------- | -------- | ----------- |
| `registry`       | the registry that the images will be published to / pulled from when adding to channel | `string` | `undefined` |
| `namespace`      | often your username but can also be an organization name                               | `string` | `undefined` |
| **`repository`** | the repository of the image                                                            | `string` |             |
| `tag`            | the string following the `:` after the repo                                            | `string` | `undefined` |
| `repository`     | the sha value following the `@` after the repo                                         | `string` | `undefined` |

```jsonc
{
  "name": {
    "registry": "string", // optional
    "namespace": "string", // optional
    "repository": "string", // required
    "tag": "string", // optional
    "sha": "string" // optional
  }
}
```

### Example Configurations

Consider the image was built and tagged with the following command. The following configurations will result in the same behavior.

```bash
docker build -t `my-namespace/my-repo:my-tag` .
```

```json
{
  "release": {
    "plugins": [
      [
        "@semantic-release-plus/docker",
        {
          "name": "ghcr.io/my-namespace/my-repo:my-tag"
        }
      ]
    ]
  }
}
```

```json
{
  "release": {
    "plugins": [
      [
        "@semantic-release-plus/docker",
        {
          "name": "my-namespace/my-repo:my-tag",
          "registry": "ghcr.io"
        }
      ]
    ]
  }
}
```

```json
{
  "release": {
    "plugins": [
      [
        "@semantic-release-plus/docker",
        {
          "name": {
            "registry": "ghcr.io",
            "namespace": "my-namespace",
            "repository": "my-repo",
            "tag": "my-tag"
          }
        }
      ]
    ]
  }
}
```

The docker commands that will be run with this config are the following when releasing version 1.2.3

```bash
docker tag my-namespace/my-repo:my-tag ghcr.io/my-namespace/my-repo:1.2.3
docker tag my-namespace/my-repo:my-tag ghcr.io/my-namespace/my-repo:latest
docker push ghcr.io/my-namespace/my-repo:1.2.3
docker push ghcr.io/my-namespace/my-repo:latest
```

In the situation where you are adding an existing build to a channel it will pull the existing build from the registry and tag it with the new channel. With the above config and adding build 1.2.4. from the next channel to the latest channel it will run the following commands.

```bash
docker pull ghcr.io/my-namespace/my-repo:1.2.4
docker tag ghcr.io/my-namespace/my-repo:1.2.4 ghcr.io/my-namespace/my-repo:latest
docker push ghcr.io/my-namespace/my-repo:latest
```

## Example github action publishing to ghcr.io

The following is an example github action configuration, the source repo can be found at https://github.com/JoA-MoS/srp-docker-example

```yml
name: CI

on:
  push:
    branches:
      - master
      - next
      - beta
      - alpha
      - '*.x'
  pull_request:
    types:
      - opened
      - synchronize

jobs:
  build_release:
    runs-on: ubuntu-latest
    steps:
      - name: Build
        uses: actions/checkout@v2
        with:
          fetch-depth: 0
      - run: docker build --tag joa-mos/srp-docker-example .
      - name: Release
        uses: actions/setup-node@v2
        with:
          cache: npm
      - run: npm ci
      - run: npx semantic-release-plus
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          DOCKER_USERNAME: joa-mos
          DOCKER_PASSWORD: ${{ secrets.DOCKER_PASSWORD }}
```

## Example .travis.yml

```yml
jobs:
  include:
    - stage: release
      language: node_js
      node_js: '8'
      services:
        - docker
      script:
        - docker build -t namespace/repository .
        - npm run semantic-release-plus

stages:
  - test
  - name: release
    if: branch = master AND type = push AND fork = false

branches:
  except:
    - /^v\d+\.\d+\.\d+$/
```

## Circle CI Example .config.yml

```yml
version: 2
jobs:
  release:
    docker:
      - image: circleci/node:8
    steps:
      - setup_remote_docker:
          docker_layer_caching: true
      - run:
          name: release
          command: |
            docker build -t namespace/repository .
            npm run semantic-release-plus

workflows:
  version: 2
  pipeline:
    jobs:
      - test
      - release:
          requires:
            - test
          filters:
            branches:
              only: master
```

> Note that `setup_remote_docker` step is required for this plugin to work in Circle CI environment

## How to keep new version in package.json inside docker image?

It is best to let semantic-release focus on releasing your built artifact and not extend semantic-release to also do the build. I recommend using semantic-release-plus to get the next version without creating a tag then using that durning your build process. An example of this can be found in the semantic-release-plus [Expected next version recipe](https://github.com/semantic-release-plus/semantic-release/blob/20b0c4420e5466a7d7ed16fb3fe4981609173187/docs/recipes/expected-next-version.md#L1).

## Publishing to multiple registries

You should be able to publish to multiple registries by adding the docker plugin multiple times with different registry configurations. You will need to login to the registries outside of the plugin and configure the plugin to skip login (unless you happen to have the same username and password between different registries)
