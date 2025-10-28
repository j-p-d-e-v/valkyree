use crate::{
    builder::resp_data_type::{RespDataTypeTrait, helpers::get_resp_value},
    types::RespDataTypeValue,
};
use anyhow::anyhow;

#[derive(Debug)]
pub struct Booleans<'a> {
    pub length: usize,
    pub value: &'a [u8],
}
impl<'a> RespDataTypeTrait<'a> for Booleans<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }
    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (new_value, id) = get_resp_value(self.value, true)?;
        if !id.is_booleans() {
            return Err(anyhow!("NOT_BOOLEANS_TYPE"));
        }
        self.length = new_value.len() + 3;
        let value = match String::from_utf8_lossy(new_value).to_string().as_str() {
            "t" => RespDataTypeValue::Boolean(true),
            "f" => RespDataTypeValue::Boolean(false),
            _ => {
                return Err(anyhow!("BOOLEAN_INVALID_VALUE".to_string()));
            }
        };
        Ok(value)
    }
}
#[cfg(test)]
pub mod test_booleans {
    use crate::types::resp_data_kind::RespDataType;

    use super::*;

    #[test]
    fn test_booleans() {
        let identifier = RespDataType::Booleans.to_decimal().unwrap();

        struct TestCase {
            pub input: Vec<u8>,
            pub expected: RespDataTypeValue,
        }

        let test_cases = vec![
            TestCase {
                // #t\r\n
                input: vec![identifier, 116, 13, 10], // 't'
                expected: RespDataTypeValue::Boolean(true),
            },
            TestCase {
                // #f\r\n
                input: vec![identifier, 102, 13, 10], // 'f'
                expected: RespDataTypeValue::Boolean(false),
            },
        ];

        for test_case in test_cases {
            let mut booleans = Booleans::new(&test_case.input);
            let result = booleans.build();
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }
}
