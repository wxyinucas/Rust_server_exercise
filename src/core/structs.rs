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

impl MyResponse {
    pub fn new(head: String) -> Self {
        Self {
            head,
            tail: "I am tail".to_string(),
        }
    }
}

impl IntoResponse for MyResponse {
    // TODO 注意Response的结构
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(json!(self))).into_response()
    }
}

impl From<&str> for MyResponse {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}