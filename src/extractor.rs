use axum::extract::{FromRequest, RequestParts};
use http::StatusCode;
use tower::ServiceExt;
use async_trait::async_trait;

pub struct UserAgentInfo;

#[async_trait]
impl<B> FromRequest<B> for UserAgentInfo
    where
        B: Send,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let user_agent = req
            .headers()
            .get(axum::http::header::USER_AGENT)
            .and_then(|value| value.to_str().ok()).unwrap()
            ;
        tracing::debug!("该用户UserAgent是：{:?}", user_agent);
        if !user_agent.contains("Firefox") {
            tracing::error!("非Firefox浏览器，禁止访问");
            return Err((
                StatusCode::BAD_REQUEST,
                "You MUST use Firefox to visit this page.".to_string(),
            ));
        }
        Ok(UserAgentInfo {})
    }
}