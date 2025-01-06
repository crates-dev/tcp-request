use super::r#type::TcpRequest;
use crate::request::r#trait::RequestTrait;
use crate::{
    request::{config::r#type::Config, error::r#type::Error, r#type::RequestResult},
    response::{
        r#trait::ResponseTrait, r#type::BoxResponseTrait,
        response_binary::r#type::TcpResponseBinary,
    },
};
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

impl TcpRequest {
    #[inline]
    fn send_request(&mut self, stream: &mut TcpStream) -> Result<BoxResponseTrait, Error> {
        stream
            .write_all(&self.data)
            .and_then(|_| stream.flush())
            .unwrap();
        self.read_response(stream)
    }

    #[inline]
    fn read_response(&mut self, stream: &mut TcpStream) -> Result<BoxResponseTrait, Error> {
        let mut tmp_buf: Vec<u8> = vec![0u8; self.get_config().get_buffer_size().clone()];
        let mut response_bytes: Vec<u8> = Vec::new();
        while let Ok(n) = stream.read(&mut tmp_buf) {
            if n == 0 {
                break;
            }
            response_bytes.extend_from_slice(&tmp_buf[..n]);
        }
        self.response = <TcpResponseBinary as ResponseTrait>::from(&response_bytes);
        return Ok(Box::new(self.response.clone()));
    }

    #[inline]
    fn get_connection_stream(&self, host: String, port: usize) -> Result<TcpStream, Error> {
        let host_port: (String, u16) = (host.clone(), port as u16);
        let timeout: Duration = Duration::from_millis(self.config.timeout);
        let tcp_stream: TcpStream =
            TcpStream::connect(host_port.clone()).map_err(|_| Error::TcpStreamConnectError)?;
        tcp_stream
            .set_read_timeout(Some(timeout))
            .map_err(|_| Error::SetReadTimeoutError)?;
        tcp_stream
            .set_write_timeout(Some(timeout))
            .map_err(|_| Error::SetWriteTimeoutError)?;
        let stream: Result<TcpStream, Error> = Ok(tcp_stream);
        stream
    }
}

impl RequestTrait for TcpRequest {
    type RequestResult = RequestResult;

    #[inline]
    fn send(&mut self) -> Self::RequestResult {
        let host: String = self.config.host.clone();
        let port: usize = self.get_config().get_port().clone();
        let mut stream: TcpStream = self
            .get_connection_stream(host, port)
            .map_err(|_| Error::TcpStreamConnectError)?;
        let res: Result<BoxResponseTrait, Error> = self.send_request(&mut stream);
        res
    }
}

impl Default for TcpRequest {
    #[inline]
    fn default() -> Self {
        Self {
            config: Config::default(),
            response: TcpResponseBinary::default(),
            data: Vec::new(),
        }
    }
}
