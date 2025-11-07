use crate::builder::resp_data_type::RespDataTypeTrait;
use crate::builder::resp_data_type::RespParser;
use crate::builder::resp_data_type::helpers::get_resp_multi_values;
use crate::builder::resp_data_type::helpers::is_cr;
use crate::builder::resp_data_type::helpers::is_lf;
use crate::types::RespDataTypeValue;
use crate::types::resp_data_kind::RespDataType;
use crate::types::resp_data_type_iter::RespDataTypeIterator;
use anyhow::anyhow;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Maps<'a> {
    value: &'a [u8],
    length: usize,
}
impl<'a> Maps<'a> {
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
impl<'a> RespDataTypeTrait<'a> for Maps<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }

    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (start, length, main_id) = get_resp_multi_values(self.value)?;
        if !main_id.is_maps() {
            return Err(anyhow!("NOT_MAPS_TYPE"));
        }
        let mut data: BTreeMap<RespDataTypeValue, RespDataTypeValue> = BTreeMap::new();
        self.length += start;
        if length > 0 {
            self.value = &self.value[start..];
            let mut kv_data: Vec<RespDataTypeValue> = Vec::new();
            loop {
                let id = if let Some(v) = self.value.first()
                    && let Ok(kind) = RespDataType::identify(*v)
                {
                    kind
                } else {
                    return Err(anyhow!("INVALID_RESP_TYPE_ID"));
                };
                let mut iter = RespDataTypeIterator::new(self.value);

                if id.is_maps() {
                    let result = self.build()?;
                    kv_data.push(result);
                } else if id.is_bulk_strings()
                    || id.is_verbatim_strings()
                    || id.is_arrays()
                    || id.is_bulk_errors()
                {
                    let _ = self.set_data(&mut RespParser::new(self.value), &mut kv_data)?;
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
                            let _ =
                                self.set_data(&mut RespParser::new(&tmp_holder), &mut kv_data)?;
                            tmp_holder = vec![];
                            break;
                        }
                    }
                    if !tmp_holder.is_empty() {
                        let _ = self.set_data(&mut RespParser::new(&tmp_holder), &mut kv_data)?;
                    }
                }
                if let Some(key) = kv_data.get(0)
                    && let Some(value) = kv_data.get(1)
                {
                    let total_keys = data.keys().filter(|k| k == &key).count();
                    if total_keys > 1 {
                        return Err(anyhow!(
                            "DUPLICATE_MAPS_KEY: key: {:?}, value: {:?}",
                            key,
                            value
                        ));
                    }
                    data.insert(key.to_owned(), value.to_owned());
                    kv_data = vec![];
                }
                if data.len() == length as usize {
                    break;
                }
            }
        }
        // TODO: Issue cannot serialize to pretty json
        //println!("DATA: {}", serde_json::to_string_pretty(&data).unwrap());
        Ok(RespDataTypeValue::Object(data))
    }
}

#[cfg(test)]
pub mod test_maps {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_maps() {
        struct TestCase {
            pub id: u8,
            pub input: Vec<u8>,
            pub expected: RespDataTypeValue,
            pub is_error: bool,
        }

