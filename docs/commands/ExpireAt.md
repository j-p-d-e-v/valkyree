# `ExpireAt(String, u64, Option<ExpiryKind>)`

Sets an absolute expiration time on a key (Unix time).

## 🧩 Description
Sets the expiration for a key as a Unix timestamp (in seconds). Optional `ExpiryKind` may modify behavior.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
use valkeyree::types::ExpiryKind;
let unix_ts = 1_700_000_000u64; 
let cmd = CommandKind::ExpireAt("token".into(), unix_ts, None).build()?;
```

## 🔗 References
- 📘 [EXPIREAT](https://valkey.io/commands/expireat/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
