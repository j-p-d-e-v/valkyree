use crate::{
    builder::resp_data_type::RespDataTypeBase,
    types::{RespDataTypeValue, resp_data_kind::RespDataType},
};

#[derive(Debug)]
pub struct Nulls {}

impl RespDataTypeBase for Nulls {}
impl Nulls {
    pub fn build(value: &[u8]) -> anyhow::Result<RespDataTypeValue> {
        Self::is_data_type(value, RespDataType::Nulls)?;
        let _ = Self::get_value(value, true)?;
        Ok(RespDataTypeValue::Null)
    }
}

#[cfg(test)]
pub mod test_nulls {
    use crate::types::resp_data_kind::RespDataType;

    use super::*;

    #[test]
    fn test() {
        let identifier = RespDataType::Nulls.to_decimal().unwrap();
        // _\r\n
        let result = Nulls::build(&[identifier, 13, 10]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(RespDataTypeValue::Null, result.unwrap());

        // _\n\n\r\n
        let result = Nulls::build(&[identifier, 10, 13, 13, 10]);
        assert_eq!(RespDataTypeValue::Null, result.unwrap());
    }
}
