use serde_json::Value;

use crate::builder::resp_data_type::helpers::get_length;

#[derive(Debug)]
pub struct BulkStrings {}

impl BulkStrings {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let l = get_length(value)?;
        let start = l.0;
        let length = l.1;

        if length == 0 {
            return Ok(Value::String("".to_string()));
        } else if length == -1 {
            return Ok(Value::Null);
        }
        let value = value.get(start..).unwrap_or(&[]);
        let result = String::from_utf8_lossy(value);
        Ok(Value::String(result.to_string()))
    }
}

#[cfg(test)]
pub mod test_bulk_strings {
    use super::*;

    #[test]
    fn test_string() {
        struct TestCase<'a> {
            pub input: &'a [u8],
            pub expected: String,
        }
        let test_cases = vec![
            TestCase {
                //5\r\nhello
                input: &[53, 13, 10, 104, 101, 108, 108, 111],
                expected: "hello".to_string(),
            },
            TestCase {
                //17\r\nhello\r\nhi\r\nworld
                input: &[
                    49, 55, 13, 10, 104, 101, 108, 108, 111, 13, 10, 104, 105, 13, 10, 119, 111,
                    114, 108, 100,
                ],
                expected: "hello\r\nhi\r\nworld".to_string(),
            },
            TestCase {
                // 18\r\nhello\r\nhi\r\nworld\r\n
                input: &[
                    49, 56, 13, 10, 104, 101, 108, 108, 111, 13, 10, 104, 105, 13, 10, 119, 111,
                    114, 108, 100, 13, 10,
                ],
                expected: "hello\r\nhi\r\nworld\r\n".to_string(),
            },
            TestCase {
                // 12\r\nline1\nline2
                input: &[
                    49, 50, 13, 10, 108, 105, 110, 101, 49, 10, 108, 105, 110, 101, 50,
                ],
                expected: "line1\nline2".to_string(),
            },
        ];
        for test_case in test_cases {
            let result = BulkStrings::build(test_case.input);
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }

    #[test]
    fn test_binary() {
        struct TestCase<'a> {
            pub input: &'a [u8],
            pub expected: String,
        }
        let test_cases = vec![
            TestCase {
                //4\r\n\x00\xFF\xAB\xCD
                input: &[52, 13, 10, 0, 255, 171, 205],
                expected: String::from_utf8_lossy(&[0x00, 0xFF, 0xAB, 0xCD]).to_string(),
            },
            TestCase {
                //10\r\n\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\r\n
                input: &[49, 48, 13, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                expected: String::from_utf8_lossy(&[
                    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
                ])
                .to_string(),
            },
        ];
        for test_case in test_cases {
            let result = BulkStrings::build(test_case.input);
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }

    #[test]
    fn test_null() {
        struct TestCase<'a> {
            pub input: &'a [u8],
        }
        let test_cases = vec![TestCase {
            //-1
            input: &[45, 49],
        }];
        for test_case in test_cases {
            let result = BulkStrings::build(test_case.input);
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(Value::Null, result.unwrap());
        }
    }

    #[test]
    fn test_empty() {
        struct TestCase<'a> {
            pub input: &'a [u8],
        }
        let test_cases = vec![TestCase {
            //0\r\n
            input: &[48, 13, 10, 13],
        }];
        for test_case in test_cases {
            let result = BulkStrings::build(test_case.input);
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(Value::String("".to_string()), result.unwrap());
        }
    }
}
