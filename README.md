# trace-diff

[![crates.io](https://img.shields.io/crates/v/trace-diff.svg)](https://crates.io/crates/trace-diff)

Diff two agent traces semantically. Align by `(kind, key)`, ignore
timestamps and ids, emit Added/Removed/Changed entries.

```rust
use trace_diff::{diff, Step};
use serde_json::json;
let base = vec![Step { kind: "tool_call".into(), key: "read".into(), payload: json!({"path": "a"}) }];
let new  = vec![Step { kind: "tool_call".into(), key: "read".into(), payload: json!({"path": "b"}) }];
let changes = diff(&base, &new);
```

MIT or Apache-2.0.
