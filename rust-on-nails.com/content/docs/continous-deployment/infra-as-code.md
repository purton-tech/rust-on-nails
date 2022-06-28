+++
title = "Infrastructure as Code"
description = "Infrastructure as Code"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

## Introduction

However we decide to deploy our application "Infrastructure as Code" is a best practice. That means we have the ability to reproduce or duplicate our deployment environment from code stored in git. 

We will use [Pulumi](https://www.pulumi.com/) as this gives us the ability to code how we deploy our infrastructure wether that be with Heroku, Digital Ocean, Google Cloud, Azure and on and on.

Basically, rather than learning the providers tools, we get good at one tool and use it across providers.

We will use Pulumi with Typescript. They don't have a Rust option and we are already using typescript to add front end enhancement so we can keep the number of languages we need to know down to 2.

## Installing Pulumi

Everything you need to use Pulumi is installed into our `devcontainer`.

We'll creat a folder called infra at the top level then set the folder up.

```
mkdir infra && cd infra
```

Then run the setup.

```
pulumi new kubernetes-typescript
```

You'll need to create a Pulumi [https://www.pulumi.com/](https://www.pulumi.com/) account which is free so that you can get an API key.

After a while you should get something like the following

```sh
Finished installing dependencies

Your new project is ready to go! 

To perform an initial deployment, run 'pulumi up'
```

And a folder structure that looks like the following


```sh
.
├── .devcontainer/
│   └── ...
├── app/
│   └── ...
├── db/
│   └── ...
├── infra/
│   ├── node_modules/
│   │   └── ...
│   └── .gitignore
│   └── index.ts
│   └── package-lock.json
│   └── package.json
│   └── tsconfig.json
├── protos/
│   └── ...
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```