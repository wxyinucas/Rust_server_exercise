use std::convert::Infallible;
use bytes::Bytes;
use hyper::server::conn::Http;
use hyper::service::{make_service_fn, Service};
use hyper::{Body, Request, Response};
use tokio::net::TcpListener;

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use http_body::Full;
use tower_http::follow_redirect::policy::PolicyExt;

type Counter = i32;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on http://{}", addr);

    let make_svc = make_service_fn(|_conn| async { // TODO 如何使用Struct！
        Ok::<_, Infallible>(HelloWorld)
    });

    axum::Server::bind(&addr).serve(make_svc).await.unwrap();
}


struct HelloWorld;

impl HelloWorld {
    fn new() -> Self {
        Self
    }
}

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // create the body
        let body = Full::new(Bytes::from("Hello World"));
        // Create the HTTP response
        let resp = Response::builder()
            .status(http::StatusCode::OK)
            .body(body)
            .expect("Unable to create `http::Response`");

        // create a response in a future.
        let fut = async {
            Ok(resp)
        };

        // Return the response as an immediate future
        Box::pin(fut)
    }
}

// Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>