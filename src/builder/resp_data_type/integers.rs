use anyhow::anyhow;
use serde_json::{Number, Value};

#[derive(Debug)]
pub struct Integers {}

impl Integers {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let value = String::from_utf8_lossy(value);
        let parsed = value.parse::<i128>()?;
        let number = if let Some(n) = Number::from_i128(parsed) {
            n
        } else {
            return Err(anyhow!("INTEGERS_NOTANUMBER".to_string()));
        };
        Ok(Value::Number(number))
    }
}

#[cfg(test)]
pub mod test_integers {
    use super::*;

    #[test]
    fn test_positive() {
        let result = Integers::build(&vec![53]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(
            Value::Number(Number::from_i128(5).unwrap()),
            result.unwrap()
        );
    }

    #[test]
    fn test_negative() {
        let result = Integers::build(&vec![45, 52, 50]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(
            Value::Number(Number::from_i128(-42).unwrap()),
            result.unwrap()
        );
    }
}
