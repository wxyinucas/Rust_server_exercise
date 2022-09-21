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
use pin_project::pin_project;
use tower_http::follow_redirect::policy::PolicyExt;
use tracing::warn;

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
    type Future = MyFuture<dyn Future<Output=Result<Self::Response, Self::Error>>>;

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
        MyFuture { inner_future: fut }
    }
}

// Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>

#[pin_project]
struct MyFuture<F> {  // todo failed
    #[pin]
    inner_future: F,

}

impl<F, Response, Error> Future for MyFuture<F>
    where F: Future<Output=Result<Response, Error>> {
    type Output = Result<Response, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.inner_future.poll(cx) {
            Poll::Ready(Ok(resp)) => {
                warn!("{:?}", resp);
                return Poll::Ready(Ok(resp));
            }
            Poll::Pending => Poll::Pending
        }
    }
}
