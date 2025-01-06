mod layout;
pub mod root;
pub mod settings;
use dioxus::prelude::*;

pub fn render(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}
