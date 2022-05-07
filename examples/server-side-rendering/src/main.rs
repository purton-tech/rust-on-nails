mod authentication;
mod config;
pub mod cornucopia;
mod error;

use axum::body::{self, Empty, Body};
use axum::extract::{Extension, Path};
use axum::http::{Response, StatusCode, header, HeaderValue};
use axum::response::IntoResponse;
use axum::{response::Html, routing::get, Router};
use cornucopia::queries::users;
use deadpool_postgres::{Config, Pool, Runtime};
use std::net::SocketAddr;
use tokio_postgres::NoTls;
use crate::templates::statics::StaticFile;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("db"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("postgres"));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        //.merge(statics::asset_pipeline_routes())
        //.merge(statics::image_routes())
        .route("/static/*path", get(static_path))
        .layer(Extension(pool))
        .layer(Extension(config));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(body::boxed(Body::from(data.content)))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap()
    }
}

async fn handler(Extension(pool): Extension<Pool>) -> Result<Html<String>, error::CustomError> {
    let client = pool.get().await?;
    let users = users::example_query(&client, &10).await?;
    
    let mut buf = Vec::new();
    crate::templates::vaults::index_html(&mut buf, "Your Vaults").unwrap();
    
    let html = format!("{}", String::from_utf8_lossy(&buf));

    Ok(Html(html))
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
