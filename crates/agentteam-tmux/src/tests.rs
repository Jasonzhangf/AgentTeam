use crate::error::TmuxAdapterError;
use crate::loopback::{
    build_sessions, runtime_scope, sanitized_runtime_scope, validate_loopback_input,
};
use crate::model::TmuxLoopbackInput;

#[test]
fn rejects_zero_session_count() {
    let error = validate_loopback_input(&TmuxLoopbackInput::new(
        "target/agentteam-tmux-test".to_owned(),
        0,
    ))
    .unwrap_err();
    assert!(matches!(error, TmuxAdapterError::Validation { .. }));
}

#[test]
fn runtime_scope_requires_directory_name() {
    let error = runtime_scope("/").unwrap_err();
    assert!(matches!(error, TmuxAdapterError::Validation { .. }));
}

#[test]
fn scope_sanitizer_keeps_ta_safe_names() {
    let scope = sanitized_runtime_scope("AgentTeam Smoke.01").unwrap();
    assert_eq!(scope, "agentteam-smoke-01");
}

#[test]
fn managed_sessions_use_ta_prefix_and_logical_ids() {
    let sessions = build_sessions("TA-agentteam-loopback-7", 2);
    assert_eq!(sessions.len(), 2);
    assert_eq!(sessions[0].logical_id, "agent-01");
    assert!(sessions[0].session_name.starts_with("TA-"));
    assert!(sessions[1]
        .output_marker
        .contains(&sessions[1].input_marker));
}
