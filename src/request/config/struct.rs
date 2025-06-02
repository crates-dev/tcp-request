use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: usize,
    pub timeout: u64,
    pub buffer_size: usize,
}
