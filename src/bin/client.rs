use hyper::{Body, Method, Client, Request};
use tokio::{join, try_join};
use tracing::warn;
use futures::future::join_all;

#[tokio::main]
async fn main() {  // TODO:用于将来的连接测试
    let client = Client::new();
    tracing_subscriber::fmt::init();

    let mut futures = vec![];

    for _i in 0..10 {
        let mut req = Request::builder()
            .method(Method::GET)
            // .uri(format!("http://127.0.0.1:3000/head/{}", i))
            .uri(format!("http://127.0.0.1:3000/my/head/11"))
            .body(Body::from("Hallo!"))
            .expect("request builder");

        let headers = req.headers_mut();
        headers.insert(http::header::USER_AGENT, "Firefox".parse().unwrap());


        let future = client.request(req);
        futures.push(future);
    }

    // try_join!(futures.pop().unwrap(), futures.pop().unwrap(), futures.pop().unwrap(), futures.pop().unwrap());
    join_all(futures).await;
}