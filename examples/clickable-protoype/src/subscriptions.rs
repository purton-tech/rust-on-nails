use crate::errors::CustomError;
use axum::{response::Html, routing::get, Router};

pub static INDEX: &str = "/app/subscriptions";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index))
}

pub async fn index() -> Result<Html<&'static str>, CustomError> {
    Ok(crate::render(|buf| {
        crate::templates::subscriptions::index_html(buf)
    }))
}
