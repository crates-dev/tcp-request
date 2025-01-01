use super::r#trait::RequestTrait;
use crate::request::error::r#type::Error;
use crate::response::r#type::BoxResponseTrait;

pub type RequestResult = Result<BoxResponseTrait, Error>;

pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
