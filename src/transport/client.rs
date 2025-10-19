use crate::builder::commands::AuthConfig;
use crate::transport::connection::{ConnectionBuilder, ConnectionConfig};
use crate::transport::execute::Execute;
use crate::types::RespDataTypeValue;
use crate::types::command_kind::CommandKind;
use anyhow::anyhow;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Client {
    pub tcp_stream: Arc<RwLock<TcpStream>>,
    pub config: ConnectionConfig,
}

impl Client {
    pub async fn new(config: ConnectionConfig) -> anyhow::Result<Self> {
        let connection = ConnectionBuilder::new(&config);
        let stream = connection.connect().await?;

        Ok(Self {
            tcp_stream: stream.clone(),
            config,
        })
    }

    async fn auth(&self) -> anyhow::Result<()> {
        if self.config.username.is_some() || self.config.password.is_some() {
            let stream = self.tcp_stream.clone();
            let config = self.config.clone();
            let command = CommandKind::Auth(AuthConfig {
                username: config.username,
                password: config.password,
            })
            .build()?;
            let execute = Execute::new(stream).await;
            let result = execute.send(&command).await?;
            if let RespDataTypeValue::SimpleError(kind, message) = result {
                return Err(anyhow!(format!(
                    "CLIENT_AUTH_ERROR: {:?}, {}",
                    kind, message
                )));
            }
        }
        Ok(())
    }

    pub async fn send(&self, command: CommandKind) -> anyhow::Result<RespDataTypeValue> {
        self.auth().await?;
        let command = command.build()?;
        let stream = self.tcp_stream.clone();
        let execute = Execute::new(stream).await;
        execute.send(&command).await
    }
}

#[cfg(test)]
pub mod test_client {
    use serde_json::Value;

    use super::*;
    #[tokio::test]
    async fn test_set_get() {
        let config = ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        };
        let client = Client::new(config).await;
        assert!(client.is_ok(), "{:#?}", client.err());
        let client = client.unwrap();
        let set_command =
            CommandKind::Set("myclient".to_string(), Value::String("hey".to_string()));
        let result = client.send(set_command).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert!(result.unwrap().is_string());
        let get_command = CommandKind::Get("myclient".to_string());
        let result = client.send(get_command).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert_eq!(
            RespDataTypeValue::String("hey".to_string()),
            result.unwrap()
        );
    }
}
