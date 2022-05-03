+++
title = "Development Environment as Code"
description = "T"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

The [Visual Studio Code Remote - Containers](https://code.visualstudio.com/docs/remote/containers) extension lets you use a Docker container as a full-featured development environment. This fixes the following problems

* Enables developers other than yourself to get quickly up to speed
* Stops issues such as "It works on my machine"
* Allows you to check your development environment into git.

Install the devcontainer extension in VSCode and then setup a Rust environment.

![Creating a vault](/containers-extension.png)

The select *Open folder in container...* Select Rust and Postgres. Also select node on the next menu.

![Creating a vault](/devcontainers.png)

## Folder Structure

How you folder structure will look.

```sh
.
├── .cargo/
└── .devcontainer/
    ├── .env
    ├── devcontainer.json
    ├── docker-compose.yml
    └── Dockerfile
```

We are going to create a workspace for our web application. Create a new `Cargo.toml` file in the root folder and add the following.

```toml
[workspace]
members = [
    "app",
]
```

The run the following command.

```
$ cargo new app
```

You should now have a folder structure like the following.

```sh
├── .devcontainer/
│   └── ...
└── app/
│   ├──src/
│   │  └── main.rs
│   └── Cargo.toml
├── Cargo.toml
└── Cargo.lock
```

## Testing

Test out you development environment with

```
$ cargo run
   Compiling app v0.1.0 (/workspace/app)
    Finished dev [unoptimized + debuginfo] target(s) in 1.16s
     Running `target/debug/app`
Hello, world!
```