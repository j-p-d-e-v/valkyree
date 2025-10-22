# `Raw(String)`

Sends a raw, already-encoded command string.

## 🧩 Description
Pass-through for sending a prebuilt command line. Useful for advanced or not-yet-modeled commands.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Raw("ECHO hello\r\n".into()).build()?;
```

## 🔗 References
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
