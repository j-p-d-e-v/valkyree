use anyhow::anyhow;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::builder::commands::AuthConfig;
use crate::types::command_kind::CommandKind;

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub address: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConnectionBuilder {
    config: ConnectionConfig,
}

impl ConnectionBuilder {
    pub fn new(config: ConnectionConfig) -> Self {
        Self { config }
    }

    pub async fn connect(&self) -> anyhow::Result<TcpStream> {
        let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
        todo!("Put more validation here if successfully connected");
        Ok(())
    }
}

#[cfg(test)]
pub mod test_connection {
    use super::*;

    #[tokio::test]
    async fn test() {
        let connection = ConnectionBuilder::new(ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("zxczxc123".to_string()),
        })
        .connect()
        .await;
        println!("Resut: {:#?}", connection);
    }
}
