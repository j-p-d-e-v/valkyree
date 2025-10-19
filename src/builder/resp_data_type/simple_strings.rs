use crate::builder::resp_data_type::RespDataTypeBase;
use crate::types::resp_data_kind::RespDataType;
use crate::types::Value;
#[derive(Debug)]
pub struct SimpleStrings {}

impl SimpleStrings {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        Self::is_data_type(value, RespDataType::SimpleStrings)?;
        let value = Self::get_value(value)?;
        let value = String::from_utf8_lossy(&value);
        Ok(Value::String(value.to_string()))
    }
}
impl RespDataTypeBase for SimpleStrings {}

#[cfg(test)]
pub mod test_simple_strings {
    use super::*;

    #[test]
    fn test_not_empty() {
        let identifier = RespDataType::SimpleStrings.to_decimal().unwrap();
        let result = SimpleStrings::build(&vec![identifier, 79, 75, 13, 10]);
        assert_eq!(Value::String("OK".to_string()), result.unwrap());
    }

    #[test]
    fn test_empty() {
        let identifier = RespDataType::SimpleStrings.to_decimal().unwrap();
        let result = SimpleStrings::build(&vec![identifier, 13, 10]);
        assert!(result.unwrap().is_string());
    }
}
