use anyhow::anyhow;
use serde_json::Value;

#[derive(Debug)]
pub struct Set {}

impl Set {
    pub fn build(key: &str, value: &Value) -> anyhow::Result<String> {
        if key.is_empty() {
            return Err(anyhow!("SET_KEY_REQUIRED"));
        }
        let value = match value.to_owned() {
            Value::String(value) => value,
            Value::Number(number) => number.to_string(),
            _ => {
                return Err(anyhow!("SET_VALUE_NOT_SUPPORTED".to_string()));
            }
        };
        Ok(format!("SET {key} {value}\r\n"))
    }
}

#[cfg(test)]
pub mod test_set {
    use serde_json::Number;

    use super::*;

    #[test]
    fn test_set_string() {
        let result = Set::build("mykey", &Value::String("mystringvalue".to_string()));
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("SET mykey mystringvalue\r\n".to_string(), result.unwrap());
    }

    #[test]
    fn test_set_number() {
        let result = Set::build("mykey", &Value::Number(Number::from_f64(1.00001).unwrap()));
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("SET mykey 1.00001\r\n".to_string(), result.unwrap());

        let result = Set::build("mykey", &Value::Number(Number::from_u128(1).unwrap()));
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("SET mykey 1\r\n".to_string(), result.unwrap());
    }
}
