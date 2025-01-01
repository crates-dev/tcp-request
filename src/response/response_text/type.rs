use lombok_macros::*;

#[derive(Debug, Clone, Lombok)]
pub struct TcpResponseText {
    pub data: String,
}
