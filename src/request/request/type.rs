use crate::{request::config::r#type::Config, TcpResponseBinary};
use lombok_macros::*;

#[derive(Debug, Clone, Lombok)]
pub struct TcpRequest {
    pub(crate) data: Vec<u8>,
    pub(crate) config: Config,
    pub(crate) response: TcpResponseBinary,
}
