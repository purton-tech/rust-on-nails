use crate::errors::CustomError;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use axum_extra::extract::Form;
use serde::Deserialize;
use validator::Validate;
use web_pages::settings;

pub async fn loader(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    let html = settings::index(users);

    Ok(Html(html))
}
