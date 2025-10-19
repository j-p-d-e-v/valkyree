use crate::types::Value;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Nulls {}

impl Nulls {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        if value.is_empty() {
            return Err(anyhow!("NULL_INVALID_VALUE".to_string()));
        }
        Ok(Value::Null)
    }
}

#[cfg(test)]
pub mod test_nulls {
    use super::*;

    #[test]
    fn test() {
        // _\r\n
        let result = Nulls::build(&[]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(Value::Null, result.unwrap());

        // _\n\n\r\n
        let result = Nulls::build(&[13, 10]);
        assert!(result.is_err());
    }
}
