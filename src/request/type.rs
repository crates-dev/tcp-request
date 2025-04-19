use crate::*;

pub type RequestResult = Result<BoxResponseTrait, RequestError>;
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
