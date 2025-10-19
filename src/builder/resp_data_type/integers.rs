use crate::{
    builder::resp_data_type::RespDataTypeBase,
    types::{RespDataTypeValue, resp_data_kind::RespDataType},
};

#[derive(Debug)]
pub struct Integers {}

impl RespDataTypeBase for Integers {}
impl Integers {
    pub fn build(value: &[u8]) -> anyhow::Result<RespDataTypeValue> {
        Self::is_data_type(value, RespDataType::Integers)?;
        let value = Self::get_value(value, true)?;
        let value = String::from_utf8_lossy(&value);
        let parsed = value.parse::<i64>()?;
        Ok(RespDataTypeValue::Integer(parsed))
    }
}

#[cfg(test)]
pub mod test_integers {
    use crate::types::resp_data_kind::RespDataType;

    use super::*;

    #[test]
    fn test_positive() {
        let identifier = RespDataType::Integers.to_decimal().unwrap();
        let result = Integers::build(&vec![identifier, 53, 13, 10]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(RespDataTypeValue::Integer(5), result.unwrap());
    }

    #[test]
    fn test_negative() {
        let identifier = RespDataType::Integers.to_decimal().unwrap();
        let result = Integers::build(&vec![identifier, 45, 52, 50, 13, 10]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(RespDataTypeValue::Integer(-42), result.unwrap());
    }
}
