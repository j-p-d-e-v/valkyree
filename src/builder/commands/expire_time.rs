use anyhow::anyhow;

#[derive(Debug)]
pub struct ExpireTime {}

impl ExpireTime {
    pub fn build(key: &str) -> anyhow::Result<String> {
        if key.is_empty() {
            return Err(anyhow!("EXPIRETIME_KEY_REQUIRED"));
        }

        Ok(format!("EXPIRETIME {key}\r\n"))
    }
}

#[cfg(test)]
pub mod test_expire_time {
    use super::*;

    #[test]
    fn test() {
        let result = ExpireTime::build("mykey");
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("EXPIRETIME mykey\r\n".to_string(), result.unwrap());
    }
}
