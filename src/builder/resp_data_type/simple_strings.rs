use crate::builder::resp_data_type::RespDataTypeBase;
use crate::types::RespDataTypeValue;
use crate::types::resp_data_kind::RespDataType;
#[derive(Debug)]
pub struct SimpleStrings {}

impl SimpleStrings {
    pub fn build(value: &[u8]) -> anyhow::Result<RespDataTypeValue> {
        Self::is_data_type(value, RespDataType::SimpleStrings)?;
        let value = Self::get_value(value, true)?;
        let value = String::from_utf8_lossy(&value);
        Ok(RespDataTypeValue::String(value.to_string()))
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
        assert_eq!(RespDataTypeValue::String("OK".to_string()), result.unwrap());
    }

    #[test]
    fn test_empty() {
        let identifier = RespDataType::SimpleStrings.to_decimal().unwrap();
        let result = SimpleStrings::build(&vec![identifier, 13, 10]);
        assert!(result.unwrap().is_string());
    }
}
