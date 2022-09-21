use axum::body::HttpBody;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, IntoMakeService};
use axum::Router;
use bytes::Bytes;
use http::Request;
use http_body::Full;
use hyper::service::make_service_fn;
use hyper::Body;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::fmt::{write, Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;
use tracing::warn;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(plain_text));


    let make_svc = make_service_fn(|_conn| async move{
        // TODO 如何使用Struct！还有router的make_into_service
        let hello = HelloWorld::new(app);
        Ok::<_, Infallible>(hello)
    });

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        // .serve(hello.into_make_service())
        .serve(make_svc)
        .await
        .unwrap();
}

struct HelloWorld<S> {
    inner_service: S,
}

impl<S> HelloWorld<S> {
    // fn into_make_service(self) -> IntoMakeService<Self> {
    //     IntoMakeService::new(self)
    // }

    fn new(router: S) -> Self {
        Self {
            inner_service: router,
        }
    }
}

struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hello_world error")
    }
}

impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "hello_world error")
    }
}

impl Error for MyError {}

impl<B, S> Service<Request<B>> for HelloWorld<S>
where
    S: Service<Request<B>>,
    <<S as Service<Request<B>>>::Future as Future>::Output: fmt::Debug,
    S::Error: Into<Box<dyn Error + Send + Sync>> + 'static,
    B: Debug,
    B: HttpBody + Send + 'static,
{
    type Response = Response<Full<Bytes>>;
    type Error = Box<dyn Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner_service.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        warn!("req = {:?}", req.body());
        let fut = async move {
            let res = self.inner_service.call(req).await;
            warn!("res = {:?}", res);

            Ok(Response::builder()
                .status(http::StatusCode::OK)
                .body(Full::new(Bytes::from("Hello, world!")))
                .map_err(|err| Box::new(MyError) as Box<dyn Error + Send + Sync>)
                .expect("TODO: panic message"))
        };

        Box::pin(fut)
    }
}

async fn plain_text() -> &'static str {
    "foo"
}
