use crate::{
    builder::resp_data_type::RespDataTypeBase,
    types::{resp_data_kind::RespDataType, Value},
};
use anyhow::anyhow;
use num_bigint::BigInt;
use regex::Regex;

#[derive(Debug)]
pub struct BigNumbers {}

impl RespDataTypeBase for BigNumbers {}
impl BigNumbers {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        Self::is_data_type(value, RespDataType::BigNumbers)?;
        let value = Self::get_value(value)?;
        let pattern = Regex::new(r"^-?[0-9]+$")?;
        if !pattern.is_match(String::from_utf8_lossy(&value).as_ref()) {
            return Err(anyhow!("BIG_NUMBERS_INVALID_VALUE".to_string()));
        }
        let parsed = if let Some(value) = BigInt::parse_bytes(&value, 10) {
            value
        } else {
            return Err(anyhow!("BIG_NUMBERS_PARSING_ERROR".to_string()));
        };
        Ok(Value::BigNumber(parsed))
    }
}

#[cfg(test)]
pub mod test_big_numbers {
    use crate::types::resp_data_kind::RespDataType;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_bigint_ok() {
        let identifier = RespDataType::BigNumbers.to_decimal().unwrap();

        struct TestCase {
            input: Vec<u8>, // '(' + ASCII digits with optional leading '-' + "\r\n"
            expect: BigInt,
        }

        let mut test_cases: Vec<TestCase> = vec![
            // "(" "0" "\r\n"
            TestCase {
                input: vec![identifier, 48, 13, 10],
                expect: BigInt::from(0),
            },
            // "(" "-0" "\r\n"  -> normalized to 0
            TestCase {
                input: vec![identifier, 45, 48, 13, 10],
                expect: BigInt::from(0),
            },
            // "(" "7" "\r\n"
            TestCase {
                input: vec![identifier, 55, 13, 10],
                expect: BigInt::from(7),
            },
            // "(" "-1" "\r\n"
            TestCase {
                input: vec![identifier, 45, 49, 13, 10],
                expect: BigInt::from(-1),
            },
            // "(" "0000" "\r\n"
            TestCase {
                input: vec![identifier, 48, 48, 48, 48, 13, 10],
                expect: BigInt::from(0),
            },
            // "(" "-0005" "\r\n" -> -5
            TestCase {
                input: vec![identifier, 45, 48, 48, 48, 53, 13, 10],
                expect: BigInt::from(-5),
            },
            // "(" "9223372036854775808" "\r\n" (i64::MAX + 1)
            TestCase {
                input: {
                    let mut v = vec![identifier];
                    v.extend_from_slice(&[
                        57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 56,
                    ]);
                    v.extend_from_slice(&[13, 10]);
                    v
                },
                expect: BigInt::from_str("9223372036854775808").unwrap(),
            },
            // "(" "-18446744073709551616" "\r\n" (-(u64::MAX + 1))
            TestCase {
                input: {
                    let mut v = vec![identifier];
                    v.extend_from_slice(&[
                        45, 49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54,
                        49, 54,
                    ]);
                    v.extend_from_slice(&[13, 10]);
                    v
                },
                expect: BigInt::from_str("-18446744073709551616").unwrap(),
            },
        ];

        // Very large, built at runtime
        let huge_pos = "9".repeat(256); // 256-digit positive
        let huge_neg = format!("-{}", "8".repeat(300)); // 300-digit negative

        test_cases.push(TestCase {
            input: {
                let mut v = vec![identifier];
                v.extend_from_slice(huge_pos.as_bytes());
                v.extend_from_slice(&[13, 10]);
                v
            },
            expect: BigInt::from_str(&huge_pos).unwrap(),
        });

        test_cases.push(TestCase {
            input: {
                let mut v = vec![identifier];
                v.extend_from_slice(huge_neg.as_bytes());
                v.extend_from_slice(&[13, 10]);
                v
            },
            expect: BigInt::from_str(&huge_neg).unwrap(),
        });

        for tc in test_cases {
            let got = BigNumbers::build(&tc.input); // your parser for '(' payload
            assert!(
                got.is_ok(),
                "parse error for {:?}: {:#?}",
                tc.input,
                got.err()
            );
            assert_eq!(
                Value::BigNumber(tc.expect),
                got.unwrap(),
                "mismatch for {:?}",
                tc.input
            );
        }
    }

    #[test]
    fn test_bigint_invalid() {
        let identifier = RespDataType::BigNumbers.to_decimal().unwrap();

        // Invalid RESP3 Big Number payloads:
        // base-10 only, optional single leading '-', then digits only; at least one digit.
        let invalid_payloads: &[&[u8]] = &[
            &[],                       // empty
            &[45],                     // "-" only
            &[43, 49, 50, 51],         // "+123" (plus sign not allowed)
            &[32, 49, 50, 51],         // " 123" (leading space)
            &[49, 50, 51, 32],         // "123 " (trailing space)
            &[49, 50, 97, 51],         // "12a3" (non-digit)
            &[49, 95, 48, 48, 48],     // "1_000" (underscore not allowed)
            &[45, 45, 53],             // "--5" (double minus)
            &[45, 43, 53],             // "-+5" (mixed sign)
            &[45, 32, 53],             // "- 5" (space after sign)
            &[49, 46, 50],             // "1.2" (decimal dot not allowed)
            &[49, 101, 51],            // "1e3" (scientific notation not allowed)
            &[48, 120, 102, 102],      // "0xff" (hex not allowed)
            &[48, 98, 49, 48, 49, 48], // "0b1010" (binary not allowed)
        ];

        for payload in invalid_payloads {
            let input = {
                let mut v = vec![identifier];
                v.extend_from_slice(payload);
                v.extend_from_slice(&[13, 10]);
                v
            };
            let got = BigNumbers::build(&input);
            assert!(
                got.is_err(),
                "should fail for invalid payload {:?} (framed {:?})",
                payload,
                input
            );
        }
    }
}
