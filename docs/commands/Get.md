# `Get(String)`

Retrieves the value of a key.

## 🧩 Description
Returns the value stored at the specified key, or `Nil` if the key does not exist.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Get("mykey".into()).build()?;
```

## 🔗 References
- 📘 [GET](https://valkey.io/commands/get/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
