use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimpleErrorKind {
    Err,         // Generic error
    WrongType,   // Operation against wrong data type
    NoAuth,      // Authentication required
    WrongPass,   // Invalid username/password
    NoPerm,      // Permission denied
    Busy,        // Busy running a script
    NoScript,    // Script not found (EVALSHA)
    Oom,         // Out of memory
    ExecAbort,   // Transaction aborted
    Loading,     // Loading dataset into memory
    MasterDown,  // Master node unavailable
    ReadOnly,    // Server or replica is read-only
    Misconf,     // Misconfiguration (e.g. persistence issue)
    ClusterDown, // Cluster not initialized or down
    Moved,       // Cluster redirection (definitive)
    Ask,         // Cluster redirection (temporary)
    TryAgain,    // Retry command later
    CrossSlot,   // Cluster key-slot mismatch
    Unloading,   // Key is being unloaded
    Index,       // Index out of range
    Proto,       // Protocol error
    Auth,        // Authentication failed or required
    Syntax,      // Command syntax error
    Exec,        // Transaction execution error
    Link,        // Link-related or cluster connection issue
    BusyKey,     // Target key exists when it shouldn’t
    NoSave,      // Save or persist operation disabled
    NoPubSub,    // Pub/Sub misuse
    Timeout,     // Blocking command timeout
    Config,      // Configuration error
    ReplConf,    // Replication configuration error
    Asking,      // Special cluster flag required
    Redis,       // Redis-like internal (for compatibility)
    Unknown,     // Fallback for unrecognized prefix
}

impl SimpleErrorKind {
    pub fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "ERR" => Self::Err,
            "WRONGTYPE" => Self::WrongType,
            "NOAUTH" => Self::NoAuth,
            "WRONGPASS" => Self::WrongPass,
            "NOPERM" => Self::NoPerm,
            "BUSY" => Self::Busy,
            "NOSCRIPT" => Self::NoScript,
            "OOM" => Self::Oom,
            "EXECABORT" => Self::ExecAbort,
            "LOADING" => Self::Loading,
            "MASTERDOWN" => Self::MasterDown,
            "READONLY" => Self::ReadOnly,
            "MISCONF" => Self::Misconf,
            "CLUSTERDOWN" => Self::ClusterDown,
            "MOVED" => Self::Moved,
            "ASK" => Self::Ask,
            "TRYAGAIN" => Self::TryAgain,
            "CROSSSLOT" => Self::CrossSlot,
            "UNLOADING" => Self::Unloading,
            "INDEX" => Self::Index,
            "PROTO" => Self::Proto,
            "AUTH" => Self::Auth,
            "SYNTAX" => Self::Syntax,
            "EXEC" => Self::Exec,
            "LINK" => Self::Link,
            "BUSYKEY" => Self::BusyKey,
            "NOSAVE" => Self::NoSave,
            "NOPUBSUB" => Self::NoPubSub,
            "TIMEOUT" => Self::Timeout,
            "CONFIG" => Self::Config,
            "REPLCONF" => Self::ReplConf,
            "ASKING" => Self::Asking,
            "REDIS" => Self::Redis,
            _ => Self::Unknown,
        }
    }
}
