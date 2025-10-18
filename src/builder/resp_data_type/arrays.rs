use crate::builder::resp_data_type::helpers::get_length;
use serde_json::Value;

#[derive(Debug)]
pub struct Arrays {}

impl Arrays {
    pub fn build(value: &[u8]) -> anyhow::Result<Value> {
        let l = get_length(value)?;
        let start = l.0;
        let length = l.1;

        if length == 0 {
            return Ok(Value::Array(vec![]));
        }
        let value = value.get(start..).unwrap_or(&[]);
        let result = String::from_utf8_lossy(value);
        Ok(Value::String(result.to_string()))
    }
}
