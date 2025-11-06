use crate::builder::resp_data_type::helpers::get_resp_multi_values;
use crate::builder::resp_data_type::helpers::is_cr;
use crate::builder::resp_data_type::helpers::is_lf;
use crate::builder::resp_data_type::RespDataTypeTrait;
use crate::builder::resp_data_type::RespParser;
use crate::types::resp_data_kind::RespDataType;
use crate::types::resp_data_type_iter::RespDataTypeIterator;
use crate::types::RespDataTypeValue;
use anyhow::anyhow;

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
        length: isize,
    ) -> anyhow::Result<bool> {
        let result = parser.parse()?;
        data.push(result);
        self.value = &self.value[parser.len()..];
        Ok(length == data.len() as isize)
    }
}
impl<'a> RespDataTypeTrait<'a> for Maps<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self {
            value,
            length: value.len(),
        }
    }
    fn len(&self) -> usize {
        self.length
    }

    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (start, length, main_id) = get_resp_multi_values(self.value)?;
        if !main_id.is_maps() {
            return Err(anyhow!("NOT_maps_TYPE"));
        }
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
                if id.is_maps() {
                    let result = self.build()?;
                    data.push(result);
                } else if id.is_bulk_strings() || id.is_verbatim_strings() || id.is_bulk_errors() {
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
pub mod test_maps {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_maps() {
        let identifier = RespDataType::Maps.to_decimal().unwrap();

        struct TestCase {
            pub input: Vec<u8>,
            pub expected: RespDataTypeValue,
        }

        let test_cases = vec![
            // 1) Empty map: %0\r\n
            TestCase {
                // %0\r\n
                input: vec![
                    37, // '%'
                    48, 13, 10, // "0\r\n" → length = 0
                ],
                expected: RespDataTypeValue::Null,
            },
        ];

        for test_case in test_cases {
            let mut maps = Maps::new(&test_case.input);
            let result = maps.build();
            assert!(result.is_ok(), "{:#?}", result.err());
            assert_eq!(test_case.expected, result.unwrap());
        }
    }
}
