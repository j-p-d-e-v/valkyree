# `Expire(String, u64, Option<ExpiryKind>)`

Sets a timeout on a key.

## 🧩 Description
Sets the expiration for a key in seconds. Optional `ExpiryKind` can adjust semantics (e.g., only set if key has/has not expiry).

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
use valkeyree::types::ExpiryKind;
let cmd = CommandKind::Expire("token".into(), 60, None).build()?;
```

## 🔗 References
- 📘 [EXPIRE](https://valkey.io/commands/expire/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
