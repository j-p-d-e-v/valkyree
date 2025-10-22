# `Exists(Vec<String>)`

Checks if one or more keys exist.

## 🧩 Description
Returns the number of keys that exist among the specified ones.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Exists(vec!["k1".into(), "k2".into()]).build()?;
```

## 🔗 References
- 📘 [EXISTS](https://valkey.io/commands/exists/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
