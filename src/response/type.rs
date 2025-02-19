use crate::*;

pub type BoxResponseTrait =
    Box<dyn ResponseTrait<OutputText = TcpResponseText, OutputBinary = TcpResponseBinary>>;
