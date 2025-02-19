#[cfg(test)]
mod cfg;
pub(crate) mod common;
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

pub(crate) use common::r#type::*;
pub(crate) use http_type::{
    DEFAULT_BUFFER_SIZE, DEFAULT_TIMEOUT, DEFAULT_WEB_PORT, EMPTY_STR, HTTP_DOUBLE_BR_BYTES,
};
pub(crate) use lombok_macros::*;
pub(crate) use request::{
    config::r#type::*, error::r#type::Error, r#type::RequestResult, request::r#type::*,
};
pub(crate) use std::{
    error::Error as StdError,
    fmt::Debug,
    fmt::{self, Display},
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, RwLock},
    time::Duration,
};
