use crate::types::Value;

#[derive(Debug)]
pub struct Integers {}

impl Integers {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let value = String::from_utf8_lossy(value);
        let parsed = value.parse::<i64>()?;
        Ok(Value::Integer(parsed))
    }
}

#[cfg(test)]
pub mod test_integers {
    use super::*;

    #[test]
    fn test_positive() {
        let result = Integers::build(&vec![53]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(Value::Integer(5), result.unwrap());
    }

    #[test]
    fn test_negative() {
        let result = Integers::build(&vec![45, 52, 50]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(Value::Integer(-42), result.unwrap());
    }
}
