use crate::*;

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
        self.clone().into_bytes()
    }
}
