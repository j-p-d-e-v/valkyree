# `Set(String, Value)`

Sets the value of a key.

## 🧩 Description
Assigns the given `Value` to the specified key. Overwrites any existing value.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
use serde_json::Value;
let cmd = CommandKind::Set("mykey".into(), Value::String("myvalue".into())).build()?;
```

## 🔗 References
- 📘 [SET](https://valkey.io/commands/set/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
