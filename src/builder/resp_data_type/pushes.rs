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
pub struct Pushes<'a> {
    value: &'a [u8],
    length: usize,
}
impl<'a> Pushes<'a> {
    fn set_data(
        &mut self,
        parser: &mut RespParser,
        data: &mut Vec<RespDataTypeValue>,
    ) -> anyhow::Result<()> {
        let result = parser.parse()?;
        data.push(result);
        self.length += parser.len();
        self.value = &self.value[parser.len()..];
        Ok(())
    }
}
impl<'a> RespDataTypeTrait<'a> for Pushes<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }

    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (start, length, main_id) = get_resp_multi_values(self.value)?;
        if !main_id.is_pushes() {
            return Err(anyhow!("NOT_PUSHES_TYPE"));
        }
        self.length += start;
        let mut data: Vec<RespDataTypeValue> = Vec::new();
        if length < 0 {
            return Err(anyhow!("PUSHES_CANNOT_BE_NULL"));
        } else if length > 0 {
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
                if id.is_pushes() {
                    let result = self.build()?;
                    data.push(result);
                } else if id.is_bulk_strings()
                    || id.is_maps()
                    || id.is_arrays()
                    || id.is_verbatim_strings()
                    || id.is_bulk_errors()
                    || id.is_sets()
                {
                    self.set_data(&mut RespParser::new(self.value), &mut data)?;
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
                            self.set_data(&mut RespParser::new(&tmp_holder), &mut data)?;
                            tmp_holder = vec![];
                            break;
                        }
                    }

                    if !tmp_holder.is_empty() {
                        self.set_data(&mut RespParser::new(&tmp_holder), &mut data)?;
                    }
                }
                if data.len() == length as usize {
                    break;
                }
            }
        }
        Ok(RespDataTypeValue::Push(data))
    }
}

#[cfg(test)]
pub mod test_pushes {
    use std::collections::BTreeMap;

    use ordered_float::OrderedFloat;

    use super::*;

