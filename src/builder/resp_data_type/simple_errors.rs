use crate::builder::resp_data_type::RespDataTypeTrait;
use crate::builder::resp_data_type::helpers::get_resp_value;
use crate::types::RespDataTypeValue;
use crate::types::RespErrorKind;
use anyhow::anyhow;

#[derive(Debug)]
pub struct SimpleErrors<'a> {
    pub length: usize,
    pub value: &'a [u8],
}

impl<'a> RespDataTypeTrait<'a> for SimpleErrors<'a> {
    fn new(value: &'a [u8]) -> Self {
        Self { value, length: 0 }
    }
    fn len(&self) -> usize {
        self.length
    }
    fn build(&mut self) -> anyhow::Result<RespDataTypeValue> {
        let (new_value, id) = get_resp_value(self.value, true)?;
        if !id.is_simple_errors() {
            return Err(anyhow!("NOT_SIMPLE_ERRORS_TYPE"));
        }
        self.length = new_value.len() + 3;
        let data = String::from_utf8_lossy(new_value).to_string();
        let result = RespErrorKind::parse(data);
        Ok(result)
    }
}

#[cfg(test)]
pub mod test_simple_errors {
    use crate::types::resp_data_kind::RespDataType;

    use super::*;

    #[test]
    fn test_not_empty() {
        let identifier = RespDataType::SimpleErrors.to_decimal().unwrap();
        let input = vec![
            identifier, 78, 79, 65, 85, 84, 72, 32, 65, 117, 116, 104, 101, 110, 116, 105, 99, 97,
            116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 100, 46, 13, 10,
        ];
        let mut serrors = SimpleErrors::new(&input);
        let result = serrors.build();
        assert_eq!(
            RespDataTypeValue::Error(
                RespErrorKind::NoAuth,
                "Authentication required.".to_string()
            ),
            result.unwrap()
        );
    }
}
