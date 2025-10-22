# `IncrementBy(String, u64)`

Increments the integer value of a key by a given amount.

## 🧩 Description
If the key does not exist, it is created with value `0` before adding the increment.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::IncrementBy("counter".into(), 10).build()?;
```

## 🔗 References
- 📘 [INCRBY](https://valkey.io/commands/incrby/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
