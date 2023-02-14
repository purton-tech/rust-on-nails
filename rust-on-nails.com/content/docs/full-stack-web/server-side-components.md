+++
title = "Server Side Components"
description = "Server Side Components"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 70
sort_by = "weight"


[extra]
toc = true
top = false
+++

The [Dioxus](https://dioxuslabs.com/) framework gives us the capability to build user interfaces out of components which can be rendered server isde. It's worth looking at their [components documenation](https://dioxuslabs.com/guide/components/index.html).

## Creating a ui-componenst crate

Edit your Cargo.toml so it now looks like.

```toml
[workspace]
members = [
    "crates/actix-server",
    "crates/db",
    "crates/ui-components",
]

```

```sh
cargo init --lib crates/ui-components
```

## Install Dioxus

```sh
cd crates/ui-components
cargo add dioxus@0.2 --features ssr
```

## Creating a Layout Component
