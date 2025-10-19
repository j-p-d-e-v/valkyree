use crate::types::Value;

#[derive(Debug)]
pub struct Doubles {}

impl Doubles {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let value = String::from_utf8_lossy(value).to_string();
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
    use super::*;

    #[test]
    fn test_infinity_nan() {
        struct TestCase<'a> {
            pub input: &'a [u8],
            pub expected: Value,
        }
        let test_cases = vec![
            TestCase {
                // ,inf\r\n
                input: &[105, 110, 102],
                expected: Value::Infinity,
            },
            TestCase {
                // ,-inf\r\n
                input: &[45, 105, 110, 102],
                expected: Value::NegativeInfinity,
            },
            TestCase {
                // ,nan\r\n
                input: &[110, 97, 110],
                expected: Value::Nan,
            },
        ];
        for test_case in test_cases {
            let result = Doubles::build(test_case.input);
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }

    #[test]
    fn test_finites() {
        struct TestCase<'a> {
            pub input: &'a [u8],
            pub expected: Value,
        }

        let test_cases = vec![
            TestCase {
                input: &[48],
                expected: Value::Double(0.0),
            }, // 0
            TestCase {
                input: &[49],
                expected: Value::Double(1.0),
            }, // 1
            TestCase {
                input: &[45, 49],
                expected: Value::Double(-1.0),
            }, // -1
            TestCase {
                input: &[51, 46, 49, 52, 49, 53, 57], // "3.14159"
                expected: Value::Double(3.14159),
            },
            TestCase {
                input: &[45, 48, 46, 48, 48, 49], // "-0.001"
                expected: Value::Double(-0.001),
            },
            TestCase {
                input: &[49, 50, 51, 52, 53, 54, 46, 55, 56, 57], // "123456.789"
                expected: Value::Double(123_456.789),
            },
            TestCase {
                input: &[49, 46, 48], // "1.0"
                expected: Value::Double(1.0),
            },
            TestCase {
                input: &[48, 46, 48], // "0.0"
                expected: Value::Double(0.0),
            },
            TestCase {
                input: &[49, 48], // "10"
                expected: Value::Double(10.0),
            },
            TestCase {
                input: &[49, 101, 51], // "1e3"
                expected: Value::Double(1000.0),
            },
            TestCase {
                input: &[45, 50, 46, 53, 101, 45, 51], // "-2.5e-3"
                expected: Value::Double(-0.0025),
            },
            TestCase {
                input: &[54, 46, 48, 50, 50, 101, 50, 51], // "6.022e23"
                expected: Value::Double(6.022e23),
            },
            TestCase {
                input: &[49, 69, 45, 55], // "1E-7"
                expected: Value::Double(1e-7),
            },
            TestCase {
                input: &[45, 48], // "-0"
                expected: Value::Double(-0.0),
            },
        ];

        for test_case in test_cases {
            let result = Doubles::build(test_case.input);
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
