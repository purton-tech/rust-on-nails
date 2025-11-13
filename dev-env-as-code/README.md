## Build (Doesn't work)

docker build --build-arg TARGETARCH=amd64 --build-arg BUILDPLATFORM=linux/amd64 .

## Run the Arm image

```sh
docker run -it --platform linux/arm64 purtontech/rust-on-nails-devcontainer:1.3.13 sh
```