mod index;

use axum::{routing::get, Router};

pub static INDEX: &str = "/fortunes";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index::index))
}
