pub trait RequestTrait {
    type RequestResult;

    fn send(&mut self) -> Self::RequestResult;
}
