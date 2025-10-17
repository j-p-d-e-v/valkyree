use anyhow::anyhow;

#[derive(Debug)]
pub struct Get {}

impl Get {
    pub fn build(value: &str) -> anyhow::Result<String> {
        if value.is_empty() {
            return Err(anyhow!("GET_KEY_REQUIRED"));
        }
        Ok(format!("GET {value}\r\n"))
    }
}

#[cfg(test)]
pub mod test_get {
    use super::*;

    #[test]
    fn test() {
        let result = Get::build("mykey");
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("GET mykey\r\n".to_string(), result.unwrap());
    }

    #[test]
    fn test_error() {
        let result = Get::build("");
        assert!(result.is_err());
    }
}
