use crate::*;

/// Configuration for TCP request settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Config {
    /// Target host address.
    pub(crate) host: String,
    /// Target port number.
    pub(crate) port: usize,
    /// Request timeout in seconds.
    pub(crate) timeout: u64,
    /// Buffer size for data transfer.
    pub(crate) buffer_size: usize,
}