        let test_cases = vec![
            // 1) Empty map: %0\r\n
            TestCase {
                // %0\r\n
                id: 1,
                input: vec![
                    37, // '%'
                    48, 13, 10, // "0\r\n" → length = 0
                ],
                is_error: false,
                expected: RespDataTypeValue::Object(BTreeMap::new()),
            },
            // 2) Single pair: +a => :1
            TestCase {
                // %1\r\n+a\r\n:1\r\n
                id: 2,
                input: vec![
                    37, 49, 13, 10, // "%1\r\n"
                    43, 97, 13, 10, // "+a\r\n"
                    58, 49, 13, 10, // ":1\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Object(BTreeMap::from([(
                    RespDataTypeValue::String("a".to_string()),
                    RespDataTypeValue::Integer(1),
                )])),
            },
            // 3) Two pairs mixed: +a => :1, +b => $5\r\nhello\r\n
            TestCase {
                // %2\r\n +a\r\n :1\r\n +b\r\n $5\r\nhello\r\n
                id: 3,
                input: vec![
                    37, 50, 13, 10, // "%2\r\n"
                    43, 97, 13, 10, // "+a\r\n"
                    58, 49, 13, 10, // ":1\r\n"
                    43, 98, 13, 10, // "+b\r\n"
                    36, 53, 13, 10, // "$5\r\n"
                    104, 101, 108, 108, 111, 13, 10, // "hello\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Object(BTreeMap::from([
                    (
                        RespDataTypeValue::String("a".to_string()),
                        RespDataTypeValue::Integer(1),
                    ),
                    (
                        RespDataTypeValue::String("b".to_string()),
                        RespDataTypeValue::String("hello".to_string()),
                    ),
                ])),
            },
            // 4) Nested map value: +outer => (%1\r\n +inner\r\n +ok\r\n)
            TestCase {
                // %1\r\n +outer\r\n %1\r\n +inner\r\n +ok\r\n
                id: 4,
                input: vec![
                    37, 49, 13, 10, // "%1\r\n"
                    43, 111, 117, 116, 101, 114, 13, 10, // "+outer\r\n"
                    37, 49, 13, 10, // "%1\r\n"
                    43, 105, 110, 110, 101, 114, 13, 10, // "+inner\r\n"
                    43, 111, 107, 13, 10, // "+ok\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Object(BTreeMap::from([(
                    RespDataTypeValue::String("outer".to_string()),
                    RespDataTypeValue::Object(BTreeMap::from([(
                        RespDataTypeValue::String("inner".to_string()),
                        RespDataTypeValue::String("ok".to_string()),
                    )])),
                )])),
            },
            // 5) Array value: +nums => *3\r\n :1\r\n :2\r\n :3\r\n
            TestCase {
                // %1\r\n +nums\r\n *3\r\n :1\r\n :2\r\n :3\r\n
                id: 5,
                input: vec![
                    37, 49, 13, 10, // "%1\r\n"
                    43, 110, 117, 109, 115, 13, 10, // "+nums\r\n"
                    42, 51, 13, 10, // "*3\r\n"
                    58, 49, 13, 10, // ":1\r\n"
                    58, 50, 13, 10, // ":2\r\n"
                    58, 51, 13, 10, // ":3\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Object(BTreeMap::from([(
                    RespDataTypeValue::String("nums".to_string()),
                    RespDataTypeValue::Array(vec![
                        RespDataTypeValue::Integer(1),
                        RespDataTypeValue::Integer(2),
                        RespDataTypeValue::Integer(3),
                    ]),
                )])),
            },
            // 6) Mixed scalar types: +ok => #t, +pi => ,3.14159
            TestCase {
                // %2\r\n +ok\r\n #t\r\n +pi\r\n ,3.14159\r\n
                id: 6,
                input: vec![
                    37, 50, 13, 10, // "%2\r\n"
                    43, 111, 107, 13, 10, // "+ok\r\n"
                    35, 116, 13, 10, // "#t\r\n"
                    43, 112, 105, 13, 10, // "+pi\r\n"
                    44, 51, 46, 49, 52, 49, 53, 57, 13, 10, // ",3.14159\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Object(BTreeMap::from([
                    (
                        RespDataTypeValue::String("ok".to_string()),
                        RespDataTypeValue::Boolean(true),
                    ),
                    (
                        RespDataTypeValue::String("pi".to_string()),
                        RespDataTypeValue::Double(OrderedFloat(3.14159)),
                    ),
                ])),
            },
            // 7) Bulk string keys (including empty): "" => +empty, "key" => +val
            TestCase {
                // %2\r\n $0\r\n\r\n +empty\r\n $3\r\nkey\r\n +val\r\n
                id: 7,
                input: vec![
                    37, 50, 13, 10, // "%2\r\n"
                    36, 48, 13, 10, 13, 10, // "$0\r\n\r\n"
                    43, 101, 109, 112, 116, 121, 13, 10, // "+empty\r\n"
                    36, 51, 13, 10, 107, 101, 121, 13, 10, // "$3\r\nkey\r\n"
                    43, 118, 97, 108, 13, 10, // "+val\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Object(BTreeMap::from([
                    (
                        RespDataTypeValue::String("".to_string()),
                        RespDataTypeValue::String("empty".to_string()),
                    ),
                    (
                        RespDataTypeValue::String("key".to_string()),
                        RespDataTypeValue::String("val".to_string()),
                    ),
                ])),
            },
            // 8) Duplicate keys preserved (vector of pairs): +k => :1, +k => :2
            TestCase {
                // %2\r\n +k\r\n :1\r\n +k\r\n :2\r\n
                id: 8,
                input: vec![
                    37, 50, 13, 10, // "%2\r\n"
                    43, 107, 13, 10, // "+k\r\n"
                    58, 49, 13, 10, // ":1\r\n"
                    43, 107, 13, 10, // "+k\r\n"
                    58, 50, 13, 10, // ":2\r\n"
                ],
                is_error: true,
                expected: RespDataTypeValue::Object(BTreeMap::from([
                    (
                        RespDataTypeValue::String("k".to_string()),
                        RespDataTypeValue::Integer(1),
                    ),
                    (
                        RespDataTypeValue::String("k".to_string()),
                        RespDataTypeValue::Integer(2),
                    ),
                ])),
            },
            // 9) Larger, mixed container map
            // Pairs:
            // +i => :-42
            // +empty => _\r\n
            // +nums => *4\r\n :0 :1 :2 :3
            // +meta => %1\r\n +k\r\n +v\r\n
            // $5\r\nhello\r\n => $5\r\nworld\r\n
            TestCase {
                id: 9,
                input: vec![
                    37, 53, 13, 10, // "%5\r\n"
                    43, 105, 13, 10, // "+i\r\n"
                    58, 45, 52, 50, 13, 10, // ":-42\r\n"
                    43, 101, 109, 112, 116, 121, 13, 10, // "+empty\r\n"
                    95, 13, 10, // "_\r\n"
                    43, 110, 117, 109, 115, 13, 10, // "+nums\r\n"
                    42, 52, 13, 10, // "*4\r\n"
                    58, 48, 13, 10, // ":0\r\n"
                    58, 49, 13, 10, // ":1\r\n"
                    58, 50, 13, 10, // ":2\r\n"
                    58, 51, 13, 10, // ":3\r\n"
                    43, 109, 101, 116, 97, 13, 10, // "+meta\r\n"
                    37, 49, 13, 10, // "%1\r\n"
                    43, 107, 13, 10, // "+k\r\n"
                    43, 118, 13, 10, // "+v\r\n"
                    36, 53, 13, 10, 104, 101, 108, 108, 111, 13, 10, // "$5\r\nhello\r\n"
                    36, 53, 13, 10, 119, 111, 114, 108, 100, 13, 10, // "$5\r\nworld\r\n"
                ],
                is_error: false,
                expected: RespDataTypeValue::Object(BTreeMap::from([
                    (
                        RespDataTypeValue::String("i".to_string()),
                        RespDataTypeValue::Integer(-42),
                    ),
                    (
                        RespDataTypeValue::String("empty".to_string()),
                        RespDataTypeValue::Null,
                    ),
                    (
                        RespDataTypeValue::String("nums".to_string()),
                        RespDataTypeValue::Array(vec![
                            RespDataTypeValue::Integer(0),
                            RespDataTypeValue::Integer(1),
                            RespDataTypeValue::Integer(2),
                            RespDataTypeValue::Integer(3),
                        ]),
                    ),
                    (
                        RespDataTypeValue::String("meta".to_string()),
                        RespDataTypeValue::Object(BTreeMap::from([(
                            RespDataTypeValue::String("k".to_string()),
                            RespDataTypeValue::String("v".to_string()),
                        )])),
                    ),
                    (
                        RespDataTypeValue::String("hello".to_string()),
                        RespDataTypeValue::String("world".to_string()),
                    ),
                ])),
            },
        ];

        for test_case in test_cases {
            if ![1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&test_case.id) {
                continue;
            }
            let mut maps = Maps::new(&test_case.input);
            let result = maps.build();
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
