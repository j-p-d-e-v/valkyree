use crate::{
    builder::resp_data_type::RespDataTypeBase,
    types::{Value, resp_data_kind::RespDataType},
};

#[derive(Debug)]
pub struct Doubles {}

impl RespDataTypeBase for Doubles {}
impl Doubles {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        Self::is_data_type(value, RespDataType::Doubles)?;
        let value = Self::get_value(value, true)?;
        let value = String::from_utf8_lossy(&value).to_string();
        let parsed = value.parse::<f64>()?;
        let result = if parsed.is_nan() {
            Value::Nan
        } else if parsed.is_infinite() && parsed.is_sign_positive() {
            Value::Infinity
        } else if parsed.is_infinite() && parsed.is_sign_negative() {
            Value::NegativeInfinity
        } else {
            Value::Double(parsed)
        };

        Ok(result)
    }
}

#[cfg(test)]
pub mod test_doubles {
    use crate::types::resp_data_kind::RespDataType;

    use super::*;

    #[test]
    fn test_infinity_nan() {
        struct TestCase {
            pub input: Vec<u8>,
            pub expected: Value,
        }
        let identifier = RespDataType::Doubles.to_decimal().unwrap();
        let test_cases = vec![
            TestCase {
                // ,inf\r\n
                input: vec![identifier, 105, 110, 102, 13, 10],
                expected: Value::Infinity,
            },
            TestCase {
                // ,-inf\r\n
                input: vec![identifier, 45, 105, 110, 102, 13, 10],
                expected: Value::NegativeInfinity,
            },
            TestCase {
                // ,nan\r\n
                input: vec![identifier, 110, 97, 110, 13, 10],
                expected: Value::Nan,
            },
        ];
        for test_case in test_cases {
            let result = Doubles::build(&test_case.input);
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }

    #[test]
    fn test_finites() {
        struct TestCase {
            pub input: Vec<u8>,
            pub expected: Value,
        }
        let identifier = RespDataType::Doubles.to_decimal().unwrap();
        let test_cases = vec![
            TestCase {
                // ,0\r\n
                input: vec![identifier, 48, 13, 10],
                expected: Value::Double(0.0),
            },
            TestCase {
                // ,1\r\n
                input: vec![identifier, 49, 13, 10],
                expected: Value::Double(1.0),
            },
            TestCase {
                // ,-1\r\n
                input: vec![identifier, 45, 49, 13, 10],
                expected: Value::Double(-1.0),
            },
            TestCase {
                // ,3.14159\r\n
                input: vec![identifier, 51, 46, 49, 52, 49, 53, 57, 13, 10],
                expected: Value::Double(3.14159),
            },
            TestCase {
                // ,-0.001\r\n
                input: vec![identifier, 45, 48, 46, 48, 48, 49, 13, 10],
                expected: Value::Double(-0.001),
            },
            TestCase {
                // ,123456.789\r\n
                input: vec![identifier, 49, 50, 51, 52, 53, 54, 46, 55, 56, 57, 13, 10],
                expected: Value::Double(123_456.789),
            },
            TestCase {
                // ,1.0\r\n
                input: vec![identifier, 49, 46, 48, 13, 10],
                expected: Value::Double(1.0),
            },
            TestCase {
                // ,0.0\r\n
                input: vec![identifier, 48, 46, 48, 13, 10],
                expected: Value::Double(0.0),
            },
            TestCase {
                // ,10\r\n
                input: vec![identifier, 49, 48, 13, 10],
                expected: Value::Double(10.0),
            },
            TestCase {
                // ,1e3\r\n
                input: vec![identifier, 49, 101, 51, 13, 10],
                expected: Value::Double(1000.0),
            },
            TestCase {
                // ,-2.5e-3\r\n
                input: vec![identifier, 45, 50, 46, 53, 101, 45, 51, 13, 10],
                expected: Value::Double(-0.0025),
            },
            TestCase {
                // ,6.022e23\r\n
                input: vec![identifier, 54, 46, 48, 50, 50, 101, 50, 51, 13, 10],
                expected: Value::Double(6.022e23),
            },
            TestCase {
                // ,1E-7\r\n
                input: vec![identifier, 49, 69, 45, 55, 13, 10],
                expected: Value::Double(1e-7),
            },
            TestCase {
                // ,-0\r\n
                input: vec![identifier, 45, 48, 13, 10],
                expected: Value::Double(-0.0),
            },
        ];

        for test_case in test_cases {
            let result = Doubles::build(&test_case.input);
            assert!(
                result.is_ok(),
                "failed to parse {:?}: {:#?}",
                test_case.input,
                result.err()
            );
            assert_eq!(
                test_case.expected,
                result.unwrap(),
                "mismatch for input {:?}",
                test_case.input
            );
        }
    }
}
