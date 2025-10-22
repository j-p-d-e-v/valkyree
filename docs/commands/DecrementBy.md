# `DecrementBy(String, u64)`

Decrements the integer value of a key by a given amount.

## 🧩 Description
If the key does not exist, it is created with value `0` before subtracting the amount.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::DecrementBy("counter".into(), 5).build()?;
```

## 🔗 References
- 📘 [DECRBY](https://valkey.io/commands/decrby/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
