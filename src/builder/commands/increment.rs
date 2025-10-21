use anyhow::anyhow;

#[derive(Debug)]
pub struct Increment {}

impl Increment {
    pub fn build(value: &str) -> anyhow::Result<String> {
        if value.is_empty() {
            return Err(anyhow!("INCR_KEY_REQUIRED"));
        }
        Ok(format!("INCR {value}\r\n"))
    }
}

#[cfg(test)]
pub mod test_increment {
    use super::*;

    #[test]
    fn test() {
        let result = Increment::build("mykey");
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("INCR mykey\r\n".to_string(), result.unwrap());
    }
}
