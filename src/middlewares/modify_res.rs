use crate::MyResponse;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use http::Request;
use http_body::Full;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;
use tracing::warn;

pub struct ModifyLayer {
    target: &'static str,
}

impl ModifyLayer {
    pub fn new(target: &'static str) -> Self {
        Self { target }
    }
}

impl<S> Layer<S> for ModifyLayer {
    type Service = ModifyService<S>;

    fn layer(&self, service: S) -> Self::Service {
        ModifyService {
            target: self.target,
            service,
        }
    }
}

// This service implements the Log behavior
#[derive(Clone, Debug)]
pub struct ModifyService<S> {
    target: &'static str,
    service: S,
}

impl<S, B> Service<Request<B>> for ModifyService<S>
where
    S: Service<Request<B>>,
    S: Clone,
    <S as Service<Request<B>>>::Future: std::marker::Send, // todo why send
    <S as Service<Request<B>>>::Future: 'static, //todo why 'static?
    B: fmt::Debug,
    S::Response: fmt::Debug,
{
    type Response = Response<Full<Bytes>>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let mut this = self.clone();
        warn!(
            "request = {:?}, added info = {:?}",
            request.body(),
            this.target
        );
        let fut = this.service.call(request);

        let f = async move {
            // TODO using this to solve lifetime problems
            let res = fut.await?;
            warn!("response = {:?}", res);
            Ok(Response::builder()
                .body(Full::new(Bytes::from(this.target)))
                .expect("error"))
        };

        Box::pin(f)
    }
}
