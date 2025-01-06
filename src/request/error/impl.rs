use super::r#type::Error;
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

impl StdError for Error {}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
            Error::TcpStreamConnectError => write!(f, "TCP Stream Connection Error"),
            Error::RequestError => write!(f, "Request Error"),
            Error::ReadConnectionError => write!(f, "Connection Read Error"),
            Error::SetReadTimeoutError => write!(f, "Failed to Set Read Timeout"),
            Error::SetWriteTimeoutError => write!(f, "Failed to Set Write Timeout"),
        }
    }
}
