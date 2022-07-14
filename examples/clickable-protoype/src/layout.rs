use crate::errors::CustomError;
use axum::response::Response;
use hyper::{Body, StatusCode};

#[derive(PartialEq, Eq)]
pub enum SideBar {
    Dashboard,
    Crud,
    Team,
    NewTeam,
    ApiKeys,
    Subscriptions,
    None
}

pub fn get_menu_class(side_bar: &SideBar, selected_sidebar: &SideBar, sub_menu: bool) -> String {
    let selected = selected_sidebar == side_bar;
    match (selected, sub_menu) {
        (true, true) => "selected submenu",
        (true, false) => "selected",
        (false, true) => "submenu",
        (false, false) => "",
    }
    .to_string()
}

pub fn redirect_and_snackbar(
    url: &str,
    message: &'static str,
) -> Result<Response<Body>, CustomError> {
    let builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("location", url)
        .header("set-cookie", format!("flash_aargh={}; Max-Age=6", message))
        .body(Body::empty());
    let response =
        builder.map_err(|_| CustomError::FaultySetup("Could not build redirect".to_string()))?;
    Ok(response)
}