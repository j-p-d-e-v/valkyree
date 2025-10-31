use crate::builder::resp_data_type::RespDataTypeTrait;
use crate::builder::resp_data_type::helpers::get_resp_multi_values;
use crate::types::{RespDataTypeValue, RespErrorKind};
use anyhow::anyhow;

#[derive(Debug)]
pub struct BulkErrors<'a> {
    value: &'a [u8],
    length: usize,
}

impl<'a> RespDataTypeTrait<'a> for BulkErrors<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }
    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (start, length, id) = get_resp_multi_values(self.value)?;
        if !id.is_bulk_errors() {
            return Err(anyhow!("NOT_BULK_ERRORS_TYPE"));
        }
        if length == 0 {
            return Ok(RespDataTypeValue::String("".to_string()));
        } else if length <= -1 {
            return Ok(RespDataTypeValue::Null);
        }
        let end = start + length as usize;
        let data = String::from_utf8_lossy(&self.value[start..end]).to_string();
        let result = RespErrorKind::parse(data);
        self.length = end + 2;
        Ok(result)
    }
}
#[cfg(test)]
pub mod test_bulk_errors {
    use super::*;

    #[test]
    fn test_errors() {
        struct TestCase {
            pub input: Vec<u8>,
            pub expected: RespDataTypeValue,
        }

        let test_cases = vec![
            TestCase {
                // !21\r\nWRONGTYPE Operation\r\n
                input: vec![
                    33, // '!' identifier for Bulk Errors
                    50, 49, 13, 10, // "21\r\n" → length = 21
                    87, 82, 79, 78, 71, 84, 89, 80, 69, 32, // "WRONGTYPE "
                    79, 112, 101, 114, 97, 116, 105, 111, 110, // "Operation"
                    13, 10, // \r\n
                ],
                expected: RespDataTypeValue::Error(
                    RespErrorKind::WrongType,
                    "Operation".to_string(),
                ),
            },
            TestCase {
                // !11\r\nOOM command\r\n
                input: vec![
                    33, // '!'
                    49, 49, 13, 10, // "11\r\n" → length = 11
                    79, 79, 77, 32, 99, 111, 109, 109, 97, 110, 100, // "OOM command"
                    13, 10, // \r\n
                ],
                expected: RespDataTypeValue::Error(RespErrorKind::Oom, "command".to_string()),
            },
            TestCase {
                // !44\r\nNOAUTH Authentication required or token expired\r\n
                input: vec![
                    33, // '!'
                    52, 55, 13, 10, // "47\r\n"
                    78, 79, 65, 85, 84, 72, 32, // "NOAUTH "
                    65, 117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 105, 111, 110,
                    32, // "Authentication "
                    114, 101, 113, 117, 105, 114, 101, 100, 32, // "required "
                    111, 114, 32, // "or "
                    116, 111, 107, 101, 110, 32, // "token "
                    101, 120, 112, 105, 114, 101, 100, // "expired"
                    13, 10,
                ],
                expected: RespDataTypeValue::Error(
                    RespErrorKind::NoAuth,
                    "Authentication required or token expired".to_string(),
                ),
            },
            TestCase {
                // !56\r\nNOPERM this user has no permissions to run 'FLUSHDB' on db 0\r\n
                input: vec![
                    33, // '!'
                    54, 48, 13, 10, // "60\r\n"
                    78, 79, 80, 69, 82, 77, 32, // "NOPERM "
                    116, 104, 105, 115, 32, 117, 115, 101, 114, 32, 104, 97, 115, 32, 110, 111, 32,
                    112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 32, 116, 111, 32, 114,
                    117, 110, 32, // "this user has no permissions to run "
                    39, 70, 76, 85, 83, 72, 68, 66, 39, 32, // "'FLUSHDB' "
                    111, 110, 32, 100, 98, 32, 48, // "on db 0"
                    13, 10,
                ],
                expected: RespDataTypeValue::Error(
                    RespErrorKind::NoPerm,
                    "this user has no permissions to run 'FLUSHDB' on db 0".to_string(),
                ),
            },
            TestCase {
                // !47\r\nERR module: invalid argument `--since=abc123`\r\n
                input: vec![
                    33, // '!'
                    52, 53, 13, 10, // "45\r\n"
                    69, 82, 82, 32, 109, 111, 100, 117, 108, 101, 58, 32, // "ERR module: "
                    105, 110, 118, 97, 108, 105, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116,
                    32, // "invalid argument "
                    96, 45, 45, 115, 105, 110, 99, 101, 61, 97, 98, 99, 49, 50, 51,
                    96, // "`--since=abc123`"
                    13, 10,
                ],
                expected: RespDataTypeValue::Error(
                    RespErrorKind::Err,
                    "module: invalid argument `--since=abc123`".to_string(),
                ),
            },
            TestCase {
                // !3\r\nERR\r\n
                input: vec![
                    33, // '!'
                    51, 13, 10, // "3\r\n"
                    69, 82, 82, // "ERR"
                    13, 10,
                ],
                expected: RespDataTypeValue::Error(RespErrorKind::Err, "".to_string()),
            },
            TestCase {
                // !43\r\nERR auth@ldap/user_not_found_in_directory\r\n
                input: vec![
                    33, // '!'
                    52, 49, 13, 10, // "43\r\n"
                    69, 82, 82, 32, // "ERR "
                    97, 117, 116, 104, 64, 108, 100, 97, 112, 47, // "auth@ldap/"
                    117, 115, 101, 114, 95, 110, 111, 116, 95, 102, 111, 117, 110, 100, 95, 105,
                    110, 95, 100, 105, 114, 101, 99, 116, 111, 114,
                    121, // "user_not_found_in_directory"
                    13, 10,
                ],
                expected: RespDataTypeValue::Error(
                    RespErrorKind::Err,
                    "auth@ldap/user_not_found_in_directory".to_string(),
                ),
            },
        ];
        for test_case in test_cases {
            let mut bulk_errors = BulkErrors::new(&test_case.input);
            let result = bulk_errors.build();
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }
}
