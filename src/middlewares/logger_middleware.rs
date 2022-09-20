use std::convert::Infallible;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use http::Request;
use tower_layer::Layer;
use tower_service::Service;
use tracing::{debug, error, warn};
use crate::MyResponse;

pub struct LogLayer {
    pub(crate) target: &'static str,
}

impl LogLayer{
    pub fn new(target: &'static str) -> Self {
        Self { target }
    }
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService {
            target: self.target,
            service
        }
    }
}

// This service implements the Log behavior
#[derive(Clone, Debug)]
pub struct LogService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
    where
        S: Service<Request>,
        Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        debug!("Hello World!"); // TODO 搞不懂
        warn!("request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}