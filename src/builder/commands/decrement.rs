use anyhow::anyhow;

#[derive(Debug)]
pub struct Decrement {}

impl Decrement {
    pub fn build(value: &str) -> anyhow::Result<String> {
        if value.is_empty() {
            return Err(anyhow!("DECR_KEY_REQUIRED"));
        }
        Ok(format!("DECR {value}\r\n"))
    }
}

#[cfg(test)]
pub mod test_increment {
    use super::*;

    #[test]
    fn test() {
        let result = Decrement::build("mykey");
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("DECR mykey\r\n".to_string(), result.unwrap());
    }
}
