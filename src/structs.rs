use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct MyResponse {
    pub head: String,
    // pub middle: String,
    pub tail: String,
}

impl IntoResponse for MyResponse { // TODO 注意结构
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(json!(self))).into_response()
    }
}