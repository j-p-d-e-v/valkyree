use serde_json::Value;

#[derive(Debug)]
pub struct SimpleStrings {}

impl SimpleStrings {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let value = String::from_utf8_lossy(value);
        Ok(Value::String(value.to_string()))
    }
}

#[cfg(test)]
pub mod test_simple_strings {
    use super::*;

    #[test]
    fn test_not_empty() {
        let result = SimpleStrings::build(&vec![79, 75]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("OK".to_string(), result.unwrap());
    }

    #[test]
    fn test_empty() {
        let result = SimpleStrings::build(&vec![]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert!(result.unwrap().is_string());
    }
}
