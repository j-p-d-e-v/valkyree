use crate::builder::resp_data_type::arrays::Arrays;
use crate::builder::resp_data_type::{BigNumbers, Booleans, BulkStrings, Doubles, Integers, Nulls};
use crate::types::Value;
use crate::{
    builder::resp_data_type::{SimpleErrors, SimpleStrings},
    types::resp_data_kind::RespDataType,
};
use anyhow::anyhow;
#[derive(Debug, Clone)]
pub struct RespParser {
    pub value: Vec<u8>,
}

impl RespParser {
    pub fn new(value: &Vec<u8>) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    pub fn get_data_type(&self) -> anyhow::Result<RespDataType> {
        match self.value.first() {
            Some(b) => Ok(RespDataType::identify(b.to_owned())?),
            None => Err(anyhow!("DATA_TYPE_NOT_FOUND".to_string())),
        }
    }

    pub fn get_value(&self) -> anyhow::Result<Vec<u8>> {
        match self.value.get(1..self.value.len() - 2) {
            Some(data) => Ok(data.to_vec()),
            None => Err(anyhow!("INVALID_RESULT_VALUE")),
        }
    }
    pub fn get(&self) -> anyhow::Result<Value> {
        let data_type = self.get_data_type()?;
        let valueb = self.get_value()?;
        match data_type {
            RespDataType::SimpleStrings => Ok(SimpleStrings::build(&valueb)?),
            RespDataType::SimpleErrors => Ok(SimpleErrors::build(&valueb)?),
            RespDataType::Integers => Ok(Integers::build(&valueb)?),
            RespDataType::BulkStrings => Ok(BulkStrings::build(&valueb)?),
            RespDataType::Nulls => Ok(Nulls::build(&valueb)?),
            RespDataType::Booleans => Ok(Booleans::build(&valueb)?),
            RespDataType::BigNumbers => Ok(BigNumbers::build(&valueb)?),
            RespDataType::Doubles => Ok(Doubles::build(&valueb)?),
            _ => Ok(Value::Null),
        }
    }
}

#[cfg(test)]
pub mod test_result {
    use super::*;

    #[test]
    fn test_data_type() {
        let input: Vec<u8> = vec![43, 79, 75, 13, 10]; //+Ok\r\n
        let result = RespParser::new(&input);
        let data_type = result.get_data_type();
        assert!(data_type.is_ok(), "{:#?}", data_type.err());
        let data_type = data_type.unwrap();
        assert_eq!(data_type, RespDataType::SimpleStrings);
        assert_ne!(data_type, RespDataType::SimpleErrors);
    }

    #[test]
    fn test_value() {
        let input: Vec<u8> = vec![43, 79, 75, 13, 10]; //+Ok\r\n
        let result = RespParser::new(&input);
        let value = result.get_value();
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();
        assert_eq!(value, vec![79, 75]);
        assert_ne!(value, vec![79, 75, 13, 10]);
    }

    #[test]
    fn test_get() {
        let input: Vec<u8> = vec![43, 79, 75, 13, 10]; //+Ok\r\n
        let result = RespParser::new(&input);
        let value = result.get();
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();
        assert_eq!(value, Value::String("OK".to_string()));
    }
}
