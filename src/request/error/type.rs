#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    TcpStreamConnectError,
    RequestError,
    ReadConnectionError,
    SetReadTimeoutError,
    SetWriteTimeoutError,
    ReadResponseError,
}
