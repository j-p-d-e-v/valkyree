# `Ping`

Checks connectivity with the server.

## 🧩 Description
Sends a ping to the server and expects a simple reply, commonly `PONG`.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Ping.build()?;
```

## 🔗 References
- 📘 [PING](https://valkey.io/commands/ping/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
