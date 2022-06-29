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
        .accept_http1(true)
        .add_service(tonic_web::enable(crate::api::fortunes_server::FortunesServer::new(
            api_service::FortunesService { pool },
        )))
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
