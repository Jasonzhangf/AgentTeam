use agentteam_contracts::control::{AgentControlAction, AgentControlMode, AgentCtlReq01ModeIntent};

use crate::{
    merge_attach_tui_status, AgentControlCenter, ControlRetryInput, ControlSendInput,
    ControlSessionInput,
};

#[test]
fn attach_help_returns_tmux_contract() {
    let control = AgentControlCenter::new();
    assert!(control.help("attach").contains("tmux"));
}

#[test]
fn control_chain_projects_receipt() {
    let projection = AgentCtlReq01ModeIntent::new(
        "Kevin",
        "default",
        AgentControlMode::AttachTui,
        "TA_local_agentteam_Kevin",
    )
    .resolve_mode()
    .bind_session()
    .apply_action(AgentControlAction::Attach, "idle", "bound")
    .project("receipt-1");

    assert_eq!(projection.state, "idle");
    assert_eq!(projection.receipt_id, "receipt-1");
}

#[test]
fn retry_input_requires_fact_ids() {
    let control = AgentControlCenter::new();
    let err = control
        .retry_dispatch(ControlRetryInput::new(
            ControlSessionInput::new("Kevin", "default", "TA_local_agentteam_Kevin"),
            "",
            "error-1",
        ))
        .unwrap_err();
    assert!(matches!(err, crate::ControlError::Validation { .. }));
}

#[test]
fn send_input_requires_text() {
    let control = AgentControlCenter::new();
    let err = control
        .send_input(ControlSendInput::new(
            ControlSessionInput::new("Kevin", "default", "TA_local_agentteam_Kevin"),
            "   ",
        ))
        .unwrap_err();
    assert!(matches!(
        err,
        crate::ControlError::Tmux { .. } | crate::ControlError::Validation { .. }
    ));
}

#[test]
fn attach_status_without_sdk_scope_reports_tmux_only() {
    let (state, details) = merge_attach_tui_status(
        &ControlSessionInput::new("Kevin", "default", "TA_local_agentteam_Kevin"),
        "busy".to_owned(),
        "tmux thinking".to_owned(),
    )
    .unwrap();

    assert_eq!(state, "busy");
    assert!(details.contains("sdk_status=not_requested"));
    assert!(details.contains("tmux_observed=true"));
}

#[test]
fn attach_status_rejects_partial_sdk_scope() {
    let session = ControlSessionInput::new("Kevin", "default", "TA_local_agentteam_Kevin")
        .with_scope("/repo/agentteam", "");
    let mut partial = session.clone();
    partial.project_slug = None;
    let err =
        merge_attach_tui_status(&partial, "idle".to_owned(), "tmux idle".to_owned()).unwrap_err();

    assert!(matches!(err, crate::ControlError::Validation { .. }));
    assert!(err.reason().contains("--cwd and --project"));
}

#[test]
fn headless_bridge_response_parses_sdk_payload() {
    let payload = r#"{
        "ok": true,
        "operation": "start",
        "session_name": "TA_local_agentteam_Kevin",
        "project_slug": "agentteam",
        "thread_id": "thread-1",
        "turn_id": null,
        "state": "idle",
        "details": "thread idle",
        "active_flags": [],
        "final_response": null
    }"#;
    let response: crate::headless_protocol::HeadlessBridgeResponse =
        serde_json::from_str(payload).unwrap();
    assert_eq!(response.project_slug, "agentteam");
    assert_eq!(response.state, "idle");
    assert_eq!(response.operation, "start");
}

#[test]
fn headless_run_requires_input() {
    let control = AgentControlCenter::new();
    let err = control
        .headless_run(ControlSendInput::new(
            ControlSessionInput::new("Kevin", "default", "TA_headless_test"),
            " ",
        ))
        .unwrap_err();
    assert!(matches!(err, crate::ControlError::Validation { .. }));
}

#[test]
fn agent_session_binding_requires_thread_id() {
    let control = AgentControlCenter::new();
    let response = crate::headless_protocol::HeadlessBridgeResponse {
        ok: true,
        operation: "seed".to_owned(),
        session_name: "TA_headless_test".to_owned(),
        project_slug: "agentteam".to_owned(),
        thread_id: None,
        turn_id: Some("turn-1".to_owned()),
        state: "idle".to_owned(),
        details: "thread idle".to_owned(),
        active_flags: Some(Vec::new()),
        final_response: None,
    };
    let err = response
        .thread_id
        .ok_or(crate::ControlError::HeadlessBridge {
            reason: "Codex SDK seed did not return thread_id".to_owned(),
        });

    assert!(err.is_err());
    assert!(control.help("headless").contains("Codex SDK"));
}

#[test]
fn stopped_headless_bridge_projects_offline() {
    let control = AgentControlCenter::new();
    let response = crate::headless_protocol::HeadlessBridgeResponse {
        ok: true,
        operation: "stop".to_owned(),
        session_name: "TA_headless_test".to_owned(),
        project_slug: "agentteam".to_owned(),
        thread_id: Some("thread-1".to_owned()),
        turn_id: None,
        state: "offline".to_owned(),
        details: "headless bridge stopped".to_owned(),
        active_flags: Some(Vec::new()),
        final_response: None,
    };
    let projection = control.project_headless_response(
        ControlSessionInput::new("Kevin", "default", "TA_headless_test"),
        AgentControlAction::HeadlessStop,
        response,
    );

    assert_eq!(projection.state, "offline");
    assert_eq!(projection.details, "headless bridge stopped");
}
