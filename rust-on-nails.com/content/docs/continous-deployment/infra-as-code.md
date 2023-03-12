+++
title = "Infrastructure as Code"
description = "Infrastructure as Code"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 20
sort_by = "weight"

[extra]
toc = true
top = false
+++

## Introduction

However we decide to deploy our application "Infrastructure as Code" is a best practice. That means we have the ability to reproduce or duplicate our deployment environment from code stored in git. 

We will use [Pulumi](https://www.pulumi.com/) as this gives us the ability to code how we deploy our infrastructure whether that be with Heroku, Digital Ocean, Google Cloud, Azure and on and on.

We will use Pulumi with Typescript. They don't have a Rust option and we are already using typescript to add front end enhancement so we can keep the number of languages we need to know down to 2.

Usually I keep Pulumi code in a separate repository. This is because usually I'm configuring infrastructure to run multiple projects. Then each project can have it's own Pulumi.yaml which can be configured just for what the particular projects needs.

## Installing Pulumi

Everything you need to use Pulumi is installed into our `devcontainer`.

We'll create a folder called `infra-as-code` at the top level then set the folder up.

```
mkdir infra-as-code && cd infra-as-code
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

You should now have a folder structure that looks like the following


```sh
.
├── .devcontainer/
│   └── ...
├── crates/
│   └── ...
├── infra-as-code/
│   ├── node_modules/
│   │   └── ...
│   └── .gitignore
│   └── index.ts
│   └── package-lock.json
│   └── package.json
│   └── tsconfig.json
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

## Configuring namespaces and adding a Postgres Operator

Change your `index.ts` to look like the following.

```typescript
import * as k8s from "@pulumi/kubernetes"
import * as kx from "@pulumi/kubernetesx"

const nameSpace = new k8s.core.v1.Namespace('rust-on-nails', {
    metadata: {
        name: 'rust-on-nails'
    },
})
```

OK. Let's run `opulumi up` and see what we get.

```sh
$ pulumi up
Previewing update (dev)

View Live: https://app.pulumi.com/ianpurton/infra-as-code/dev/previews/18c545e4-d7d3-4dbe-bae7-6fc4302304eb

     Type                              Name               Plan       
 +   pulumi:pulumi:Stack               infra-as-code-dev  create     
 +   ├─ kubernetes:core/v1:Namespace   rust-on-nails      create     
 +   ├─ kubernetes:core/v1:Namespace   cloud-native-pg    create     
 +   └─ kubernetes:helm.sh/v3:Release  cloudnative-pg     create     


Resources:
    + 4 to create

Do you want to perform this update? yes
Updating (dev)

View Live: https://app.pulumi.com/ianpurton/infra-as-code/dev/updates/1

     Type                              Name               Status             
 +   pulumi:pulumi:Stack               infra-as-code-dev  created (3s)       
 +   ├─ kubernetes:core/v1:Namespace   rust-on-nails      created (0.36s)    
 +   ├─ kubernetes:core/v1:Namespace   cloud-native-pg    created (0.59s)    
 +   └─ kubernetes:helm.sh/v3:Release  cloudnative-pg     created (14s)      


Resources:
    + 4 created

Duration: 24s
```

## Getting familiar with k9s

[k9s](https://k9scli.io/) is a terminal based UI to interact with your Kubernetes clusters. Fire it up.

```sh
k9s
```

It looks something like the image below and gives you the ability to see running pods and view the logs.

![Adding secrets to cloak](../pods.png)

## Creating a Database and Users