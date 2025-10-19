use crate::types::Value;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Booleans {}

impl Booleans {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let value = match String::from_utf8_lossy(value).to_string().as_str() {
            "t" => Value::Boolean(true),
            "f" => Value::Boolean(false),
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
        assert_eq!(Value::Boolean(true), result.unwrap());
    }
    #[test]
    fn test_false() {
        let result = Booleans::build(&[102]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(Value::Boolean(false), result.unwrap());
    }
}
