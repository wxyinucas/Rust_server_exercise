use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use axum::response::{IntoResponse, Response};
use tower_layer::Layer;
use tower_service::Service;
use tracing::warn;
use crate::MyResponse;

pub struct PlainLayer {
    target: &'static str,
}

impl PlainLayer {
    pub fn new(target: &'static str) -> Self {
        Self { target }
    }
}

impl<S> Layer<S> for PlainLayer {
    type Service = PlainService<S>;

    fn layer(&self, service: S) -> Self::Service {
        PlainService {
            target: self.target,
            service,
        }
    }
}

// This service implements the Log behavior
#[derive(Clone, Debug)]
pub struct PlainService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for PlainService<S>
    where
        S: Service<Request>,
        S::Response: fmt::Debug,
        Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        warn!("request = {:?}, added info = {:?}", request, self.target);
        self.service.call(request)
    }
}