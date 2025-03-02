use crate::*;

pub trait RequestTrait: Send + Debug {
    type RequestResult: Sized;

    fn send(&mut self, data: &[u8]) -> Self::RequestResult;
}
