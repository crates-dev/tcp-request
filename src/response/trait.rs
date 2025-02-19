use crate::*;

pub trait ResponseTrait: Send + Debug {
    type OutputText: Clone + Sized;
    type OutputBinary: Clone + Sized;

    fn text(&self) -> Self::OutputText;

    fn binary(&self) -> Self::OutputBinary;

    fn from(response: &[u8]) -> Self
    where
        Self: Sized;
}
