use crate::builder::resp_data_type::RespDataTypeTrait;
use crate::builder::resp_data_type::RespParser;
use crate::builder::resp_data_type::helpers::get_resp_multi_values;
use crate::builder::resp_data_type::helpers::is_cr;
use crate::builder::resp_data_type::helpers::is_lf;
use crate::types::RespDataTypeValue;
use crate::types::resp_data_kind::RespDataType;
use crate::types::resp_data_type_iter::RespDataTypeIterator;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Sets<'a> {
    value: &'a [u8],
    length: usize,
}
impl<'a> Sets<'a> {
    fn set_data(
        &mut self,
        parser: &mut RespParser,
        data: &mut Vec<RespDataTypeValue>,
    ) -> anyhow::Result<()> {
        let result = parser.parse()?;
        if self.is_value_exists(data, &result) {
            return Err(anyhow!("VALUE_ALREADY_EXISTS: {:?}", result));
        }
        data.push(result);
        self.length += parser.len();
        self.value = &self.value[parser.len()..];
        Ok(())
    }

    fn is_value_exists(&self, data: &Vec<RespDataTypeValue>, value: &RespDataTypeValue) -> bool {
        data.contains(value)
    }
}

impl<'a> RespDataTypeTrait<'a> for Sets<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }

    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (start, length, main_id) = get_resp_multi_values(self.value)?;
        if !main_id.is_sets() {
            return Err(anyhow!("NOT_SETS_TYPE"));
        }
        self.length += start;
        let mut data: Vec<RespDataTypeValue> = Vec::new();
        if length > 0 {
            self.value = &self.value[start..];
            loop {
                let id = if let Some(v) = self.value.first()
                    && let Ok(kind) = RespDataType::identify(*v)
                {
                    kind
                } else {
                    return Err(anyhow!("INVALID_RESP_TYPE_ID"));
                };
                let mut iter = RespDataTypeIterator::new(self.value);
                if id.is_sets() {
                    let result = self.build()?;
                    data.push(result);
                } else if id.is_bulk_strings()
                    || id.is_maps()
                    || id.is_verbatim_strings()
                    || id.is_bulk_errors()
                    || id.is_arrays()
                {
                    let _ = self.set_data(&mut RespParser::new(self.value), &mut data)?;
                } else {
                    let mut tmp_holder: Vec<u8> = Vec::new();
                    while let Some(v) = iter.next() {
                        tmp_holder.push(*v);
                        if is_cr(v)
                            && let Some(peek_values) = iter.npeek(2)
                            && let Some(lf) = peek_values.first()
                            && is_lf(lf)
                            && let Some(next_id) = peek_values.get(1)
                            && RespDataType::identify(*next_id).is_ok()
                            && let Some(mut next_values) = iter.nnext(1)
                        {
                            tmp_holder.append(&mut next_values);

                            let _ = self.set_data(&mut RespParser::new(&tmp_holder), &mut data)?;
                            tmp_holder = vec![];
                            break;
                        }
                    }

                    if !tmp_holder.is_empty() {
                        let _ = self.set_data(&mut RespParser::new(&tmp_holder), &mut data)?;
                    }
                }
                if data.len() == length as usize {
                    break;
                }
            }
        }
        Ok(RespDataTypeValue::Set(data))
    }
}

#[cfg(test)]
pub mod test_sets {
    use std::collections::BTreeMap;

    use ordered_float::OrderedFloat;

    use super::*;

