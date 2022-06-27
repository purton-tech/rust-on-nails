+++
title = "Integrating gRPC"
description = "gRPC"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
weight = 90
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

## Why gRPC?

[gRPC](https://grpc.io/) has gives us a nice way to declare our API in a schema definition and then code generate the server side implementation. 

The data transfer protocol (Protobuf) is also useful if we want to do data storage i.e. for Big data projects. So we get 1 tool that can do 2 jobs.

## Installation

Add the following to your `app/Cargo.toml` below the `[dependencies]` 

```toml
# gRPC dependencies
tonic = "0"
prost = "0"
# Needed for our hybrid server (i.e. gRPC and Web)
pin-project = "1"
tower = { version = "0", default-features = false }
hyper = { version = "0", features = ["server"] }
```

Add the following to your `app/Cargo.toml` below the `[build-dependencies]` 

```toml
tonic-build = "0"
```

Create a folder called `protos` and a file called `api.proto`

```sh
.
├── .devcontainer/
│   └── ...
├── app/
│   └── ...
├── db/
│   └── ...
├── protos/
│   └── api.proto
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

## Defining the API

```proto
syntax = "proto3";

package api;

service Fortunes {
    rpc GetFortunes(GetFortunesRequest) returns (GetFortunesResponse);
}

message GetFortunesRequest {
}

message GetFortunesResponse {
    repeated Fortune fortunes = 1;
}

message Fortune {
    uint32 id = 1;
    string message = 2;
}
```

## Updating our build.rs

Add the following to your `app/build.rs` in the `main` function.

```rust
fn main() -> Result<()> {

    ...

    tonic_build::configure()
    .compile(
        &["api.proto"], // Files in the path
        &["../protos"], // The path to search
    )
    .unwrap();

    ...

    Ok(())
}
```

Everything should compile at this point.

## Implementing our API End Point

Create a file called `app/api_service.rs` and add the following implemetation for our gRPC service.

```rust
use crate::api::*;
use crate::errors::CustomError;
use crate::queries;
use deadpool_postgres::Pool;
use tonic::{Request, Response, Status};

pub struct FortunesService {
    pub pool: Pool,
}

#[tonic::async_trait]
impl crate::api::fortunes_server::Fortunes for FortunesService {
    async fn get_fortunes(
        &self,
        _request: Request<GetFortunesRequest>,
    ) -> Result<Response<GetFortunesResponse>, Status> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        let fortunes = queries::fortunes::fortunes(&client)
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        // Map the structs we get from cornucopia to the structs
        // we need for our gRPC reply.
        let fortunes = fortunes
            .into_iter()
            .map(|fortune| Fortune {
                id: fortune.id as u32,
                message: fortune.message,
            })
            .collect();

        let response = GetFortunesResponse {
            fortunes,
        };

        return Ok(Response::new(response));
    }
}

