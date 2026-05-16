//! # trace-diff
//!
//! Diff two agent traces semantically.
//!
//! A "step" is `{ type, key, payload }`. The diff walks both traces in
//! order; aligned positions are compared by `(type, key)`. The result
//! lists steps that are `Added`, `Removed`, or `Changed`. Timestamps
//! and noisy ids are not compared.
//!
//! Use this in agent regression suites: capture a baseline trace once,
//! re-run the agent, diff the new trace against the baseline. Any
//! `Changed` step is something worth looking at.
//!
//! ## Example
//!
//! ```
//! use trace_diff::{diff, Step, Change};
//! use serde_json::json;
//!
//! let base = vec![
//!     Step { kind: "tool_call".into(), key: "read".into(), payload: json!({"path": "a.txt"}) },
//!     Step { kind: "tool_call".into(), key: "write".into(), payload: json!({"path": "out.txt"}) },
//! ];
//! let new = vec![
//!     Step { kind: "tool_call".into(), key: "read".into(), payload: json!({"path": "a.txt"}) },
//!     Step { kind: "tool_call".into(), key: "write".into(), payload: json!({"path": "out.NEW.txt"}) },
//! ];
//! let changes = diff(&base, &new);
//! assert!(matches!(changes[0], Change::Changed { .. }));
//! ```

#![deny(missing_docs)]

use serde_json::Value;

/// One step in a trace.
#[derive(Debug, Clone)]
pub struct Step {
    /// Step kind, e.g. `tool_call`, `llm_response`, `error`.
    pub kind: String,
    /// Key inside the kind (e.g. the tool name).
    pub key: String,
    /// Payload to compare for equality.
    pub payload: Value,
}

/// A single change.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum Change {
    /// A step present in the new trace but missing in the baseline.
    Added { index: usize, kind: String, key: String },
    /// A step present in the baseline but missing in the new trace.
    Removed { index: usize, kind: String, key: String },
    /// A step present in both but with a different payload.
    Changed {
        index: usize,
        kind: String,
        key: String,
        baseline: Value,
        new: Value,
    },
}

/// Diff `base` against `new`. Returns one entry per detected change.
pub fn diff(base: &[Step], new: &[Step]) -> Vec<Change> {
    let mut out = Vec::new();
    let max = base.len().max(new.len());
    for i in 0..max {
        match (base.get(i), new.get(i)) {
            (Some(b), Some(n)) => {
                if b.kind != n.kind || b.key != n.key {
                    out.push(Change::Removed {
                        index: i,
                        kind: b.kind.clone(),
                        key: b.key.clone(),
                    });
                    out.push(Change::Added {
                        index: i,
                        kind: n.kind.clone(),
                        key: n.key.clone(),
                    });
                } else if b.payload != n.payload {
                    out.push(Change::Changed {
                        index: i,
                        kind: b.kind.clone(),
                        key: b.key.clone(),
                        baseline: b.payload.clone(),
                        new: n.payload.clone(),
                    });
                }
            }
            (Some(b), None) => out.push(Change::Removed {
                index: i,
                kind: b.kind.clone(),
                key: b.key.clone(),
            }),
            (None, Some(n)) => out.push(Change::Added {
                index: i,
                kind: n.kind.clone(),
                key: n.key.clone(),
            }),
            (None, None) => unreachable!(),
        }
    }
    out
}
