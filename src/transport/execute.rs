use tokio::net::TcpStream;

use crate::types::command_kind::CommandKind;

#[derive(Debug, Clone)]
pub struct Execute<'a> {
    pub stream: &'a TcpStream,
}

impl<'a> Execute<'a> {
    pub async fn new(stream: &'a TcpStream) -> Self {
        Self { stream }
    }
    pub async fn send(command: &String) {
        let command = command.as_bytes();
    }
}
