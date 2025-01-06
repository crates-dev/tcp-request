use super::r#type::TcpResponseBinary;
use crate::response::{r#trait::ResponseTrait, response_text::r#type::TcpResponseText};

impl ResponseTrait for TcpResponseBinary {
    type OutputText = TcpResponseText;
    type OutputBinary = TcpResponseBinary;

    #[inline]
    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        response.to_vec()
    }

    #[inline]
    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    #[inline]
    fn text(&self) -> TcpResponseText {
        let data: String = String::from_utf8_lossy(&self).to_string();
        data
    }

    #[inline]
    fn decode(&self) -> TcpResponseBinary {
        let data: Vec<u8> = self.clone();
        data
    }
}
