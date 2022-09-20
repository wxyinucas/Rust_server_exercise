use std::thread::sleep;
use std::time::Duration;
use axum::extract::Path;
use axum::Json;
use axum::response::IntoResponse;
use http::{Response, StatusCode};
use crate::{MyResponse};
use axum_macros::debug_handler;
use serde_json::{json, Value};
use tracing::warn;


pub async fn plain_text() -> &'static str {
    "foo"
}

pub async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

// 暂时不需要，应为将核心功能转移到MyResponse的impl中了。
// pub async fn my_response() -> impl IntoResponse { // TODO 注意这里
//     let my = MyResponse { head: "head value".to_string(), tail: "tail value".to_string() };
//
//     (StatusCode::OK, Json(json!(my))) // TODO 注意这里
// }


pub async fn my_response_input(Path((head, tail)): Path<(String, String)>) -> MyResponse {
    // 《在axum中获取请求数据》
    // TODO 注意input

    sleep(Duration::from_secs(5));
    warn!("{}", head);
    MyResponse { head, tail }
}