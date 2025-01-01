use crate::request::request::r#type::TcpRequest;

#[derive(Debug, Clone)]
pub struct RequestBuilder {
    pub(crate) tcp_request: TcpRequest,
    pub(crate) builder: TcpRequest,
}
