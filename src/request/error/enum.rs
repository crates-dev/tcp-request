#[derive(Debug)]
pub enum RequestError {
    InvalidUrl,
    TcpStreamConnectError,
    RequestError,
    ReadConnectionError,
    SetReadTimeoutError,
    SetWriteTimeoutError,
    ReadResponseError,
}
