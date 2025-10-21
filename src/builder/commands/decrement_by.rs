use anyhow::anyhow;

#[derive(Debug)]
pub struct DecrementBy {}

impl DecrementBy {
    pub fn build(key: &str, value: &u64) -> anyhow::Result<String> {
        if key.is_empty() {
            return Err(anyhow!("DECRBY_KEY_REQUIRED"));
        }
        Ok(format!("DECRBY {key} {value}\r\n"))
    }
}

#[cfg(test)]
pub mod test_increment_by {
    use super::*;

    #[test]
    fn test() {
        let result = DecrementBy::build("mykey", &1);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("DECRBY mykey 1\r\n".to_string(), result.unwrap());
    }
}
