use crate::builder::resp_data_type::RespParser;
use crate::types::RespDataTypeValue;
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
    pub async fn send(&self, command: &String) -> anyhow::Result<RespDataTypeValue> {
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
        let result = RespParser::parse(&data)?;
        Ok(result)
    }
}

#[cfg(test)]
pub mod test_execute {
    use serde_json::Value;

    use super::*;
    use crate::builder::commands::AuthConfig;
    use crate::transport::connection::{ConnectionBuilder, ConnectionConfig};
    use crate::types::ExpiryKind;
    use crate::types::command_kind::CommandKind;

    async fn auth(execute: &Execute) -> anyhow::Result<RespDataTypeValue> {
        let auth_command = CommandKind::Auth(AuthConfig {
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .build();
        execute.send(&auth_command.unwrap()).await
    }

    #[tokio::test]
    async fn test_auth() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
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
    async fn test_get_not_exists_key() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let execute = Execute::new(stream).await;
        auth(&execute).await.unwrap();
        let get_command = CommandKind::Get("idontexistkey".to_string()).build();
        assert!(get_command.is_ok(), "{:#?}", get_command.err());
        let result = execute.send(&get_command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert_eq!(RespDataTypeValue::Null, result.unwrap());
    }

    #[tokio::test]
    async fn test_set_get() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let execute = Execute::new(stream).await;
        auth(&execute).await.unwrap();
        let set_command =
            CommandKind::Set("testmykey".to_string(), Value::String("hello".to_string())).build();
        assert!(set_command.is_ok(), "{:#?}", set_command.err());
        let result = execute.send(&set_command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        let get_command = CommandKind::Get("testmykey".to_string()).build();
        assert!(get_command.is_ok(), "{:#?}", get_command.err());
        let result = execute.send(&get_command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
    }

    #[tokio::test]
    async fn test_set_delete() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let execute = Execute::new(stream).await;
        auth(&execute).await.unwrap();
        let set_command =
            CommandKind::Set("deleteme".to_string(), Value::String("hello".to_string())).build();
        assert!(set_command.is_ok(), "{:#?}", set_command.err());
        let result = execute.send(&set_command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        let delete_command = CommandKind::Delete(vec!["deleteme".to_string()]).build();
        assert!(delete_command.is_ok(), "{:#?}", delete_command.err());
        let result = execute.send(&delete_command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert_eq!(RespDataTypeValue::Integer(1), result.unwrap());
    }

    #[tokio::test]
    async fn test_raw() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
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
        assert!(result.unwrap().is_simple_error());
    }

    #[tokio::test]
    async fn test_ping() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let execute = Execute::new(stream).await;
        auth(&execute).await.unwrap();
        let ping_command = CommandKind::Ping.build();
        assert!(ping_command.is_ok(), "{:#?}", ping_command.err());
        let command = ping_command.unwrap();
        let result = execute.send(&command).await;
        assert_eq!(
            RespDataTypeValue::String("PONG".to_string()),
            result.unwrap()
        );
    }

    #[tokio::test]
    async fn test_expire_ttl() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let execute = Execute::new(stream).await;
        auth(&execute).await.unwrap();
        let expire_command =
            CommandKind::Expire("expireme".to_string(), 10, Some(ExpiryKind::Nx)).build();
        assert!(expire_command.is_ok(), "{:#?}", expire_command.err());
        let result = execute.send(&expire_command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert!(result.unwrap().is_integer());
        let ttl_command = CommandKind::Ttl("expireme".to_string()).build();
        assert!(ttl_command.is_ok(), "{:#?}", ttl_command.err());
        let result = execute.send(&ttl_command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert!(result.unwrap().is_integer());
    }
    #[tokio::test]
    async fn test_incr_decr() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let execute = Execute::new(stream).await;
        auth(&execute).await.unwrap();
        let command =
            CommandKind::Increment("incrdecr".to_string()).build();
        assert!(command.is_ok(), "{:#?}", command.err());
        let result = execute.send(&command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        println!("result:{:#?}",result);
        assert!(result.unwrap().is_integer());
        let command =
            CommandKind::Decrement("incrdecr".to_string()).build();
        assert!(command.is_ok(), "{:#?}", command.err());
        let result = execute.send(&command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert!(result.unwrap().is_integer());
        let command =
            CommandKind::IncrementBy("incrdecr".to_string(),2).build();
        assert!(command.is_ok(), "{:#?}", command.err());
        let result = execute.send(&command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert!(result.unwrap().is_integer());
        let command =
            CommandKind::DecrementBy("incrdecr".to_string(),2).build();
        assert!(command.is_ok(), "{:#?}", command.err());
        let result = execute.send(&command.unwrap()).await;
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert!(result.unwrap().is_integer());
    }
    #[tokio::test]
    async fn test_keys() {
        let connection = ConnectionBuilder::new(&ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("password123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
        let stream = connection.unwrap();
        let execute = Execute::new(stream).await;
        auth(&execute).await.unwrap();
        let command =
            CommandKind::Keys("*".to_string()).build();
        assert!(command.is_ok(), "{:#?}", command.err());
        let result = execute.send(&command.unwrap()).await;
        println!("result: {:#?}",result);
        assert!(result.is_ok(), "{:#?}", result.is_err());
        assert!(result.unwrap().is_array());
    }
}
