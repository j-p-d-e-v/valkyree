use crate::types::Value;
#[derive(Debug)]
pub struct SimpleStrings {}

impl SimpleStrings {
    pub fn build(value: &[u8]) -> Value {
        let value = String::from_utf8_lossy(value);
        Value::String(value.to_string())
    }
}

#[cfg(test)]
pub mod test_simple_strings {
    use super::*;

    #[test]
    fn test_not_empty() {
        let result = SimpleStrings::build(&vec![79, 75]);
        assert_eq!(Value::String("OK".to_string()), result);
    }

    #[test]
    fn test_empty() {
        let result = SimpleStrings::build(&vec![]);
        assert!(result.is_string());
    }
}
