use super::{
    r#trait::ResponseTrait, response_binary::r#type::TcpResponseBinary,
    response_text::r#type::TcpResponseText,
};

pub type BoxResponseTrait =
    Box<dyn ResponseTrait<OutputText = TcpResponseText, OutputBinary = TcpResponseBinary>>;
