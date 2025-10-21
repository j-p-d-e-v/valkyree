use anyhow::anyhow;

#[derive(Debug)]
pub struct Exists {}

impl Exists {
    pub fn build(value: &[String]) -> anyhow::Result<String> {
        if value.is_empty() {
            return Err(anyhow!("EXISTS_KEYS_REQUIRED"));
        }
        let keys = value.join(" ");
        Ok(format!("EXISTS {keys}\r\n"))
    }
}

#[cfg(test)]
pub mod test_exists {
    use super::*;

    #[test]
    fn test() {
        let result = Exists::build(&vec!["mykey".to_string(), "yourkey".to_string()]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("EXISTS mykey yourkey\r\n".to_string(), result.unwrap());
    }
}
