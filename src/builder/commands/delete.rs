use anyhow::anyhow;

#[derive(Debug)]
pub struct Delete {}

impl Delete {
    pub fn build(value: &[String]) -> anyhow::Result<String> {
        if value.is_empty() {
            return Err(anyhow!("DELETE_KEYS_REQUIRED"));
        }
        let keys = value.join(" ");
        Ok(format!("DEL {keys}\r\n"))
    }
}

#[cfg(test)]
pub mod test_delete {
    use super::*;

    #[test]
    fn test() {
        let result = Delete::build(&vec!["mykey".to_string()]);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("DEL mykey\r\n".to_string(), result.unwrap());
    }
}
