#![allow(warnings, unused)]

mod structs;
mod handlers;
mod extractor;
mod middleware;

use axum::Router;
use axum::routing::{get, post};
use tower_layer::layer_fn;
use tower_http::trace::TraceLayer;
use tracing::warn;
pub use structs::{MyResponse};
use crate::extractor::UserAgentInfo;
use crate::handlers::{json,  my_response_input, plain_text};
use crate::middleware::LogLayer;


#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "test_server=debug,tower_http=debug");
    }
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json))
        // .route("/my", get(my_response))
        .route("/my/:head/:tail", get(my_response_input))
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_extractor::<UserAgentInfo>())
        .layer(LogLayer::new("jkj"))
        ;

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}