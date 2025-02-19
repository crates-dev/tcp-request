use crate::*;

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
}
