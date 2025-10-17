use anyhow::anyhow;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespDataType {
    SimpleString,
    SimpleError,
    Integers,
    BulkStrings,
    Arrays,
}

impl RespDataType {
    pub fn identify(value: u8) -> anyhow::Result<Self> {
        let value = match value {
            43 => Self::SimpleString,
            45 => Self::SimpleError,
            58 => Self::Integers,
            36 => Self::BulkStrings,
            42 => Self::Arrays,
            _ => {
                return Err(anyhow!("NOT_SUPPORTED"));
            }
        };
        Ok(value)
    }
}

#[cfg(test)]
pub mod test_resp_data_type {
    use super::*;

    #[test]
    fn test_identify() {
        struct TestCase {
            pub input: u8,
            pub expected: RespDataType,
            pub result_is_error: bool,
            pub assert_is_error: bool,
        }
        let test_cases = vec![
            TestCase {
                input: 36,
                expected: RespDataType::BulkStrings,
                result_is_error: false,
                assert_is_error: false,
            },
            TestCase {
                input: 42,
                expected: RespDataType::Arrays,
                result_is_error: false,
                assert_is_error: false,
            },
            TestCase {
                input: 45,
                expected: RespDataType::SimpleString,
                result_is_error: false,
                assert_is_error: true,
            },
        ];

        for test_case in test_cases {
            let result = RespDataType::identify(test_case.input);
            if test_case.result_is_error {
                assert!(result.is_err());
            } else {
                assert!(result.is_ok(), "{:#?}", result.err());
                if test_case.assert_is_error {
                    assert_ne!(result.unwrap(), test_case.expected);
                } else {
                    assert_eq!(result.unwrap(), test_case.expected);
                }
            }
        }
    }
}
