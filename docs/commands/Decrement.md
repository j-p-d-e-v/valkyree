# `Decrement(String)`

Decrements the integer value of a key by one.

## 🧩 Description
If the key does not exist, it is created with value `0` before decrementing (resulting in `-1`).

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Decrement("counter".into()).build()?;
```

## 🔗 References
- 📘 [DECR](https://valkey.io/commands/decr/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
