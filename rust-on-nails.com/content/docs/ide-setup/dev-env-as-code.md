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

## Installation

Install the devcontainer extension in VSCode and then setup a Rust environment.

![Creating a vault](/containers-extension.png)

## Install Rust on Nails

We have pre-configured a development environment with all the tools needed to create a full stack rust application.

To get started create a folder for your project. Change directory into that folder then run.

```
mkdir project-name
cd project-name
```

### MacOS and Linux

```
mkdir .devcontainer \
  && curl -L https://github.com/purton-tech/rust-on-nails/archive/main.tar.gz -O -J \
  && tar -xf rust-on-nails-main.tar.gz \
  && mv rust-on-nails-main/dev-env-as-code/Dockerfile.devcontainer .devcontainer/Dockerfile \
  && mv rust-on-nails-main/dev-env-as-code/docker-compose.yml .devcontainer \
  && mv rust-on-nails-main/dev-env-as-code/devcontainer.json .devcontainer \
  && mv rust-on-nails-main/dev-env-as-code/ps1.bash .devcontainer \
  && mv rust-on-nails-main/dev-env-as-code/.bash_aliases .devcontainer \
  && mv rust-on-nails-main/dev-env-as-code/.gitignore . \
  && mv rust-on-nails-main/dev-env-as-code/.githooks .devcontainer \
  && rm -rf rust-on-nails-main*
```

### Windows

```
mkdir .devcontainer 
curl -L https://github.com/purton-tech/rust-on-nails/archive/main.zip -O -J \
tar -xf rust-on-nails-main.zip \
move rust-on-nails-main\dev-env-as-code\Dockerfile.devcontainer .devcontainer\Dockerfile
move rust-on-nails-main\dev-env-as-code\docker-compose.yml .devcontainer 
move rust-on-nails-main\dev-env-as-code\devcontainer.json .devcontainer 
move rust-on-nails-main\dev-env-as-code\ps1.bash .devcontainer 
move rust-on-nails-main\dev-env-as-code\.bash_aliases .devcontainer 
move rust-on-nails-main\dev-env-as-code\.gitignore . 
move rust-on-nails-main\dev-env-as-code\.githooks .devcontainer 
del /S rust-on-nails-main.zip
rmdir /S /Q rust-on-nails-main
```

## VS Code

Load the folder into visual studio code. On the bottom left corner of VS Code you should see a green icon. Click on this and select open in container.

After the container is downloaded you will have a preconfigured development environment with the following folder structure.

How you folder structure will look.

```sh
.
└── .devcontainer/
    ├── .bash_aliases
    ├── .githooks/
    │   └── precommit
    ├── devcontainer.json
    ├── docker-compose.yml
    └── Dockerfile
└── target
└── .gitignore
```

## Setting up Git

Open up a terminal in VSCode (CTRL + `) and execute

```sh
$ git init
Initialized empty Git repository in /workspace/.git/
```

## Add a Workspace

We are going to create a workspace for our web application. Create a new `Cargo.toml` file in the root folder and add the following.

```toml
[workspace]
members = [
    "app",
]
```

Open up the terminal in VSCode again and run the following

```
$ cargo new app
     Created binary (application) `app` package
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

## Commit your code

From the `/workspace` folder

```
$ git add .
$ git commit -m"Initial Commit"
```