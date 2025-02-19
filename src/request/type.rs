use crate::*;

pub type RequestResult = Result<BoxResponseTrait, Error>;
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
