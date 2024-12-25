mod config;
mod errors;
mod root;
mod static_files;

use std::net::SocketAddr;
use tower_http::services::ServeDir;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(root::loader))
        .route("/new_user", post(root::new_user_action))
        .route("/static/*path", get(static_files::static_path))
        .nest_service("/wasm", ServeDir::new("/workspace/crates/web-csr/dist"))
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
