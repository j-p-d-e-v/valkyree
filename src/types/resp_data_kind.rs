use anyhow::anyhow;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
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
    BulkErrors,
    VerbatimStrings,
    Maps,
}

impl RespDataType {
    pub fn is_maps(&self) -> bool {
        matches!(self, Self::Maps)
    }

    pub fn is_simple_strings(&self) -> bool {
        matches!(self, Self::SimpleStrings)
    }

    pub fn is_verbatim_strings(&self) -> bool {
        matches!(self, Self::VerbatimStrings)
    }

    pub fn is_bulk_errors(&self) -> bool {
        matches!(self, Self::BulkErrors)
    }

    pub fn is_simple_errors(&self) -> bool {
        matches!(self, Self::SimpleErrors)
    }

    pub fn is_integers(&self) -> bool {
        matches!(self, Self::Integers)
    }

    pub fn is_bulk_strings(&self) -> bool {
        matches!(self, Self::BulkStrings)
    }

    pub fn is_arrays(&self) -> bool {
        matches!(self, Self::Arrays)
    }

    pub fn is_nulls(&self) -> bool {
        matches!(self, Self::Nulls)
    }

    pub fn is_booleans(&self) -> bool {
        matches!(self, Self::Booleans)
    }

    pub fn is_doubles(&self) -> bool {
        matches!(self, Self::Doubles)
    }

    pub fn is_big_numbers(&self) -> bool {
        matches!(self, Self::BigNumbers)
    }
    // Returns the decimal equivalent of all resp types.
    pub fn get_identifiers_decimals() -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        for kind in Self::iter() {
            if let Ok(value) = kind.to_decimal() {
                data.push(value);
            }
        }
        data.sort();
        data
    }
    // Identify the resp type variant
    // Parameters
    // - The decimal value of the resp type. See https://valkey.io/topics/protocol
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
            33 => Self::BulkErrors,
            61 => Self::VerbatimStrings,
            37 => Self::Maps,
            _ => {
                return Err(anyhow!("NOT_SUPPORTED"));
            }
        };
        Ok(value)
    }

    // Converts the resp type variant to decimal
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
            Self::BulkErrors => 33,
            Self::Maps => 37,
            Self::VerbatimStrings => 61,
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
