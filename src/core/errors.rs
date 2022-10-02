use std::convert::Infallible;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Something not found:(0)")]
    NotFound(String),

    #[error("Should not be here")]
    FromInfallible(#[from] Infallible),
}
