use crate::builder::resp_data_type::RespDataTypeTrait;
use crate::builder::resp_data_type::helpers::get_resp_value;
use crate::types::RespDataTypeValue;
use anyhow::anyhow;
#[derive(Debug)]
pub struct SimpleStrings<'a> {
    pub length: usize,
    pub value: &'a [u8],
}

impl<'a> RespDataTypeTrait<'a> for SimpleStrings<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }
    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (new_value, id) = get_resp_value(self.value, true)?;
        if !id.is_simple_strings() {
            return Err(anyhow!("NOT_SIMPLE_STRINGS_TYPE"));
        }
        self.length = new_value.len() + 3;
        let data = String::from_utf8_lossy(new_value).to_string();
        Ok(RespDataTypeValue::String(data))
    }
}

#[cfg(test)]
pub mod test_simple_strings {
    use crate::types::resp_data_kind::RespDataType;

    use super::*;

    #[test]
    fn test_not_empty() {
        let identifier = RespDataType::SimpleStrings.to_decimal().unwrap();
        let input = vec![identifier, 79, 75, 13, 10];
        let mut sstrings = SimpleStrings::new(&input);
        let result = sstrings.build();
        assert_eq!(RespDataTypeValue::String("OK".to_string()), result.unwrap());
    }

    #[test]
    fn test_empty() {
        let identifier = RespDataType::SimpleStrings.to_decimal().unwrap();
        let input = vec![identifier, 13, 10];
        let mut sstrings = SimpleStrings::new(&input);
        let result = sstrings.build();
        assert!(result.unwrap().is_string());
    }
}
