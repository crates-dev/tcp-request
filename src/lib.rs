#[cfg(test)]
mod cfg;
pub(crate) mod constant;
pub(crate) mod request;
pub(crate) mod response;

pub use request::{
    error::r#type::Error as RequestError, r#trait::RequestTrait, r#type::BoxRequestTrait,
    request_builder::r#type::RequestBuilder,
};
pub use response::{
    r#trait::ResponseTrait, r#type::BoxResponseTrait, response_binary::r#type::TcpResponseBinary,
    response_text::r#type::TcpResponseText,
};
