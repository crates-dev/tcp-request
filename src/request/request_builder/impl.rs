use crate::*;

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
    pub fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        let _ = self
            .tcp_request
            .get_mut_config()
            .write()
            .and_then(|mut data| {
                data.host = host.into();
                Ok(())
            });
        self
    }

    #[inline]
    pub fn port(&mut self, port: usize) -> &mut Self {
        let _ = self
            .tcp_request
            .get_mut_config()
            .write()
            .and_then(|mut data| {
                data.port = port;
                Ok(())
            });
        self
    }

    #[inline]
    pub fn data<T>(&mut self, data: T) -> &mut Self
    where
        T: Into<Vec<u8>>,
    {
        let mut data_clone: Vec<u8> = data.into();
        data_clone.extend_from_slice(HTTP_DOUBLE_BR_BYTES);
        self.tcp_request.data = Arc::new(RwLock::new(data_clone));
        self
    }

    #[inline]
    pub fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        let _ = self
            .tcp_request
            .get_mut_config()
            .write()
            .and_then(|mut data| {
                data.buffer_size = buffer_size;
                Ok(())
            });
        self
    }

    #[inline]
    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        let _ = self
            .tcp_request
            .get_mut_config()
            .write()
            .and_then(|mut data| {
                data.timeout = timeout;
                Ok(())
            });
        self
    }

    #[inline]
    pub fn build(&mut self) -> BoxRequestTrait {
        self.builder = self.tcp_request.clone();
        self.tcp_request = TcpRequest::default();
        Box::new(self.builder.clone())
    }
}
