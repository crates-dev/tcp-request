use crate::{request::config::r#type::Config, TcpResponseBinary};
use http_type::ArcRwLock;
use lombok_macros::*;

#[derive(Debug, Clone, Lombok)]
pub struct TcpRequest {
    pub(crate) data: ArcRwLock<Vec<u8>>,
    pub(crate) config: ArcRwLock<Config>,
    pub(crate) response: ArcRwLock<TcpResponseBinary>,
}
