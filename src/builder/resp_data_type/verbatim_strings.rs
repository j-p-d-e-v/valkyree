use crate::builder::resp_data_type::RespDataTypeTrait;
use crate::builder::resp_data_type::helpers::get_resp_multi_values;
use crate::types::{RespDataTypeValue, VerbatimEncoding};
use anyhow::anyhow;

#[derive(Debug)]
pub struct VerbatimStrings<'a> {
    value: &'a [u8],
    length: usize,
}

impl<'a> RespDataTypeTrait<'a> for VerbatimStrings<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }
    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (start, length, id) = get_resp_multi_values(self.value)?;
        if !id.is_verbatim_strings() {
            return Err(anyhow!("NOT_VERBATIM_STRINGS_TYPE"));
        }
        if length == 0 {
            return Ok(RespDataTypeValue::String("".to_string()));
        } else if length <= -1 {
            return Ok(RespDataTypeValue::Null);
        }

        let end = start + length as usize;
        if (length as usize) > end {
            return Err(anyhow!("VERBATIM_STRINGS_LENGTH_ERROR"));
        }
        let encoding = if let Some(values) = &self.value.get(start..start + 3) {
            VerbatimEncoding::from(&String::from_utf8_lossy(values))
        } else {
            return Err(anyhow!("VERBATIM_STRINGS_ENCODING_ERROR"));
        };
        let message = if let Some(values) = &self.value.get(start + 4..end) {
            String::from_utf8_lossy(values).to_string()
        } else {
            String::new()
        };
        self.length = end + 2;
        Ok(RespDataTypeValue::VerbatimString(message, encoding))
    }
}
#[cfg(test)]
pub mod test_verbatim_strings {
    use super::*;

    #[test]
    fn test_string() {
        struct TestCase {
            pub input: Vec<u8>,
            pub expected: RespDataTypeValue,
        }

        let test_cases = vec![
            TestCase {
                // =9\r\ntxt:hello\r\n
                input: vec![
                    61, // '='
                    57, 13, 10, // "9\r\n"
                    116, 120, 116, 58, 104, 101, 108, 108, 111, // "txt:hello"
                    13, 10,
                ],
                expected: RespDataTypeValue::VerbatimString(
                    "hello".to_string(),
                    VerbatimEncoding::Txt,
                ),
            },
            TestCase {
                // =12\r\nmkd:## Hello\r\n
                input: vec![
                    61, 49, 50, 13, 10, // "12\r\n"
                    109, 107, 100, 58, 35, 35, 32, 72, 101, 108, 108, 111, // "mkd:## Hello"
                    13, 10,
                ],
                expected: RespDataTypeValue::VerbatimString(
                    "## Hello".to_string(),
                    VerbatimEncoding::Mkd,
                ),
            },
            TestCase {
                // =16\r\njsn:{"ok":true}\r\n
                input: vec![
                    61, 49, 53, 13, 10, // "15\r\n"
                    106, 115, 110, 58, 123, 34, 111, 107, 34, 58, 116, 114, 117, 101, 125, 13, 10,
                ],
                expected: RespDataTypeValue::VerbatimString(
                    r#"{"ok":true}"#.to_string(),
                    VerbatimEncoding::Unknown("jsn".to_string()),
                ),
            },
        ];
        for test_case in test_cases {
            let mut verbatim_strings = VerbatimStrings::new(&test_case.input);
            let result = verbatim_strings.build();
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }
}
