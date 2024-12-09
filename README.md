## Rust on Nails

A full stack architecture for web development with Rust.

![Rust on Nails](./rust-on-nails.com/static/yay-your-on-nails.png)

## Building the static site

Run inside the supplied `devcontainer`. You'll need to fetch the template which is a git submodule.

1. git submodule init
1. git submodule update
1. `cd rust-on-nails.com`
1. `zs` which is an alias for `zola serve --interface 0.0.0.0 --port 2222`

The access the site from http://localhost:2222