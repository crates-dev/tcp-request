use crate::*;

#[derive(Debug, Clone)]
pub struct RequestBuilder {
    pub(crate) tcp_request: TcpRequest,
    pub(crate) builder: TcpRequest,
}
