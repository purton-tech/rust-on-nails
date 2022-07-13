use crate::errors::CustomError;
use axum::response::Html;

pub static INDEX: &str = "/";

pub async fn index() -> Result<Html<&'static str>, CustomError> {
    Ok(crate::render(|buf| {
        crate::templates::dashboard::index_html(buf)
    }))
}