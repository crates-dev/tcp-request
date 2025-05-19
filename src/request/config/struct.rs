use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Data)]
pub struct Config {
    pub host: String,
    pub port: usize,
    pub timeout: u64,
    pub buffer_size: usize,
}
