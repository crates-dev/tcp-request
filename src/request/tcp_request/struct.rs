use crate::*;

/// TCP request structure containing configuration and response data.
#[derive(Debug, Clone)]
pub struct TcpRequest {
    /// Thread-safe configuration for the request.
    pub(crate) config: ArcRwLock<Config>,
    /// Thread-safe binary response storage.
    pub(crate) response: ArcRwLock<TcpResponseBinary>,
}
