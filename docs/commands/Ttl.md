# `Ttl(String)`

Returns the remaining time to live of a key.

## 🧩 Description
Returns the TTL in seconds, `-1` if the key exists but has no associated expire, or `-2` if the key does not exist.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Ttl("session".into()).build()?;
```

## 🔗 References
- 📘 [TTL](https://valkey.io/commands/ttl/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
