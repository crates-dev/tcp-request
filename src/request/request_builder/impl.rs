use crate::*;

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            tcp_request: TcpRequest::default(),
            builder: TcpRequest::default(),
        }
    }
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        let _ = self.tcp_request.config.write().and_then(|mut data| {
            data.host = host.into();
            Ok(())
        });
        self
    }

    pub fn port(&mut self, port: usize) -> &mut Self {
        let _ = self.tcp_request.config.write().and_then(|mut data| {
            data.port = port;
            Ok(())
        });
        self
    }

    pub fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        let _ = self.tcp_request.config.write().and_then(|mut data| {
            data.buffer_size = buffer_size;
            Ok(())
        });
        self
    }

    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        let _ = self.tcp_request.config.write().and_then(|mut data| {
            data.timeout = timeout;
            Ok(())
        });
        self
    }

    pub fn build(&mut self) -> BoxRequestTrait {
        self.builder = self.tcp_request.clone();
        self.tcp_request = TcpRequest::default();
        Box::new(self.builder.clone())
    }
}
