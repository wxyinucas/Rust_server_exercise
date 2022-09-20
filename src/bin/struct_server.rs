use bytes::Bytes;
use hyper::server::conn::Http;
use hyper::service::Service;
use hyper::{Body, Request, Response};
use tokio::net::TcpListener;

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use http_body::Full;

type Counter = i32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        let hello_world = HelloWorld;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                // .serve_connection(stream, Svc { counter: 81818 })
                .serve_connection(stream, hello_world)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

struct Svc {
    counter: Counter,
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        let res = match req.uri().path() {
            "/" => mk_response(format!("home! counter = {:?}", self.counter)),
            "/posts" => mk_response(format!("posts, of course! counter = {:?}", self.counter)),
            "/authors" => mk_response(format!(
                "authors extraordinare! counter = {:?}",
                self.counter
            )),
            // Return the 404 Not Found for other routes, and don't increment counter.
            _ => return Box::pin(async { mk_response("oh no! not found".into()) }),
        };

        if req.uri().path() != "/favicon.ico" {
            self.counter += 1;
        }

        Box::pin(async { res })
    }
}


struct HelloWorld;

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>; // Todo 加了Send就没错了

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