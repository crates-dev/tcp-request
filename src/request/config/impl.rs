use crate::*;

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            buffer_size: DEFAULT_BUFFER_SIZE,
            host: EMPTY_STR.to_owned(),
            port: DEFAULT_WEB_PORT,
        }
    }
}
