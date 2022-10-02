use crate::MyResponse;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use axum_macros::debug_handler;
use http::{Response, StatusCode};
use serde_json::{json, Value};
use std::thread::sleep;
use std::time::Duration;
use tracing::warn;

pub async fn plain_text() -> &'static str {
    "foo"
}

pub async fn json_handler() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

// 暂时不需要，应为将核心功能转移到MyResponse的impl中了。
// pub async fn my_response() -> impl IntoResponse { // TODO 注意这里
//     let my = MyResponse { head: "head value".to_string(), tail: "tail value".to_string() };
//
//     (StatusCode::OK, Json(json!(my))) // TODO 注意这里
// }

pub async fn my_response_with_input(Path((head, tail)): Path<(String, String)>) -> MyResponse {
    // 《在axum中获取请求数据》
    // TODO 注意input中Path以及其他解包方式

    sleep(Duration::from_secs(1));
    warn!("{}", head);
    (format!("{}-{}", head, tail).as_str()).into()
}
