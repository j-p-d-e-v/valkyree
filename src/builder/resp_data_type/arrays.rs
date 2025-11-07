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
pub struct Arrays<'a> {
    value: &'a [u8],
    length: usize,
}
impl<'a> Arrays<'a> {
    fn set_data(
        &mut self,
        parser: &mut RespParser,
        data: &mut Vec<RespDataTypeValue>,
        length: isize,
    ) -> anyhow::Result<bool> {
        let result = parser.parse()?;
        data.push(result);
        self.length += parser.len();
        self.value = &self.value[parser.len()..];
        Ok(length == data.len() as isize)
    }
}
impl<'a> RespDataTypeTrait<'a> for Arrays<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }

    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (start, length, main_id) = get_resp_multi_values(self.value)?;
        if !main_id.is_arrays() {
            return Err(anyhow!("NOT_ARRAYS_TYPE"));
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
                if id.is_arrays() {
                    let result = self.build()?;
                    data.push(result);
                } else if id.is_bulk_strings()
                    || id.is_maps()
                    || id.is_verbatim_strings()
                    || id.is_bulk_errors()
                {
                    let _ = self.set_data(&mut RespParser::new(self.value), &mut data, length)?;
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
                            let is_break = self.set_data(
                                &mut RespParser::new(&tmp_holder),
                                &mut data,
                                length,
                            )?;
                            tmp_holder = vec![];
                            if is_break {
                                break;
                            }
                        }
                    }

                    if !tmp_holder.is_empty() {
                        let _ =
                            self.set_data(&mut RespParser::new(&tmp_holder), &mut data, length)?;
                    }
                }
                if data.len() == length as usize {
                    break;
                }
            }
        }
        Ok(RespDataTypeValue::Array(data))
    }
}

#[cfg(test)]
pub mod test_arrays {
    use super::*;

    #[test]
    fn test_arrays() {
        let identifier = RespDataType::Arrays.to_decimal().unwrap();

        struct TestCase {
            pub input: Vec<u8>,
            pub expected: RespDataTypeValue,
        }

        let test_cases = vec![
            TestCase {
                // *0\r\n
                // empty array
                input: vec![identifier, 48, 13, 10],
                expected: RespDataTypeValue::Array(vec![]),
            },
            TestCase {
                // *2\r\n+hello\r\n:5\r\n
                // [ "hello", 5 ]
                input: vec![
                    identifier, 50, 13, 10, // *2\r\n
                    43, 104, 101, 108, 108, 111, 13, 10, // +hello\r\n
                    58, 53, 13, 10, // :5\r\n
                ],
                expected: RespDataTypeValue::Array(vec![
                    RespDataTypeValue::String("hello".into()),
                    RespDataTypeValue::Integer(5),
                ]),
            },
            TestCase {
                // *3\r\n#t\r\n#f\r\n_\r\n
                // [ true, false, null ]
                input: vec![
                    identifier, 51, 13, 10, // *3\r\n
                    35, 116, 13, 10, // #t\r\n
                    35, 102, 13, 10, // #f\r\n
                    95, 13, 10, // _\r\n
                ],
                expected: RespDataTypeValue::Array(vec![
                    RespDataTypeValue::Boolean(true),
                    RespDataTypeValue::Boolean(false),
                    RespDataTypeValue::Null,
                ]),
            },
            TestCase {
                // *2\r\n*2\r\n:1\r\n:2\r\n*3\r\n+foo\r\n+bar\r\n+baz\r\n
                // [ [1, 2], ["foo", "bar", "baz"] ]
                input: vec![
                    identifier, 50, 13, 10, // *2\r\n
                    // inner array 1
                    42, 50, 13, 10, 58, 49, 13, 10, 58, 50, 13, 10, // *2\r\n:1\r\n:2\r\n
                    // inner array 2
                    42, 51, 13, 10, // *3\r\n
                    43, 102, 111, 111, 13, 10, // +foo\r\n
                    43, 98, 97, 114, 13, 10, // +bar\r\n
                    43, 98, 97, 122, 13, 10, // +baz\r\n
                ],
                expected: RespDataTypeValue::Array(vec![
                    RespDataTypeValue::Array(vec![
                        RespDataTypeValue::Integer(1),
                        RespDataTypeValue::Integer(2),
                    ]),
                    RespDataTypeValue::Array(vec![
                        RespDataTypeValue::String("foo".into()),
                        RespDataTypeValue::String("bar".into()),
                        RespDataTypeValue::String("baz".into()),
                    ]),
                ]),
            },
            TestCase {
                // *3\r\n
                //   *2\r\n
                //     *2\r\n
                //       +3\r\n1+1\r\n
                //       +5\r\n1:234\r\n
                //     $19\r\ncharlie+dev@work.net\r\n
                //   $5\r\nouter\r\n
                //
                // [
                //   [
                //     [ "1+1", "1:234" ],
                //     "charlie+dev@work.net"
                //   ],
                //   "outer1",
                //   "outer2",
                // ]
                input: vec![
                    42, 51, 13, 10, // *2\r\n           OUTER
                    42, 50, 13, 10, // *2\r\n           MIDDLE
                    42, 50, 13, 10, // *2\r\n           INNER
                    36, 51, 13, 10, // $3\r\n
                    49, 43, 49, 13, 10, // 1+1\r\n
                    36, 53, 13, 10, // $5\r\n
                    49, 58, 50, 51, 52, 13, 10, // 1:234\r\n
                    36, 50, 48, 13, 10, // $20\r\n
                    99, 104, 97, 114, 108, 105, 101, 43, 100, 101, 118, 64, 119, 111, 114, 107, 46,
                    110, 101, 116, 13, 10, // charlie+dev@work.net\r\n
                    43, 111, 117, 116, 101, 114, 49, 13, 10, // +outer1\r\n
                    43, 111, 117, 116, 101, 114, 50, 13, 10, // +outer2\r\n
                ],
                expected: RespDataTypeValue::Array(vec![
                    RespDataTypeValue::Array(vec![
                        RespDataTypeValue::Array(vec![
                            RespDataTypeValue::String("1+1".into()),
                            RespDataTypeValue::String("1:234".into()),
                        ]),
                        RespDataTypeValue::String("charlie+dev@work.net".into()),
                    ]),
                    RespDataTypeValue::String("outer1".into()),
                    RespDataTypeValue::String("outer2".into()),
                ]),
            },
        ];

        for test_case in test_cases {
            let mut arrays = Arrays::new(&test_case.input);
            let result = arrays.build();
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }
}
