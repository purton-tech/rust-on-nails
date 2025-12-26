# Dagger

[https://dagger.io](Dagger) allows us to define our build using Rust. It's also based around docker so we can re-use our devcontainer.

## The Prompt

We'll use AI to setup our build. Use the following prompt

```
Using the  `Justfile` you can figure out how we build the code

- `clorinde` 
- build the webassembly that hydrates the front end 
- `just islands` generate tailwind 
- `just tailwind` 
- and finally build the server `watch`.

Create a dagger pipeline using dagger-sdk = "0.19" that builds 2 containers.

- A database migration container using `dbmate`
- The web server

We are using `devconatiners` and all our build tools are in the conatiner referenced by `.devconatiners/Dockerfile`

- Use the conatiner in `.devconatiners/Dockerfile` for the build
- The web container must be built from `scratch`

Call the pipeline `crates/infrastructure` it will be a cli with initally one command `build`.
```