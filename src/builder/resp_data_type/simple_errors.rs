use crate::types::SimpleErrorKind;
use crate::types::Value;

#[derive(Debug)]
pub struct SimpleErrors {}

impl SimpleErrors {
    pub fn build(value: &[u8]) -> Value {
        let data = String::from_utf8_lossy(value).to_string();
        let split_data = data
            .split(" ")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let kind = if let Some(fvalue) = split_data.first() {
            SimpleErrorKind::from(fvalue)
        } else {
            SimpleErrorKind::Unknown
        };
        let message = if kind != SimpleErrorKind::Unknown
            && let Some(values) = split_data.get(1..)
        {
            values.join(" ")
        } else {
            data
        };
        Value::Error(kind, message)
    }
}

#[cfg(test)]
pub mod test_simple_errors {
    use super::*;

    #[test]
    fn test_not_empty() {
        let result = SimpleErrors::build(&vec![
            78, 79, 65, 85, 84, 72, 32, 65, 117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 105,
            111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 100, 46,
        ]);
        assert_eq!(
            Value::Error(
                SimpleErrorKind::NoAuth,
                "Authentication required.".to_string()
            ),
            result
        );
    }
}
