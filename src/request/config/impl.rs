use super::r#type::Config;
use crate::constant::r#type::DEFAULT_TIMEOUT;
use http_type::*;

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            buffer_size: 1_024_000,
            host: EMPTY_STR.to_owned(),
            port: usize::default(),
        }
    }
}
