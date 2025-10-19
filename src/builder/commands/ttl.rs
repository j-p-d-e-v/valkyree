use anyhow::anyhow;

#[derive(Debug)]
pub struct Ttl {}

impl Ttl {
    pub fn build(value: &str) -> anyhow::Result<String> {
        if value.is_empty() {
            return Err(anyhow!("TTL_KEY_REQUIRED"));
        }
        Ok(format!("TTL {value}\r\n"))
    }
}

#[cfg(test)]
pub mod test_ttl {
    use super::*;

    #[test]
    fn test() {
        let result = Ttl::build("mykey");
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("TTL mykey\r\n".to_string(), result.unwrap());
    }
}
