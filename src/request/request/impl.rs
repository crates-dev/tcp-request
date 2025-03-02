use crate::*;

impl TcpRequest {
    #[inline]
    fn send_request(
        &mut self,
        stream: &mut TcpStream,
        data: &[u8],
    ) -> Result<BoxResponseTrait, Error> {
        stream.write_all(data).and_then(|_| stream.flush()).unwrap();
        self.read_response(stream)
    }

    #[inline]
    fn read_response(&mut self, stream: &mut TcpStream) -> Result<BoxResponseTrait, Error> {
        let cfg_buffer_size: usize = self
            .get_config()
            .read()
            .map_or(DEFAULT_BUFFER_SIZE, |data| data.buffer_size);
        let mut tmp_buf: Vec<u8> = vec![0u8; cfg_buffer_size];
        let mut response_bytes: Vec<u8> = Vec::with_capacity(cfg_buffer_size);
        while let Ok(n) = stream.read(&mut tmp_buf) {
            if n == 0 {
                break;
            }
            response_bytes.extend_from_slice(&tmp_buf[..n]);
        }
        self.response = Arc::new(RwLock::new(<TcpResponseBinary as ResponseTrait>::from(
            &response_bytes,
        )));
        return Ok(Box::new(
            self.response.read().map_or(Vec::new(), |data| data.clone()),
        ));
    }

    #[inline]
    fn get_connection_stream(&self, host: String, port: usize) -> Result<TcpStream, Error> {
        let host_port: (String, u16) = (host.clone(), port as u16);
        let cfg_timeout: u64 = self
            .get_config()
            .read()
            .map_or(DEFAULT_TIMEOUT, |data| data.timeout);
        let timeout: Duration = Duration::from_millis(cfg_timeout);
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
    fn send(&mut self, data: &[u8]) -> Self::RequestResult {
        let cfg_timeout: Config = self
            .get_config()
            .read()
            .map_or(Config::default(), |data| data.clone());
        let host: String = cfg_timeout.get_host().clone();
        let port: usize = cfg_timeout.get_port().clone();
        let mut stream: TcpStream = self
            .get_connection_stream(host, port)
            .map_err(|_| Error::TcpStreamConnectError)?;
        let res: Result<BoxResponseTrait, Error> = self.send_request(&mut stream, data);
        res
    }
}

impl Default for TcpRequest {
    #[inline]
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(Config::default())),
            response: Arc::new(RwLock::new(TcpResponseBinary::default())),
        }
    }
}