    #[test]
    fn test_sets() {
        struct TestCase {
            pub id: u8,
            pub input: Vec<u8>,
            pub expected: RespDataTypeValue,
            pub is_error: bool,
        }
        let test_cases: Vec<TestCase> = vec![
            // 1) Empty set: ~0\r\n
            TestCase {
                // ~0\r\n
                id: 1,
                input: vec![
                    126, // '~'
                    48, 13, 10, // "0\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(Vec::new()),
            },
            // 2) Two simple strings: ~2\r\n+foo\r\n+bar\r\n
            TestCase {
                // ~2\r\n+foo\r\n+bar\r\n
                id: 2,
                input: vec![
                    126, 50, 13, 10, // "~2\r\n"
                    43, 102, 111, 111, 13, 10, // +foo\r\n
                    43, 98, 97, 114, 13, 10, // +bar\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::String("foo".into()),
                    RespDataTypeValue::String("bar".into()),
                ]),
            },
            // 3) Mixed types (string, integer, double 3.14)
            TestCase {
                // ~3\r\n+apple\r\n:100\r\n,3.14\r\n
                id: 3,
                input: vec![
                    126, 51, 13, 10, // "~3\r\n"
                    43, 97, 112, 112, 108, 101, 13, 10, // +apple\r\n
                    58, 49, 48, 48, 13, 10, // :100\r\n
                    44, 51, 46, 49, 52, 13, 10, // ,3.14\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::String("apple".into()),
                    RespDataTypeValue::Integer(100),
                    RespDataTypeValue::Double(OrderedFloat(3.14)),
                ]),
            },
            // 4) Nested set
            TestCase {
                // ~2\r\n+outer\r\n~2\r\n+inner1\r\n+inner2\r\n
                id: 4,
                input: vec![
                    126, 50, 13, 10, // "~2\r\n"
                    43, 111, 117, 116, 101, 114, 13, 10, // +outer\r\n
                    126, 50, 13, 10, // "~2\r\n"
                    43, 105, 110, 110, 101, 114, 49, 13, 10, // +inner1\r\n
                    43, 105, 110, 110, 101, 114, 50, 13, 10, // +inner2\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::String("outer".into()),
                    RespDataTypeValue::Set(vec![
                        RespDataTypeValue::String("inner1".into()),
                        RespDataTypeValue::String("inner2".into()),
                    ]),
                ]),
            },
            // 5) Duplicate element (error)
            TestCase {
                // ~3\r\n+dup\r\n+dup\r\n+z\r\n
                id: 5,
                input: vec![
                    126, 51, 13, 10, // "~3\r\n"
                    43, 100, 117, 112, 13, 10, // +dup\r\n
                    43, 100, 117, 112, 13, 10, // +dup\r\n  <-- duplicate
                    43, 122, 13, 10, // +z\r\n
                ],
                is_error: true,
                // Your decoder should raise DuplicateSetMember (or similar).
                expected: RespDataTypeValue::Null,
            },
            // 6) Blob string + simple string
            TestCase {
                // ~2\r\n$5\r\nhello\r\n+world\r\n
                id: 6,
                input: vec![
                    126, 50, 13, 10, // "~2\r\n"
                    36, 53, 13, 10, // $5\r\n
                    104, 101, 108, 108, 111, 13, 10, // hello\r\n
                    43, 119, 111, 114, 108, 100, 13, 10, // +world\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    // If you keep Blob vs Simple distinct, swap this to BlobString(..)
                    RespDataTypeValue::String("hello".into()),
                    RespDataTypeValue::String("world".into()),
                ]),
            },
            // 7) Booleans
            TestCase {
                // ~2\r\n#t\r\n#f\r\n
                id: 7,
                input: vec![
                    126, 50, 13, 10, // "~2\r\n"
                    35, 116, 13, 10, // #t\r\n
                    35, 102, 13, 10, // #f\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::Boolean(true),
                    RespDataTypeValue::Boolean(false),
                ]),
            },
            // 8) Doubles with special values: inf, -inf, nan
            TestCase {
                // ~3\r\n,inf\r\n,-inf\r\n,nan\r\n
                id: 8,
                input: vec![
                    126, 51, 13, 10, // "~3\r\n"
                    44, 105, 110, 102, 13, 10, // ,inf\r\n
                    44, 45, 105, 110, 102, 13, 10, // ,-inf\r\n
                    44, 110, 97, 110, 13, 10, // ,nan\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::Infinity,
                    RespDataTypeValue::NegativeInfinity,
                    RespDataTypeValue::Nan,
                ]),
            },
            // 9) Set containing a small map and a string
            TestCase {
                // ~2\r\n%1\r\n+key\r\n+value\r\n+solo\r\n
                id: 9,
                input: vec![
                    126, 50, 13, 10, // "~2\r\n"
                    37, 49, 13, 10, // %1\r\n
                    43, 107, 101, 121, 13, 10, // +key\r\n
                    43, 118, 97, 108, 117, 101, 13, 10, // +value\r\n
                    43, 115, 111, 108, 111, 13, 10, // +solo\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::Object(BTreeMap::from([(
                        RespDataTypeValue::String("key".into()),
                        RespDataTypeValue::String("value".into()),
                    )])),
                    RespDataTypeValue::String("solo".into()),
                ]),
            },
            // 10) Order A (alpha, beta, gamma)
            TestCase {
                // ~3\r\n+alpha\r\n+beta\r\n+gamma\r\n
                id: 10,
                input: vec![
                    126, 51, 13, 10, // "~3\r\n"
                    43, 97, 108, 112, 104, 97, 13, 10, // +alpha\r\n
                    43, 98, 101, 116, 97, 13, 10, // +beta\r\n
                    43, 103, 97, 109, 109, 97, 13, 10, // +gamma\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::String("alpha".into()),
                    RespDataTypeValue::String("beta".into()),
                    RespDataTypeValue::String("gamma".into()),
                ]),
            },
            // 11) Order B (gamma, alpha, beta) — same logical set, different wire order
            TestCase {
                // ~3\r\n+gamma\r\n+alpha\r\n+beta\r\n
                id: 11,
                input: vec![
                    126, 51, 13, 10, // "~3\r\n"
                    43, 103, 97, 109, 109, 97, 13, 10, // +gamma\r\n
                    43, 97, 108, 112, 104, 97, 13, 10, // +alpha\r\n
                    43, 98, 101, 116, 97, 13, 10, // +beta\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::String("gamma".into()),
                    RespDataTypeValue::String("alpha".into()),
                    RespDataTypeValue::String("beta".into()),
                ]),
            },
            // 12) Set with array member and a string
            TestCase {
                // ~2\r\n*2\r\n+one\r\n+two\r\n+solo\r\n
                id: 12,
                input: vec![
                    126, 50, 13, 10, // "~2\r\n"
                    42, 50, 13, 10, // *2\r\n
                    43, 111, 110, 101, 13, 10, // +one\r\n
                    43, 116, 119, 111, 13, 10, // +two\r\n
                    43, 115, 111, 108, 111, 13, 10, // +solo\r\n
                ],
                is_error: false,
                expected: RespDataTypeValue::Set(vec![
                    RespDataTypeValue::Array(vec![
                        RespDataTypeValue::String("one".into()),
                        RespDataTypeValue::String("two".into()),
                    ]),
                    RespDataTypeValue::String("solo".into()),
                ]),
            },
        ];

        for test_case in test_cases {
            if ![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].contains(&test_case.id) {
                continue;
            }
            let mut sets = Sets::new(&test_case.input);
            let result = sets.build();
            if test_case.is_error {
                assert!(result.is_err());
                eprintln!("EXPECTED ERROR: {:?}", result.err());
            } else {
                assert!(result.is_ok(), "{:#?}", result.err());
                assert_eq!(test_case.expected, result.unwrap());
            }
        }
    }
}
