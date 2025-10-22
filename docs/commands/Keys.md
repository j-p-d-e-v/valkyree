# `Keys(String)`

Finds all keys matching a given pattern.

## 🧩 Description
Returns keys that match the pattern (e.g., `user:*`). Use with care in production environments.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Keys("user:*".into()).build()?;
```

## 🔗 References
- 📘 [KEYS](https://valkey.io/commands/keys/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
