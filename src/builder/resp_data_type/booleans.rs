use anyhow::anyhow;
use serde_json::Value;

#[derive(Debug)]
pub struct Booleans {}

impl Booleans {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let value = match String::from_utf8_lossy(value).to_string().as_str() {
            "t" => Value::Bool(true),
            "f" => Value::Bool(false),
            _ => {
                return Err(anyhow!("BOOLEAN_INVALID_VALUE".to_string()));
            }
        };
        Ok(value)
    }
}

#[cfg(test)]
pub mod test_booleans {
    use super::*;

    #[test]
    fn test_true() {
        let result = Booleans::build(&[116]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(Value::Bool(true), result.unwrap());
    }
    #[test]
    fn test_false() {
        let result = Booleans::build(&[102]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(Value::Bool(false), result.unwrap());
    }
}