```

## Integrating Tonic and Axum

The last thing we need to do is add our API to the Axum server. Tonic uses [Hyper](https://github.com/hyperium/hyper) as the underlying server for it's gRPC implemenation. Axum uses Hyper too so we can get our server to run both our web pages and our api.

Create the following file `app/src/hybrid.rs`.

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use tower::Service;

use hyper::HeaderMap;
use hyper::{body::HttpBody, Body, Request, Response};
use pin_project::pin_project;

pub fn hybrid<MakeWeb, Grpc>(make_web: MakeWeb, grpc: Grpc) -> HybridMakeService<MakeWeb, Grpc> {
    HybridMakeService { make_web, grpc }
}

pub struct HybridMakeService<MakeWeb, Grpc> {
    make_web: MakeWeb,
    grpc: Grpc,
}

impl<ConnInfo, MakeWeb, Grpc> Service<ConnInfo> for HybridMakeService<MakeWeb, Grpc>
where
    MakeWeb: Service<ConnInfo>,
    Grpc: Clone,
{
    type Response = HybridService<MakeWeb::Response, Grpc>;
    type Error = MakeWeb::Error;
    type Future = HybridMakeServiceFuture<MakeWeb::Future, Grpc>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.make_web.poll_ready(cx)
    }

    fn call(&mut self, conn_info: ConnInfo) -> Self::Future {
        HybridMakeServiceFuture {
            web_future: self.make_web.call(conn_info),
            grpc: Some(self.grpc.clone()),
        }
    }
}

#[pin_project]
pub struct HybridMakeServiceFuture<WebFuture, Grpc> {
    #[pin]
    web_future: WebFuture,
    grpc: Option<Grpc>,
}

impl<WebFuture, Web, WebError, Grpc> Future for HybridMakeServiceFuture<WebFuture, Grpc>
where
    WebFuture: Future<Output = Result<Web, WebError>>,
{
    type Output = Result<HybridService<Web, Grpc>, WebError>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        let this = self.project();
        match this.web_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Ready(Ok(web)) => Poll::Ready(Ok(HybridService {
                web,
                grpc: this.grpc.take().expect("Cannot poll twice!"),
            })),
        }
    }
}

pub struct HybridService<Web, Grpc> {
    web: Web,
    grpc: Grpc,
}

impl<Web, Grpc, WebBody, GrpcBody> Service<Request<Body>> for HybridService<Web, Grpc>
where
    Web: Service<Request<Body>, Response = Response<WebBody>>,
    Grpc: Service<Request<Body>, Response = Response<GrpcBody>>,
    Web::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    Grpc::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Response = Response<HybridBody<WebBody, GrpcBody>>;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    type Future = HybridFuture<Web::Future, Grpc::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match self.web.poll_ready(cx) {
            Poll::Ready(Ok(())) => match self.grpc.poll_ready(cx) {
                Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
                Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
                Poll::Pending => Poll::Pending,
            },
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if req.headers().get("content-type").map(|x| x.as_bytes()) == Some(b"application/grpc") {
            HybridFuture::Grpc(self.grpc.call(req))
        } else {
            HybridFuture::Web(self.web.call(req))
        }
    }
}

#[pin_project(project = HybridBodyProj)]
pub enum HybridBody<WebBody, GrpcBody> {
    Web(#[pin] WebBody),
    Grpc(#[pin] GrpcBody),
}

impl<WebBody, GrpcBody> HttpBody for HybridBody<WebBody, GrpcBody>
where
    WebBody: HttpBody + Send + Unpin,
    GrpcBody: HttpBody<Data = WebBody::Data> + Send + Unpin,
    WebBody::Error: std::error::Error + Send + Sync + 'static,
    GrpcBody::Error: std::error::Error + Send + Sync + 'static,
{
    type Data = WebBody::Data;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn is_end_stream(&self) -> bool {
        match self {
            HybridBody::Web(b) => b.is_end_stream(),
            HybridBody::Grpc(b) => b.is_end_stream(),
        }
    }

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        match self.project() {
            HybridBodyProj::Web(b) => b.poll_data(cx).map_err(|e| e.into()),
            HybridBodyProj::Grpc(b) => b.poll_data(cx).map_err(|e| e.into()),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        match self.project() {
            HybridBodyProj::Web(b) => b.poll_trailers(cx).map_err(|e| e.into()),
            HybridBodyProj::Grpc(b) => b.poll_trailers(cx).map_err(|e| e.into()),
        }
    }
}

#[pin_project(project = HybridFutureProj)]
pub enum HybridFuture<WebFuture, GrpcFuture> {
    Web(#[pin] WebFuture),
    Grpc(#[pin] GrpcFuture),
}

impl<WebFuture, GrpcFuture, WebBody, GrpcBody, WebError, GrpcError> Future
    for HybridFuture<WebFuture, GrpcFuture>
where
    WebFuture: Future<Output = Result<Response<WebBody>, WebError>>,
    GrpcFuture: Future<Output = Result<Response<GrpcBody>, GrpcError>>,
    WebError: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    GrpcError: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Output = Result<
        Response<HybridBody<WebBody, GrpcBody>>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    >;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match self.project() {
            HybridFutureProj::Web(a) => match a.poll(cx) {
                Poll::Ready(Ok(res)) => Poll::Ready(Ok(res.map(HybridBody::Web))),
                Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
                Poll::Pending => Poll::Pending,
            },
            HybridFutureProj::Grpc(b) => match b.poll(cx) {
                Poll::Ready(Ok(res)) => Poll::Ready(Ok(res.map(HybridBody::Grpc))),
                Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}
```

## Integrate into our main.rs


Our `app/src/main.rs` now needs to look like this.

```rust
mod api_service;
mod config;
mod errors;
mod fortunes;
mod hybrid;
mod worlds;

use axum::{extract::Extension, response::Html, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = config.create_pool();

    // build our application with a route
    let axum_make_service = Router::new()
        .merge(fortunes::routes())
        .merge(worlds::routes())
        .layer(Extension(config))
        .layer(Extension(pool.clone()))
        .into_make_service();

    let grpc_service = tonic::transport::Server::builder()
        .add_service(crate::api::fortunes_server::FortunesServer::new(
            api_service::FortunesService { pool },
        ))
        .into_service();

    let hybrid_make_service = hybrid::hybrid(axum_make_service, grpc_service);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(hybrid_make_service)
        .await
        .unwrap();
}

pub fn render<F>(f: F) -> Html<&'static str>
where
    F: FnOnce(&mut Vec<u8>) -> Result<(), std::io::Error>,
{
    let mut buf = Vec::new();
    f(&mut buf).expect("Error rendering template");
    let html: String = String::from_utf8_lossy(&buf).into();

    Html(Box::leak(html.into_boxed_str()))
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub mod api {
    tonic::include_proto!("api");
}
```

## BloomRPC

To see our server working we can use [BloomRPC](https://github.com/bloomrpc/bloomrpc) with which we can load our `api.proto` and fire off an RPC call to our fortunes API.

It will look something like the screenshot below.

![BloomRPC](/bloom-rpc.png)