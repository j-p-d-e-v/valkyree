# `Delete(Vec<String>)`

Removes one or more keys.

## 🧩 Description
Deletes the specified keys. Non-existing keys are ignored.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Delete(vec!["k1".into(), "k2".into()]).build()?;
```

## 🔗 References
- 📘 [DEL](https://valkey.io/commands/del/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
