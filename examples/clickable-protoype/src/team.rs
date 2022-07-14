use crate::errors::CustomError;
use axum::{response::Html, routing::get, Router};

pub static INDEX: &str = "/app/team";
pub static SWITCH: &str = "/app/team/switch";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index))
        .route(SWITCH, get(switch))
}

pub async fn index() -> Result<Html<&'static str>, CustomError> {
    Ok(crate::render(|buf| crate::templates::team::index_html(buf)))
}

pub async fn switch() -> Result<Html<&'static str>, CustomError> {
    Ok(crate::render(|buf| crate::templates::team::switch_html(buf)))
}
