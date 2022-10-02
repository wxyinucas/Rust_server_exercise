use crate::{MyError, MyResponse};
use axum::body::HttpBody;
use axum::response::{IntoResponse, Response};
use axum::Router;
use http::Request;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;
use tracing::{error, warn};

#[derive(Debug)] // todo why clone?
pub struct MyService<B> {
    router: Router<B>,
}

impl<B> MyService<B> {
    pub fn from_router(router: Router<B>) -> Self {
        Self { router }
    }

    // pub fn make_into_service()
}

impl<B> Clone for MyService<B> {
    // TODO how to make MyService clone
    fn clone(&self) -> Self {
        Self {
            router: self.router.clone(),
        }
    }
}

impl<B> Service<Request<B>> for MyService<B>
where
    B: HttpBody + Send + 'static, // todo why these three constrains?
    B: Debug,
{
    type Response = Response;
    type Error = MyError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>; // todo why send here?

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.router.poll_ready(cx).map_err(|err| err.into())
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        warn!("request = {:?}", req.body());

        let mut this = self.clone();
        let fut = this.router.call(req);

        let f = async move {
            let tmp = fut.await;
            error!("Router's result's body = {:?}", tmp.expect("should have body").into_body().await);
            let res = MyResponse::from("test string").into_response();
            error!("After operation, new response = {:?}", res);
            Ok(res)
        };

        Box::pin(f)
    }
}
