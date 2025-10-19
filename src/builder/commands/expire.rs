use crate::types::ExpiryKind;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Expire {}

impl Expire {
    pub fn build(key: &str, duration: &u64, kind: &Option<ExpiryKind>) -> anyhow::Result<String> {
        if key.is_empty() {
            return Err(anyhow!("EXPIRE_KEY_REQUIRED"));
        }
        if let Some(k) = kind {
            Ok(format!("EXPIRE {key} {duration} {k}\r\n"))
        } else {
            Ok(format!("EXPIRE {key} {duration}\r\n"))
        }
    }
}

#[cfg(test)]
pub mod test_expire {
    use super::*;

    #[test]
    fn test_with_kind() {
        let result = Expire::build("mykey", &10, &Some(ExpiryKind::Xx));
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("EXPIRE mykey 10 XX\r\n".to_string(), result.unwrap());
    }

    #[test]
    fn test_without_kind() {
        let result = Expire::build("mykey", &10, &None);
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("EXPIRE mykey 10\r\n".to_string(), result.unwrap());
    }
}
