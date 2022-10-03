# Rust_server_exercise

A toy server to learn the traits and structs in Axum, Tower and Hyber.

The struct `hyper::Server`, the trait `tower::{Service, Layer}` are core components in axum.
This project wants to write my own `Service` and `Layer` from scratch, and make it usable for `Server`.
Furthermore, I want to add a layer to modify the response produced by `Router`, and add more functions.

我们一方面希望使用axum的路由功能传入不同的命令，另一方面我们又希望用一个结构封装执行命令的过程，避免修改每一个`handler function`。
为了实现这个目的，我们使用了`MyService<R>`来包裹 `Router`，将作为中间产物的`MyResponse`重新封装为`axum::response::Response`，从而完成了抽象过程。

下一步，可以更新future的定义，以进一步提高效率。

References:

- [Inventing the Service trait](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait#the-handler-trait)
- [Building a middleware from scratch](https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md)
- [hyper guides](https://hyper.rs/guides)