use crate::types::ExpiryKind;
use anyhow::anyhow;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
#[derive(Debug)]
pub struct ExpireAt {}

impl ExpireAt {
    /// EXPIREAT has the same effect and semantic as EXPIRE, but instead of specifying the number of seconds representing the TTL (time to live), it takes an absolute Unix timestamp (seconds since January 1, 1970). A timestamp in the past will delete the key immediately.
    /// Reference: https://valkey.io/commands/expireat/
    /// Parameters:
    /// - key - The of the expiration
    /// - duration - The duration value in seconds
    /// - kind - The expiration kind variant.
    pub fn build(key: &str, duration: &u64, kind: &Option<ExpiryKind>) -> anyhow::Result<String> {
        let duration =
            if let Some(time) = SystemTime::now().checked_add(Duration::from_secs(*duration)) {
                time.duration_since(UNIX_EPOCH)?.as_secs()
            } else {
                return Err(anyhow!("EXPIRE_AT_DURATION_INVALID"));
            };
        if key.is_empty() {
            return Err(anyhow!("EXPIRE_AT_KEY_REQUIRED"));
        }
        if let Some(k) = kind {
            Ok(format!("EXPIREAT {key} {duration} {k}\r\n"))
        } else {
            Ok(format!("EXPIREAT {key} {duration}\r\n"))
        }
    }
}

#[cfg(test)]
pub mod test_expire_at {
    use super::*;

    #[test]
    fn test_with_kind() {
        let result = ExpireAt::build("mykey", &100, &Some(ExpiryKind::Xx));
        assert!(result.is_ok(), "{:#?}", result.err());
    }

    #[test]
    fn test_without_kind() {
        let result = ExpireAt::build("mykey", &100, &None);
        assert!(result.is_ok(), "{:#?}", result.err());
    }
}
