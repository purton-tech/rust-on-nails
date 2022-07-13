use crate::errors::CustomError;
use axum::response::Html;
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/crud";
pub static FILLED: &str = "/app/crud/not-empty";
pub static NEW: &str = "/app/crud/new";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(empty))
        .route(NEW, post(new))
        .route(FILLED, get(index))
}

pub async fn index() -> Result<Html<&'static str>, CustomError> {
    Ok(crate::render(|buf| crate::templates::crud::index_html(buf)))
}

pub async fn empty() -> Result<Html<&'static str>, CustomError> {
    Ok(crate::render(|buf| crate::templates::crud::empty_html(buf)))
}
pub async fn new() -> Result<impl IntoResponse, CustomError> {
    crate::layout::redirect_and_snackbar(FILLED, "Vault Created")
}
