use crate::builder::resp_data_type::arrays::Arrays;
use crate::builder::resp_data_type::sets::Sets;
use crate::builder::resp_data_type::{
    BigNumbers, Booleans, BulkErrors, BulkStrings, Doubles, Integers, Maps, Nulls,
    RespDataTypeTrait, VerbatimStrings,
};
use crate::types::RespDataTypeValue;
use crate::{
    builder::resp_data_type::{SimpleErrors, SimpleStrings},
    types::resp_data_kind::RespDataType,
};
use anyhow::anyhow;
#[derive(Debug)]
pub struct RespParser<'a> {
    length: usize,
    value: &'a [u8],
}

impl<'a> RespParser<'a> {
    pub fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }

    fn builder<T: RespDataTypeTrait<'a>>(&mut self, b: T) -> anyhow::Result<RespDataTypeValue> {
        let mut b = b;
        let value = b.build()?;
        self.length = b.len();
        Ok(value)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn parse(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let value = self.value;
        let identifier = match value.first() {
            Some(b) => RespDataType::identify(b.to_owned())?,
            None => {
                return Err(anyhow!("DATA_TYPE_NOT_FOUND".to_string()));
            }
        };
        let value = match identifier {
            RespDataType::SimpleStrings => self.builder(SimpleStrings::new(value))?,
            RespDataType::SimpleErrors => self.builder(SimpleErrors::new(value))?,
            RespDataType::Integers => self.builder(Integers::new(value))?,
            RespDataType::BulkStrings => self.builder(BulkStrings::new(value))?,
            RespDataType::Nulls => self.builder(Nulls::new(value))?,
            RespDataType::Booleans => self.builder(Booleans::new(value))?,
            RespDataType::BigNumbers => self.builder(BigNumbers::new(value))?,
            RespDataType::Doubles => self.builder(Doubles::new(value))?,
            RespDataType::Arrays => self.builder(Arrays::new(value))?,
            RespDataType::BulkErrors => self.builder(BulkErrors::new(value))?,
            RespDataType::VerbatimStrings => self.builder(VerbatimStrings::new(value))?,
            RespDataType::Maps => self.builder(Maps::new(value))?,
            RespDataType::Sets => self.builder(Sets::new(value))?,
        };
        Ok(value)
    }
}

#[cfg(test)]
pub mod test_result {
    use super::*;
    #[test]
    fn test_get() {
        let input: Vec<u8> = vec![43, 79, 75, 13, 10]; //+Ok\r\n
        let mut parser = RespParser::new(&input);
        let result = parser.parse();
        assert!(result.is_ok(), "{:#?}", result.err());
        let value = result.unwrap();
        assert_eq!(value, RespDataTypeValue::String("OK".to_string()));
    }
}
