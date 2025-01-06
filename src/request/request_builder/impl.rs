use http_type::HTTP_DOUBLE_BR_BYTES;

use super::r#type::RequestBuilder;
use crate::request::{r#type::BoxRequestTrait, request::r#type::TcpRequest};

impl Default for RequestBuilder {
    #[inline]
    fn default() -> Self {
        Self {
            tcp_request: TcpRequest::default(),
            builder: TcpRequest::default(),
        }
    }
}

impl RequestBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn host(&mut self, host: &str) -> &mut Self {
        self.tcp_request.config.host = host.to_owned();
        self
    }

    #[inline]
    pub fn port(&mut self, port: usize) -> &mut Self {
        self.tcp_request.config.port = port;
        self
    }

    #[inline]
    pub fn data<T>(&mut self, data: T) -> &mut Self
    where
        T: Into<Vec<u8>>,
    {
        let mut data_clone: Vec<u8> = data.into();
        data_clone.extend_from_slice(HTTP_DOUBLE_BR_BYTES);
        self.tcp_request.data = data_clone;
        self
    }

    #[inline]
    pub fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        self.tcp_request.config.buffer_size = buffer_size;
        self
    }

    #[inline]
    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        self.tcp_request.config.timeout = timeout;
        self
    }

    #[inline]
    pub fn build(&mut self) -> BoxRequestTrait {
        self.builder = self.tcp_request.clone();
        self.tcp_request = TcpRequest::default();
        Box::new(self.builder.clone())
    }
}
