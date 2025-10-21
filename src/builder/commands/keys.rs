use anyhow::anyhow;

#[derive(Debug)]
pub struct Keys {}

impl Keys {
    pub fn build(key: &str) -> anyhow::Result<String> {
        if key.is_empty() {
            return Err(anyhow!("KEYS_KEY_REQUIRED"));
        }
        Ok(format!("KEYS {key}\r\n"))
    }
}

#[cfg(test)]
pub mod test_keys {
    use super::*;

    #[test]
    fn test() {
        let result = Keys::build("*");
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("KEYS *\r\n".to_string(), result.unwrap());
    }
}
