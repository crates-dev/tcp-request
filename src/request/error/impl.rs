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
            Self::InvalidUrl => write!(f, "Invalid URL"),
            Self::TcpStreamConnectError => write!(f, "TCP Stream Connection Error"),
            Self::RequestError => write!(f, "Request Error"),
            Self::ReadConnectionError => write!(f, "Connection Read Error"),
            Self::SetReadTimeoutError => write!(f, "Failed to Set Read Timeout"),
            Self::SetWriteTimeoutError => write!(f, "Failed to Set Write Timeout"),
            Self::ReadResponseError => write!(f, "Read response error"),
        }
    }
}
