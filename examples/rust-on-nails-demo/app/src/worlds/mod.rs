mod db;

use axum::{routing::get, Router};

pub static JSON: &str = "/json";

pub fn routes() -> Router {
    Router::new()
        .route(JSON, get(db::db))
}
