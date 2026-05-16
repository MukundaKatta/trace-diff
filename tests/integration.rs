use serde_json::json;
use trace_diff::{diff, Change, Step};

fn step(kind: &str, key: &str, payload: serde_json::Value) -> Step {
    Step {
        kind: kind.into(),
        key: key.into(),
        payload,
    }
}

#[test]
fn identical_traces_have_no_changes() {
    let s = vec![step("tool_call", "read", json!({"path": "a"}))];
    assert!(diff(&s, &s).is_empty());
}

#[test]
fn payload_change_is_changed() {
    let a = vec![step("tool_call", "read", json!({"path": "a"}))];
    let b = vec![step("tool_call", "read", json!({"path": "b"}))];
    let d = diff(&a, &b);
    assert_eq!(d.len(), 1);
    assert!(matches!(d[0], Change::Changed { .. }));
}

#[test]
fn extra_new_step_is_added() {
    let a = vec![step("tool_call", "read", json!({}))];
    let b = vec![
        step("tool_call", "read", json!({})),
        step("tool_call", "write", json!({})),
    ];
    let d = diff(&a, &b);
    assert_eq!(d.len(), 1);
    assert!(matches!(d[0], Change::Added { .. }));
}

#[test]
fn missing_step_is_removed() {
    let a = vec![
        step("tool_call", "read", json!({})),
        step("tool_call", "write", json!({})),
    ];
    let b = vec![step("tool_call", "read", json!({}))];
    let d = diff(&a, &b);
    assert_eq!(d.len(), 1);
    assert!(matches!(d[0], Change::Removed { .. }));
}

#[test]
fn kind_swap_is_removed_then_added() {
    let a = vec![step("tool_call", "read", json!({}))];
    let b = vec![step("llm_response", "claude", json!({}))];
    let d = diff(&a, &b);
    assert_eq!(d.len(), 2);
}
