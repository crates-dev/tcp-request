pub trait ResponseTrait {
    type OutputText;
    type OutputBinary;

    fn text(&self) -> Self::OutputText;

    fn binary(&self) -> Self::OutputBinary;

    fn from(response: &[u8]) -> Self
    where
        Self: Sized;

    fn decode(&self) -> Self::OutputBinary;
}
