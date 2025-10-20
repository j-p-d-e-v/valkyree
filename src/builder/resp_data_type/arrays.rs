use crate::builder::resp_data_type::RespDataTypeBase;
use crate::builder::resp_data_type::RespParser;
use crate::builder::resp_data_type::helpers::get_length;
use crate::types::RespDataTypeValue;
use crate::types::resp_data_kind::RespDataType;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Arrays {}

impl RespDataTypeBase for Arrays {}
impl Arrays {
    fn final_push_data(tmp_data: &mut Vec<u8>, iter: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>) {
        tmp_data.push(13);
        tmp_data.push(10);
        iter.next();
        iter.next();
    }

    fn is_array_identifier(id: &u8) -> bool{
        if let Ok(array_id) = RespDataType::Arrays.to_decimal() && &array_id == id {
            true
        } else { 
            false 
        }
    }

    pub fn split_resp_types(value: &[u8]) -> Vec<Vec<u8>> {
        let identifiers = RespDataType::get_identifiers_decimals();
        let mut tmp_data: Vec<u8> = Vec::new();
        let mut data: Vec<Vec<u8>> = Vec::new();
        let mut iter = value.into_iter().peekable();
        let mut peek_iter = value.iter();        
        let main_id_is_array = if let Some(starting_id) = peek_iter.next() { Self::is_array_identifier(starting_id) } else { false };
        while let Some(v) = iter.next() {
            tmp_data.push(*v);
            let mut peek = iter.clone();
            if let Some(cr) = peek.next() && cr == &13 &&
                let Some(lf) = peek.next() && lf == &10 &&
                let Some(id) = peek.next() && identifiers.contains(&id) {
                let next_id_is_array = Self::is_array_identifier(id);
                if main_id_is_array {
                    if next_id_is_array {
                        Self::final_push_data(&mut tmp_data, &mut iter);
                        data.push(tmp_data);
                        tmp_data = Vec::new();
                    }
                }
                else {
                    Self::final_push_data(&mut tmp_data, &mut iter);
                    data.push(tmp_data);
                    tmp_data = Vec::new();
                }                
            }
        }
        if !tmp_data.is_empty() {
            data.push(tmp_data);
        }
        data
    }
    pub fn build(value: &[u8]) -> anyhow::Result<RespDataTypeValue> {
        Self::is_data_type(value, RespDataType::Arrays)?;
        let value = Self::get_value(value, false)?;
        let (start, length) = get_length(&value)?;
        let resp_values = Self::split_resp_types(&value[start..]);
        let mut data: Vec<RespDataTypeValue> = Vec::new();
        for resp_value in resp_values {
            let result = RespParser::parse(&resp_value)?;
            data.push(result);
        }
        if length as usize != data.len() {
            return Err(anyhow!("RESP_LENGTH_ACTUAL_LENGTH_MISMATCHED"));
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
        ];

        for test_case in test_cases {
            let result = Arrays::build(&test_case.input);
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }
}
