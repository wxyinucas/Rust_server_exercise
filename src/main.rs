#![allow(warnings, unused)]

mod handlers;
mod core;
mod services;

use crate::handlers::{json, my_response_input, plain_text};
use axum::routing::{get, post};
use axum::Router;
pub use structs::MyResponse;
use tower_http::trace::TraceLayer;
use tower_layer::layer_fn;
use tracing::warn;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "test_server=debug,tower_http=debug");
    }
    tracing_subscriber::fmt().init();

    // let app = Router::new()
    //     .route("/plain_text", get(plain_text))
    //     .route("/json", get(json))
    //     .route("/my/:head/:tail", get(my_response_input))
    //     .layer(TraceLayer::new_for_http())
    //     .layer(ModifyLayer::new("wxy"));
    //
    // axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
