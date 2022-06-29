use crate::errors::CustomError;
use crate::queries;
use axum::{extract::Extension, response::Html};
use deadpool_postgres::Pool;

pub async fn index(Extension(pool): Extension<Pool>) -> Result<Html<&'static str>, CustomError> {
    let client = pool.get().await?;

    let fortunes = queries::fortunes::fortunes(&client).await?;

    Ok(crate::render(|buf| {
        crate::templates::fortunes::index_html(buf, "Fortunes", fortunes)
    }))
}
