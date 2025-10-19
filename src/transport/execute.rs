use crate::builder::resp_data_type::RespParser;
use crate::types::Value;
use anyhow::anyhow;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Execute {
    pub stream: Arc<RwLock<TcpStream>>,
}

impl Execute {
    pub async fn new(stream: Arc<RwLock<TcpStream>>) -> Self {
        Self {
            stream: stream.clone(),
        }
    }
    pub async fn send(&self, command: &String) -> anyhow::Result<Value> {
        let command = command.as_bytes();
        let stream = self.stream.clone();
        let mut connection = stream.write().await;
        connection.write_all(command).await?;
        let mut data: Vec<u8> = Vec::new();
        loop {
            connection.readable().await?;
            let mut buf = [0; 32];
            match connection.try_read(&mut buf) {
                Ok(0) => break,
                Ok(size) => {
                    for b in buf[..size].iter() {
                        data.push(*b);
                    }
                }
                Err(err) => {
                    if err.kind() == tokio::io::ErrorKind::WouldBlock {
                        break;
                    }
                    return Err(anyhow!(err.to_string()));
                }
            }
        }
        let result = RespParser::new(&data);
        result.get()
    }
}

#[cfg(test)]
pub mod test_execute {
    use serde_json::Value;

    use super::*;
    use crate::builder::commands::AuthConfig;
    use crate::transport::connection::{ConnectionBuilder, ConnectionConfig};
    use crate::types::command_kind::CommandKind;

    #[tokio::test]
    async fn test_auth() {
        let connection = ConnectionBuilder::new(ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("zxczxc123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let auth_command = CommandKind::Auth(AuthConfig {
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .build();
        assert!(auth_command.is_ok(), "{:#?}", auth_command.err());
        let command = auth_command.unwrap();
        let execute = Execute::new(stream).await;
        let result = execute.send(&command).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
    }

    #[tokio::test]
    async fn test_set_get() {
        let connection = ConnectionBuilder::new(ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let execute = Execute::new(stream).await;
        let auth_command = CommandKind::Auth(AuthConfig {
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .build();
        assert!(auth_command.is_ok(), "{:#?}", auth_command.err());
        let _ = execute.send(&auth_command.unwrap()).await;
        let set_command =
            CommandKind::Set("testmykey".to_string(), Value::String("hello".to_string())).build();
        assert!(set_command.is_ok(), "{:#?}", set_command.err());
        let result = execute.send(&set_command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
    }

    #[tokio::test]
    async fn test_raw() {
        let connection = ConnectionBuilder::new(ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: None,
            password: None,
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let raw_command = CommandKind::Raw("myerror".to_string()).build();
        assert!(raw_command.is_ok(), "{:#?}", raw_command.err());
        let command = raw_command.unwrap();
        let execute = Execute::new(stream).await;
        let result = execute.send(&command).await;
        assert!(result.is_err());
    }
}
