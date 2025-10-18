use crate::builder::error::Error;
use crate::types::error_kind::ErrorKind;

#[derive(Debug)]
pub struct SimpleErrors {}

impl SimpleErrors {
    pub fn build(value: &[u8]) -> Error {
        let data = String::from_utf8_lossy(value).to_string();
        let split_data = data
            .split(" ")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let kind = if let Some(fvalue) = split_data.first() {
            ErrorKind::from(fvalue)
        } else {
            ErrorKind::Unknown
        };
        let message = if kind != ErrorKind::Unknown
            && let Some(values) = split_data.get(1..)
        {
            values.join(" ")
        } else {
            data
        };
        Error {
            kind,
            message,
            source: None,
        }
    }

    pub fn get_error_kind(value: &str) -> ErrorKind {
        let error_prefixes: Vec<&'static str> = vec![
            "ERR",         // Generic error
            "WRONGTYPE",   // Operation against wrong data type
            "NOAUTH",      // Authentication required
            "WRONGPASS",   // Invalid username/password
            "NOPERM",      // Permission denied
            "BUSY",        // Busy running a script
            "NOSCRIPT",    // Script not found (EVALSHA)
            "OOM",         // Out of memory
            "EXECABORT",   // Transaction aborted
            "LOADING",     // Loading dataset into memory
            "MASTERDOWN",  // Master node unavailable
            "READONLY",    // Server or replica is read-only
            "MISCONF",     // Misconfiguration (e.g. persistence issue)
            "CLUSTERDOWN", // Cluster not initialized or down
            "MOVED",       // Cluster redirection (definitive)
            "ASK",         // Cluster redirection (temporary)
            "TRYAGAIN",    // Retry command later
            "CROSSSLOT",   // Cluster key-slot mismatch
            "UNLOADING",   // Key is being unloaded
            "INDEX",       // Index out of range
            "PROTO",       // Protocol error
            "AUTH",        // Authentication failed or required (variant)
            "SYNTAX",      // Command syntax error
            "EXEC",        // Transaction execution error
            "READONLY",    // Replica or read-only node
            "LINK",        // Link-related or cluster connection issue
            "BUSYKEY",     // Target key exists when it shouldn’t
            "NOSAVE",      // Save or persist operation disabled
            "NOPUBSUB",    // Pub/Sub misuse
            "TIMEOUT",     // Blocking command timeout
            "CONFIG",      // Configuration error
            "REPLCONF",    // Replication configuration error
            "ASKING",      // Special cluster flag required
            "REDIS",       // Redis-like internal (for compatibility)
        ];
        let matched_prefix = error_prefixes
            .iter()
            .find(|prefix_value| prefix_value.to_string() == value);
        if matched_prefix.is_some() {
            ErrorKind::from(value)
        } else {
            ErrorKind::Unknown
        }
    }
}

#[cfg(test)]
pub mod test_simple_errors {
    use super::*;

    #[test]
    fn test_not_empty() {
        let result = SimpleErrors::build(&vec![
            78, 79, 65, 85, 84, 72, 32, 65, 117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 105,
            111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 100, 46,
        ]);
        println!("{:#?}", result);
    }
}
