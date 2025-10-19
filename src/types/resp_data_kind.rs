use anyhow::anyhow;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespDataType {
    SimpleStrings,
    SimpleErrors,
    Integers,
    BulkStrings,
    Arrays,
    Nulls,
    Booleans,
    Doubles,
    BigNumbers,
}

impl RespDataType {
    pub fn identify(value: u8) -> anyhow::Result<Self> {
        let value = match value {
            43 => Self::SimpleStrings,
            45 => Self::SimpleErrors,
            58 => Self::Integers,
            36 => Self::BulkStrings,
            42 => Self::Arrays,
            95 => Self::Nulls,
            35 => Self::Booleans,
            44 => Self::Doubles,
            40 => Self::BigNumbers,
            _ => {
                return Err(anyhow!("NOT_SUPPORTED"));
            }
        };
        Ok(value)
    }

    pub fn to_decimal(&self) -> anyhow::Result<u8> {
        let value = match self {
            Self::SimpleStrings => 43,
            Self::SimpleErrors => 45,
            Self::Integers => 58,
            Self::BulkStrings => 36,
            Self::Arrays => 42,
            Self::Nulls => 95,
            Self::Booleans => 35,
            Self::Doubles => 44,
            Self::BigNumbers => 40,
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
                expected: RespDataType::SimpleStrings,
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
