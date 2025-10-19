use crate::builder::resp_data_type::arrays::Arrays;
use crate::builder::resp_data_type::{BigNumbers, Booleans, BulkStrings, Doubles, Integers, Nulls};
use crate::types::RespDataTypeValue;
use crate::{
    builder::resp_data_type::{SimpleErrors, SimpleStrings},
    types::resp_data_kind::RespDataType,
};
use anyhow::anyhow;
#[derive(Debug)]
pub struct RespParser {}

impl RespParser {
    pub fn parse(value: &[u8]) -> anyhow::Result<RespDataTypeValue> {
        let identifer = match value.first() {
            Some(b) => RespDataType::identify(b.to_owned())?,
            None => {
                return Err(anyhow!("DATA_TYPE_NOT_FOUND".to_string()));
            }
        };
        match identifer {
            RespDataType::SimpleStrings => Ok(SimpleStrings::build(value)?),
            RespDataType::SimpleErrors => Ok(SimpleErrors::build(value)?),
            RespDataType::Integers => Ok(Integers::build(value)?),
            RespDataType::BulkStrings => Ok(BulkStrings::build(value)?),
            RespDataType::Nulls => Ok(Nulls::build(value)?),
            RespDataType::Booleans => Ok(Booleans::build(value)?),
            RespDataType::BigNumbers => Ok(BigNumbers::build(value)?),
            RespDataType::Doubles => Ok(Doubles::build(value)?),
            RespDataType::Arrays => Ok(Arrays::build(value)?),
        }
    }
}

#[cfg(test)]
pub mod test_result {
    use super::*;
    #[test]
    fn test_get() {
        let input: Vec<u8> = vec![43, 79, 75, 13, 10]; //+Ok\r\n
        let result = RespParser::parse(&input);
        assert!(result.is_ok(), "{:#?}", result.err());
        let value = result.unwrap();
        assert_eq!(value, RespDataTypeValue::String("OK".to_string()));
    }
}
