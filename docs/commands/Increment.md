# `Increment(String)`

Increments the integer value of a key by one.

## 🧩 Description
If the key does not exist, it is created with value `0` before incrementing.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Increment("counter".into()).build()?;
```

## 🔗 References
- 📘 [INCR](https://valkey.io/commands/incr/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
