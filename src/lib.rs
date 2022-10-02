mod core;
mod services;

pub use crate::core::handlers::{json_handler, my_response_with_input, plain_text};
pub use crate::core::{errors::MyError, structs::MyResponse};
pub use crate::services::middlewares::modify_res_layer::ModifyLayer;
pub use services::service_wrapper::MyService;
