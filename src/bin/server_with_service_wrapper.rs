#![allow(warnings, unused)]

use axum::routing::{get, post};
use axum::Router;
use hyper::service::make_service_fn;
use std::convert::Infallible;
use test_server::{json_handler, my_response_with_input, plain_text, ModifyLayer, MyService};
use tower_http::trace::TraceLayer;
use tower_layer::layer_fn;
use tracing::warn;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "test_server=debug,tower_http=debug");
    }
    tracing_subscriber::fmt().init();

    let make_svc = make_service_fn(|_conn| async move {
        let app = Router::new()
            .route("/plain_text", get(plain_text))
            .route("/json", get(json_handler))
            .route("/my/:head/:tail", get(my_response_with_input));
        let service = MyService::from_router(app);
        Ok::<_, Infallible>(service)
    });

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(make_svc)
        .await
        .unwrap();
}
