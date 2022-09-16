#![allow(warnings, unused)]

mod structs;
mod handlers;
mod extractor;

use axum::extract::extractor_middleware;
use axum::Router;
use axum::routing::{get, post};
use tower_layer::layer_fn;
use tower_http::trace::TraceLayer;
use tracing::warn;
pub use structs::{MyResponse};
use crate::extractor::UserAgentInfo;
use crate::handlers::{json, my_response, my_response_input, plain_text};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json))
        .route("/my", get(my_response))
        .route("/my/:head/:tail", get(my_response_input))
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_extractor::<UserAgentInfo>())
        ;

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}