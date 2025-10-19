use crate::{
    builder::resp_data_type::RespDataTypeBase,
    types::{RespDataTypeValue, resp_data_kind::RespDataType},
};
use anyhow::anyhow;

#[derive(Debug)]
pub struct Booleans {}

impl RespDataTypeBase for Booleans {}
impl Booleans {
    pub fn build(value: &[u8]) -> anyhow::Result<RespDataTypeValue> {
        Self::is_data_type(value, RespDataType::Booleans)?;
        let value = Self::get_value(value, true)?;
        let value = match String::from_utf8_lossy(&value).to_string().as_str() {
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
            let result = Booleans::build(&test_case.input);
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }
}
