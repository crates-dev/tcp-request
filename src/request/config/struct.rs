use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Config {
    pub(crate) host: String,
    pub(crate) port: usize,
    pub(crate) timeout: u64,
    pub(crate) buffer_size: usize,
}
