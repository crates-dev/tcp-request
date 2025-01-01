use super::r#type::TcpResponseBinary;
use crate::response::{r#trait::ResponseTrait, response_text::r#type::TcpResponseText};

impl ResponseTrait for TcpResponseBinary {
    type OutputText = TcpResponseText;
    type OutputBinary = TcpResponseBinary;

    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        TcpResponseBinary {
            data: response.to_vec(),
        }
    }

    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    fn text(&self) -> TcpResponseText {
        let data: String = String::from_utf8_lossy(&self.data).to_string();
        TcpResponseText { data }
    }

    fn decode(&self) -> TcpResponseBinary {
        let data: Vec<u8> = self.data.clone();
        TcpResponseBinary { data }
    }
}

impl Default for TcpResponseBinary {
    fn default() -> Self {
        Self {
            data: Vec::default(),
        }
    }
}
