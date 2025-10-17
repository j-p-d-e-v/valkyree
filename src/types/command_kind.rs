use crate::builder::commands::{Auth, AuthConfig, Get, Set};
use serde_json::Value;
#[derive(Debug, Clone)]
pub enum CommandKind {
    Auth(AuthConfig),
    Get(String),
    Set(String, Value),
}
impl CommandKind {
    pub fn build(&self) -> anyhow::Result<String> {
        match self {
            Self::Auth(config) => Auth::build(config),
            Self::Get(value) => Get::build(value),
            Self::Set(key, value) => Set::build(key, value),
        }
    }
}

#[cfg(test)]
pub mod test_command_kind {
    use super::*;

    #[test]
    fn test_get() {
        let result = CommandKind::Get("mykey".to_string()).build();
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("GET mykey\r\n", result.unwrap());
    }

    #[test]
    fn test_set() {
        let result =
            CommandKind::Set("mykey".to_string(), Value::String("myvalue".to_string())).build();
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("SET mykey myvalue\r\n", result.unwrap());
    }
}