    #[test]
    fn test_pushes() {
        struct TestCase {
            pub id: u8,
            pub input: Vec<u8>,
            pub expected: RespDataTypeValue,
            pub is_error: bool,
        }
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                // 1) Empty push: >0\r\n
                id: 1,
                input: vec![
                    62, // '>'
                    48, 13, 10, // "0\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Push(vec![]),
            },
            TestCase {
                // >3\r\n +message\r\n +somechan\r\n +hello\r\n
                id: 2,
                input: vec![
                    62, 51, 13, 10, // ">3\r\n"
                    43, 109, 101, 115, 115, 97, 103, 101, 13, 10, // "+message\r\n"
                    43, 115, 111, 109, 101, 99, 104, 97, 110, 13, 10, // "+somechan\r\n"
                    43, 104, 101, 108, 108, 111, 13, 10, // "+hello\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Push(vec![
                    RespDataTypeValue::String("message".to_string()),
                    RespDataTypeValue::String("somechan".to_string()),
                    RespDataTypeValue::String("hello".to_string()),
                ]),
            },
            TestCase {
                // >3\r\n +message\r\n +mychannel\r\n $5\r\nhello\r\n
                id: 3,
                input: vec![
                    62, 51, 13, 10, // ">3\r\n"
                    43, 109, 101, 115, 115, 97, 103, 101, 13, 10, // "+message\r\n"
                    43, 109, 121, 99, 104, 97, 110, 110, 101, 108, 13, 10, // "+mychannel\r\n"
                    36, 53, 13, 10, 104, 101, 108, 108, 111, 13, 10, // "$5\r\nhello\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Push(vec![
                    RespDataTypeValue::String("message".to_string()),
                    RespDataTypeValue::String("mychannel".to_string()),
                    RespDataTypeValue::String("hello".to_string()),
                ]),
            },
            TestCase {
                // >2\r\n +meta\r\n *2\r\n :1\r\n :2\r\n
                id: 4,
                input: vec![
                    62, 50, 13, 10, // ">2\r\n"
                    43, 109, 101, 116, 97, 13, 10, // "+meta\r\n"
                    42, 50, 13, 10, // "*2\r\n"
                    58, 49, 13, 10, // ":1\r\n"
                    58, 50, 13, 10, // ":2\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Push(vec![
                    RespDataTypeValue::String("meta".to_string()),
                    RespDataTypeValue::Array(vec![
                        RespDataTypeValue::Integer(1),
                        RespDataTypeValue::Integer(2),
                    ]),
                ]),
            },
            TestCase {
                // >4\r\n :1\r\n ,3.14\r\n #t\r\n _\r\n
                id: 5,
                input: vec![
                    62, 52, 13, 10, // ">4\r\n"
                    58, 49, 13, 10, // ":1\r\n"
                    44, 51, 46, 49, 52, 13, 10, // ",3.14\r\n"
                    35, 116, 13, 10, // "#t\r\n"
                    95, 13, 10, // "_\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Push(vec![
                    RespDataTypeValue::Integer(1),
                    RespDataTypeValue::Double(OrderedFloat(3.14)),
                    RespDataTypeValue::Boolean(true),
                    RespDataTypeValue::Null,
                ]),
            },
            TestCase {
                // >2\r\n +outer\r\n >1\r\n +inner\r\n
                id: 6,
                input: vec![
                    62, 50, 13, 10, // ">2\r\n"
                    43, 111, 117, 116, 101, 114, 13, 10, // "+outer\r\n"
                    62, 49, 13, 10, // ">1\r\n"
                    43, 105, 110, 110, 101, 114, 13, 10, // "+inner\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Push(vec![
                    RespDataTypeValue::String("outer".to_string()),
                    RespDataTypeValue::Push(vec![RespDataTypeValue::String("inner".to_string())]),
                ]),
            },
            TestCase {
                // >-1\r\n invalid
                id: 7,
                input: vec![
                    62, // '>'
                    45, 49, 13, 10, // "-1\r\n"
                ],
                is_error: true,
                expected: RespDataTypeValue::Null,
            },
            TestCase {
                // >2\r\n +onlyone\r\n  (missing second element)
                id: 8,
                input: vec![
                    62, 50, 13, 10, // ">2\r\n"
                    43, 111, 110, 108, 121, 111, 110, 101, 13,
                    10, // "+onlyone\r\n"
                        // missing second element
                ],
                is_error: true,
                expected: RespDataTypeValue::Null,
            },
            TestCase {
                // >3\r\n
                // %1\r\n
                // +key\r\n
                // :1\r\n
                // ~2\r\n
                // +member1\r\n
                // +member2\r\n
                // +done\r\n
                id: 9,
                input: vec![
                    62, 51, 13, 10, // ">3\r\n"
                    37, 49, 13, 10, // "%1\r\n"
                    43, 107, 101, 121, 13, 10, // "+key\r\n"
                    58, 49, 13, 10, // ":1\r\n"
                    126, 50, 13, 10, // "~2\r\n"
                    43, 109, 101, 109, 98, 101, 114, 49, 13, 10, // "+member1\r\n"
                    43, 109, 101, 109, 98, 101, 114, 50, 13, 10, // "+member2\r\n"
                    43, 100, 111, 110, 101, 13, 10, // "+done\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Push(vec![
                    RespDataTypeValue::Object(BTreeMap::from([(
                        RespDataTypeValue::String("key".to_string()),
                        RespDataTypeValue::Integer(1),
                    )])),
                    RespDataTypeValue::Set(vec![
                        RespDataTypeValue::String("member1".to_string()),
                        RespDataTypeValue::String("member2".to_string()),
                    ]),
                    RespDataTypeValue::String("done".to_string()),
                ]),
            },
            TestCase {
                // >2\r\n
                // $12\r\n
                // line1\r\nline2\r\n
                // +done\r\n
                //
                // Bulk content is "line1\r\nline2" (12 bytes)
                id: 10,
                input: vec![
                    62, 50, 13, 10, // ">2\r\n"
                    36, 49, 50, 13, 10, // "$12\r\n"
                    108, 105, 110, 101, 49, // "line1"
                    13, 10, // "\r\n"
                    108, 105, 110, 101, 50, // "line2"
                    13, 10, // "\r\n" terminator
                    43, 100, 111, 110, 101, 13, 10, // "+done\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Push(vec![
                    RespDataTypeValue::String("line1\r\nline2".to_string()),
                    RespDataTypeValue::String("done".to_string()),
                ]),
            },
        ];

        for test_case in test_cases {
            if ![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].contains(&test_case.id) {
                continue;
            }
            let mut sets = Pushes::new(&test_case.input);
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
