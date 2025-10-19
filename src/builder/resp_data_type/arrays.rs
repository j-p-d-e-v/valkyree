use crate::builder::resp_data_type::helpers::get_length;
use crate::builder::resp_data_type::RespDataTypeBase;
use crate::builder::resp_data_type::RespParser;
use crate::types::resp_data_kind::RespDataType;
use crate::types::Value;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Arrays {}

impl RespDataTypeBase for Arrays {}
impl Arrays {
    pub fn split_resp_types(value: &[u8]) -> Vec<Vec<u8>> {
        let mut cr_flag = false;
        let mut lf_flag = false;
        let mut kind_flag = false;
        let mut saw_array: u8 = 0;
        let identifiers = RespDataType::get_identifiers_decimals();
        let mut tmp_data: Vec<u8> = Vec::new();
        let mut data: Vec<Vec<u8>> = Vec::new();
        for v in value.iter() {
            if v == &13 {
                cr_flag = true;
            }
            if v == &10 {
                lf_flag = true;
            }
            if let Ok(array) = RespDataType::Arrays.to_decimal() && &array == v {
               saw_array += 1; 
            }
            if identifiers.contains(v) && cr_flag && lf_flag {
                kind_flag = true;
            }
            if cr_flag && lf_flag && kind_flag && (saw_array == 0 || saw_array == 2) {
                data.push(tmp_data);
                tmp_data = Vec::new();
                cr_flag = false;
                lf_flag = false;
                kind_flag = false;
                saw_array = if saw_array > 0 { 1 } else { 0 };
            }

            tmp_data.push(*v);
        }
        if tmp_data.len() > 0 {
            data.push(tmp_data);
        }
        data
    }
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        Self::is_data_type(value, RespDataType::Arrays)?;
        let value = Self::get_value(value, false)?;
        let (start, length) = get_length(&value)?;
        let resp_values = Self::split_resp_types(&value[start..]);
        let mut data: Vec<Value> = Vec::new();
        for resp_value in resp_values {
            let parser = RespParser::new(&resp_value);
            let result = parser.get()?;
            data.push(result);
        }
        if length as usize != data.len() {
            return Err(anyhow!("RESP_LENGTH_ACTUAL_LENGTH_MISMATCHED"));
        }
        Ok(Value::Array(data))
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
            pub expected: Value,
        }

        let test_cases = vec![
             TestCase {
                 // *0\r\n
                 // empty array
                 input: vec![identifier, 48, 13, 10],
                 expected: Value::Array(vec![]),
             },
            TestCase {
                // *2\r\n+hello\r\n:5\r\n
                // [ "hello", 5 ]
                input: vec![
                    identifier, 50, 13, 10, // *2\r\n
                    43, 104, 101, 108, 108, 111, 13, 10, // +hello\r\n
                    58, 53, 13, 10, // :5\r\n
                ],
                expected: Value::Array(vec![Value::String("hello".into()), Value::Integer(5)]),
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
                 expected: Value::Array(vec![
                     Value::Boolean(true),
                     Value::Boolean(false),
                     Value::Null,
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
                expected: Value::Array(vec![
                    Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
                    Value::Array(vec![
                        Value::String("foo".into()),
                        Value::String("bar".into()),
                        Value::String("baz".into()),
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
