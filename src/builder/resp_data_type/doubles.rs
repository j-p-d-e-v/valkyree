// TODO
// Work In Progress
// Check if serde Value needs to replace with our own Value
use serde_json::Value;

#[derive(Debug)]
pub struct Doubles {}

impl Doubles {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let value = String::from_utf8_lossy(value);
        Ok(Value::String(value.to_string()))
    }
}

#[cfg(test)]
pub mod test_doubles {
    use super::*;

    #[test]
    fn test_not_empty() {
        let result = Doubles::build(&vec![79, 75]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("OK".to_string(), result.unwrap());
    }

    #[test]
    fn test_empty() {
        let result = Doubles::build(&vec![]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert!(result.unwrap().is_string());
    }
}

