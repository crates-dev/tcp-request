use crate::*;

#[derive(Debug, Clone)]
pub struct TcpRequest {
    pub(crate) config: ArcRwLock<Config>,
    pub(crate) response: ArcRwLock<TcpResponseBinary>,
}
