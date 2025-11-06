use crate::{
    builder::resp_data_type::{helpers::get_resp_value, RespDataTypeTrait},
    types::RespDataTypeValue,
};
use anyhow::anyhow;

#[derive(Debug)]
pub struct Integers<'a> {
    pub length: usize,
    pub value: &'a [u8],
}
impl<'a> RespDataTypeTrait<'a> for Integers<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }
    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (new_value, id) = get_resp_value(self.value, true)?;
        if !id.is_integers() {
            return Err(anyhow!("NOT_INTEGERS_TYPE"));
        }
        self.length = new_value.len() + 3;
        let value = String::from_utf8_lossy(new_value);
        let parsed = value.parse::<i64>()?;
        Ok(RespDataTypeValue::Integer(parsed))
    }
}

#[cfg(test)]
pub mod test_integers {
    use super::*;
    use crate::types::resp_data_kind::RespDataType;

    #[test]
    fn test_positive() {
        let identifier = RespDataType::Integers.to_decimal().unwrap();
        let input = vec![identifier, 53, 13, 10];
        let mut integers = Integers::new(&input);
        let result = integers.build();
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(RespDataTypeValue::Integer(5), result.unwrap());
    }

    #[test]
    fn test_negative() {
        let identifier = RespDataType::Integers.to_decimal().unwrap();
        let input = vec![identifier, 45, 52, 50, 13, 10];
        let mut integers = Integers::new(&input);
        let result = integers.build();
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(RespDataTypeValue::Integer(-42), result.unwrap());
    }
}
