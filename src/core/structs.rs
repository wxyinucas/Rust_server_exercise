use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct MyResponse {
    pub status: u32,
    pub message: String,
    pub container: Vec<Pair>,
}

impl Default for MyResponse {
    fn default() -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            message: String::new(),
            container: vec![],
        }
    }
}

impl IntoResponse for MyResponse {
    // TODO 注意Response的结构
    fn into_response(self) -> Response {
        (Json(json!(self))).into_response()
    }
}

impl From<&str> for MyResponse {
    fn from(s: &str) -> Self {
        Self {
            message: s.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pair {
    pub key: String,
    pub value: String,
}

impl Pair {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
