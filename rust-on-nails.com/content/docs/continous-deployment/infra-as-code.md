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

## Installing Some Pulumi Dependencies

We'll need to include another Pulumi library.

```
npm install @pulumi/random
```

## Configuring namespaces and adding a Postgres Operator

Change your `index.ts` to look like the following.

```typescript
import * as pulumi from "@pulumi/pulumi"
import * as k8s from "@pulumi/kubernetes"
import * as kx from "@pulumi/kubernetesx"
import * as random from "@pulumi/random"

// Setup a namespace for Cloud Native Pg https://github.com/cloudnative-pg/cloudnative-pg
const databaseNameSpace = new k8s.core.v1.Namespace('cloud-native-pg', {
    metadata: {
        name: 'cloud-native-pg'
    },
})

// Install the Postgres operator from a helm chart
const cloudnativePg = new k8s.helm.v3.Release("cloudnative-pg", {
    chart: "cloudnative-pg",
    namespace: databaseNameSpace.metadata.name,
    repositoryOpts: {
        repo: "https://cloudnative-pg.github.io/charts",
    }
}); 
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

Extend our `index.ts` and add the following code under the code we already created above. 

This code is responsible for creating a namespace called `rust-on-nails` we then install Postgres into that name space and setup a Kubernetes secret called `database-urls` so that our application can connect to the database.

```typescript
// Setup a namespace for our application
const applicationNameSpace = new k8s.core.v1.Namespace('rust-on-nails', {
    metadata: {
        name: 'rust-on-nails'
    },
})

// Create 2 Database passwords and store them as secrets
const migrationPassword = new random.RandomPassword("app_password", {
    length: 20,
    special: false,
});

const DATABASE_NAME="app"
const MIGRATIONS_ROLE="app"

const appSecret = new kx.Secret("app-secret", {
    type: "kubernetes.io/basic-auth",
    metadata: {
        namespace: applicationNameSpace.metadata.name,
        name: "app-secret"
    },
    stringData: {
        "username": MIGRATIONS_ROLE,
        "password": migrationPassword.result,
    }
})

const pgCluster = new k8s.apiextensions.CustomResource('nails-db-cluster', {
    apiVersion: 'postgresql.cnpg.io/v1',
    kind: 'Cluster',
    metadata: {
        name: 'nails-db-cluster',
        namespace: applicationNameSpace.metadata.name,
    },
    spec: {
        instances: 1,
        bootstrap: {
            initdb: {
                database: DATABASE_NAME,
                // Bootstrap uses the secrerts we created
                // above to give us a user
                owner: appSecret.stringData.username,
                secret: {
                    name: appSecret.metadata.name
                },
                postInitSQL: [
                    // Add users here.
                    // "CREATE ROLE cloak_application LOGIN ENCRYPTED PASSWORD 'testpassword'"
                ]
            }
        },
        storage: {
            size: '1Gi'
        }
    }
}, {
    dependsOn: cloudnativePg
})

let migrationsUrl = pulumi.all([migrationPassword.result])
    .apply(([password]) => 
    `postgres://${MIGRATIONS_ROLE}:${password}@cluster-sample-rw:5432/${DATABASE_NAME}?sslmode=require`)


// Create a database url secret so our app will work.
new kx.Secret("database-urls", {
    metadata: {
        namespace: applicationNameSpace.metadata.name,
        name: "database-urls"
    },
    stringData: {
        "migrations-url": migrationsUrl
    }
})
```

Run `pulumi up` to apply our latest configuration.

## Connecting to the database

```sh
kubectl port-forward service/nails-db-cluster-rw 5455:5432 --namespace=rust-on-nails
```

You'll need to get the database password from the `database-urls` secret.

```sh
psql -p 5455 -h 127.0.0.1 -U app nails_migrations
```