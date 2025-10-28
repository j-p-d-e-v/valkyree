use crate::{
    builder::resp_data_type::{RespDataTypeTrait, helpers::get_resp_value},
    types::RespDataTypeValue,
};
use anyhow::anyhow;

#[derive(Debug)]
pub struct Nulls<'a> {
    pub length: usize,
    pub value: &'a [u8],
}

impl<'a> RespDataTypeTrait<'a> for Nulls<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }
    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (_, id) = get_resp_value(self.value, true)?;
        if !id.is_nulls() {
            return Err(anyhow!("NOT_NULLS_TYPE"));
        }
        self.length = 3;
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
        let input = &[identifier, 13, 10];
        let mut nulls = Nulls::new(input);
        let result = nulls.build();
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(RespDataTypeValue::Null, result.unwrap());

        // _\n\n\r\n
        let input = &[identifier, 10, 13, 13, 10];
        let mut nulls = Nulls::new(input);
        let result = nulls.build();
        assert_eq!(RespDataTypeValue::Null, result.unwrap());
    }
}
