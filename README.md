# Rust_server_exercise

A toy server to learn the traits and structs in Axum, Tower and Hyber.

The struct `hyper::Server`, the trait `tower::{Service, Layer}` are core components in axum.
This project wants to write my own `Service` and `Layer` from scratch, and make it usable for `Server`.
Furthermore, I want to add a layer to modify the response produced by `Router`, and add more functions.

References:

- [Inventing the Service trait](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait#the-handler-trait)
- [Building a middleware from scratch](https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md)
- [hyper guides](https://hyper.rs/guides)