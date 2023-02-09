+++
title = "gRPC Web"
description = "gRPC Web"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
weight = 20
sort_by = "weight"

[extra]
toc = true
top = false
+++

## Browser to Server communications

To be able to use gRPC from the browser we need to enable [gRPC Web](https://github.com/grpc/grpc-web).

Tonic comes with a gRPC-Web to gRPC layer so let's add that.

## Install Tonic Web

Add the following to your Cargo.toml below the dependencies section.

```toml
tonic-web = "0"
```

Then change your `grpc_service` and `hybrid_make_service` in `main.rs` to the following.

```rust
let grpc_service = tonic::transport::Server::builder()
    .accept_http1(true)
    .add_service(tonic_web::enable(crate::api::fortunes_server::FortunesServer::new(
        api_service::FortunesService { pool },
    )))
    .into_service();

let hybrid_make_service = hybrid::hybrid(axum_make_service, grpc_service);
```

## BloomRPC

Now the server will respond to gRPC and gRPC web calls. We can test this with Bloom RPC by clicking on the GRPC button and setting it to WEB.

![BloomRPC](/bloom-rpc-web.png)

## Generating Typescript Client Stubs

To generate gRPC client stubs in your asset pipeline we can use the [protobuf-ts](https://github.com/timostamm/protobuf-ts) project. Run the following from the `app` folder.

```sh
npm install @protobuf-ts/plugin
npm install @protobuf-ts/grpcweb-transport
```

Update the `scripts` section to your `app/package.json` i.e.

```json
...
"scripts": {
    "start": "parcel watch ./asset-pipeline/index.ts",
    "release": "parcel build ./asset-pipeline/index.ts",
    "protoc": "npx protoc --ts_out ./asset-pipeline --proto_path ../protos ../protos/api.proto"
},
...
```

Now the grpc client will be built and you can import it into your front end code.

```typescript
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
```

For more read the docs on [https://github.com/timostamm/protobuf-ts](https://github.com/timostamm/protobuf-ts)