use anyhow::anyhow;

#[derive(Debug)]
pub struct Raw {}

impl Raw {
    pub fn build(value: &str) -> anyhow::Result<String> {
        if value.is_empty() {
            return Err(anyhow!("RAW_VALUE_REQUIRED"));
        }
        Ok(format!("{value}\r\n"))
    }
}

#[cfg(test)]
pub mod test_get {
    use super::*;

    #[test]
    fn test() {
        let result = Raw::build("something that will produce an error");
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(
            "something that will produce an error\r\n".to_string(),
            result.unwrap()
        );
    }

    #[test]
    fn test_error() {
        let result = Raw::build("");
        assert!(result.is_err());
    }
}
