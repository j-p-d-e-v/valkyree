#[derive(Debug)]
pub struct Ping {}

impl Ping {
    pub fn build() -> anyhow::Result<String> {
        Ok(String::from("PING\r\n"))
    }
}

#[cfg(test)]
pub mod test_ping {
    use super::*;

    #[test]
    fn test() {
        let result = Ping::build();
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("PING\r\n".to_string(), result.unwrap());
    }
}
