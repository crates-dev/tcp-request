use crate::*;

#[derive(Debug, Clone, Data)]
pub struct TcpRequest {
    pub(crate) config: ArcRwLock<Config>,
    pub(crate) response: ArcRwLock<TcpResponseBinary>,
}
