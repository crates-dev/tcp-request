use super::r#type::TcpResponseText;
use crate::response::{r#trait::ResponseTrait, response_binary::r#type::TcpResponseBinary};

impl ResponseTrait for TcpResponseText {
    type OutputText = TcpResponseText;
    type OutputBinary = TcpResponseBinary;

    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <TcpResponseBinary as ResponseTrait>::from(response).text()
    }

    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    fn binary(&self) -> TcpResponseBinary {
        TcpResponseBinary {
            data: self.data.clone().into_bytes(),
        }
    }

    fn decode(&self) -> TcpResponseBinary {
        let data: Vec<u8> = self.data.as_bytes().to_vec();
        TcpResponseBinary { data }
    }
}

impl Default for TcpResponseText {
    fn default() -> Self {
        Self {
            data: String::new(),
        }
    }
}
