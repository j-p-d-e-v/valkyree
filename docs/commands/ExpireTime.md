# `ExpireTime(String)`

Returns the absolute expire time of a key as Unix time.

## 🧩 Description
If the key has an expire set, returns its Unix timestamp. May return special values if no expire or key missing.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::ExpireTime("token".into()).build()?;
```

## 🔗 References
- 📘 [EXPIRETIME](https://valkey.io/commands/expiretime/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
