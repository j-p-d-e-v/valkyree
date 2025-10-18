use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

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

    pub async fn connect(&self) -> anyhow::Result<Arc<RwLock<TcpStream>>> {
        let stream = TcpStream::connect(&self.config.address).await?;
        Ok(Arc::new(RwLock::new(stream)))
    }
}

#[cfg(test)]
pub mod test_connection {
    use super::*;

    #[tokio::test]
    async fn test_connected() {
        let connection = ConnectionBuilder::new(ConnectionConfig {
            address: "127.0.0.1:6379".to_string(),
            username: Some("myapp".to_string()),
            password: Some("zxczxc123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_ok(), "{:#?}", connection.err());
    }

    #[tokio::test]
    async fn test_error() {
        let connection = ConnectionBuilder::new(ConnectionConfig {
            address: "127.0.0.1:1111".to_string(),
            username: Some("myapp".to_string()),
            password: Some("zxczxc123".to_string()),
        })
        .connect()
        .await;
        assert!(connection.is_err());
    }
}
