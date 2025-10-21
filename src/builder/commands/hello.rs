#[derive(Debug)]
pub struct Hello {}

impl Hello {
    pub fn build() -> anyhow::Result<String> {
        Ok(String::from("HELLO\r\n"))
    }
}

#[cfg(test)]
pub mod test_hello {
    use super::*;

    #[test]
    fn test() {
        let result = Hello::build();
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("HELLO\r\n".to_string(), result.unwrap());
    }
}
