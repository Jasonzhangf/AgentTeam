use crate::*;
use agentteam_runtime::local::LocalCommandResult;
use agentteam_runtime::local_projection::{ConfigCheckResult, StartupStartResult, TaskBoardResult};

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn parses_config_check_intent() {
    let intent = parse_cli_args(strings(&[
        "config",
        "check",
        "--config",
        "docs/config/config.toml.example",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "config.check");
}

#[test]
fn parses_daemon_check_intent() {
    let intent = parse_cli_args(strings(&[
        "daemon",
        "check",
        "--config",
        "docs/config/config.toml.example",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "daemon.check");
}

#[test]
fn parses_domain_resolve_intent() {
    let intent = parse_cli_args(strings(&[
        "domain",
        "resolve",
        "--target",
        "Alice@review-daemon",
        "--config",
        "docs/config/config.toml.example",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "domain.resolve");
}

#[test]
fn parses_debug_snapshot_intent() {
    let intent = parse_cli_args(strings(&[
        "debug",
        "snapshot",
        "--config",
        "docs/config/config.toml.example",
        "--runtime-home",
        "target/agentteam-smoke",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "debug.snapshot");
}

#[test]
fn parses_start_intent() {
    let intent = parse_cli_args(strings(&[
        "start",
        "--cwd",
        "/Users/fanzhang/Documents/github/agentteam",
        "--team",
        "default",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "start");
}

#[test]
fn parses_ready_report_intent() {
    let intent = parse_cli_args(strings(&[
        "ready",
        "report",
        "--runtime-home",
        "target/agentteam-smoke",
        "--sender",
        "Alice",
        "--team",
        "default",
        "--agent-name",
        "Alice",
        "--body",
        "ready",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "ready.report");
}

#[test]
fn parses_task_send_intent() {
    let intent = parse_cli_args(strings(&[
        "task",
        "send",
        "--runtime-home",
        "target/agentteam-smoke",
        "--team",
        "default",
        "--created-by",
        "Kevin",
        "--target-kind",
        "role",
        "--target",
        "builder",
        "--title",
        "Build task",
        "--body",
        "Implement owner slice",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "task.send");
}

#[test]
fn parses_task_status_intent() {
    let intent = parse_cli_args(strings(&[
        "task",
        "status",
        "--runtime-home",
        "target/agentteam-smoke",
        "--task",
        "AT-000001",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "task.status");
}

#[test]
fn parses_task_claim_intent() {
    let intent = parse_cli_args(strings(&[
        "task",
        "claim",
        "--runtime-home",
        "target/agentteam-smoke",
        "--worker-name",
        "Alice",
        "--worker-role",
        "builder",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "task.claim");
}

#[test]
fn parses_msg_send_intent() {
    let intent = parse_cli_args(strings(&[
        "msg",
        "send",
        "--runtime-home",
        "target/agentteam-smoke",
        "--from",
        "Kevin",
        "--to",
        "Alice",
        "--action",
        "message",
        "--body",
        "hello",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "msg.send");
}

#[test]
fn parses_msg_broadcast_intent() {
    let intent = parse_cli_args(strings(&[
        "msg",
        "broadcast",
        "--runtime-home",
        "target/agentteam-smoke",
        "--sender",
        "Kevin",
        "--team",
        "default",
        "--action",
        "broadcast",
        "--body",
        "hello",
        "--members",
        "Alice,Bob",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "msg.broadcast");
}

#[test]
fn parses_control_attach_intent() {
    let intent = parse_cli_args(strings(&[
        "control",
        "attach",
        "--agent",
        "Kevin",
        "--team",
        "default",
        "--session",
        "TA_local_agentteam_Kevin",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "control");
}

#[test]
fn parses_control_retry_intent() {
    let intent = parse_cli_args(strings(&[
        "control",
        "retry",
        "--agent",
        "Kevin",
        "--team",
        "default",
        "--session",
        "TA_local_agentteam_Kevin",
        "--task",
        "AT-000001",
        "--error-fact",
        "error-1",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "control");
}

#[test]
fn parses_control_headless_run_intent() {
    let intent = parse_cli_args(strings(&[
        "control",
        "headless-run",
        "--agent",
        "Kevin",
        "--team",
        "default",
        "--session",
        "TA_headless_Kevin",
        "--input",
        "reply with exactly: ready",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "control");
}

#[test]
fn parses_tmux_loopback_intent() {
    let intent = parse_cli_args(strings(&[
        "tmux",
        "loopback",
        "--runtime-home",
        "target/agentteam-tmux-smoke",
        "--session-count",
        "2",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "tmux.loopback");
}

#[test]
fn missing_json_is_validation_error() {
    let error = parse_cli_args(strings(&[
        "config",
        "check",
        "--config",
        "docs/config/config.toml.example",
    ]))
    .unwrap_err();
    assert_eq!(error.class, "validation");
}

#[test]
fn unknown_flag_is_parse_error() {
    let error = parse_cli_args(strings(&[
        "config",
        "check",
        "--config",
        "docs/config/config.toml.example",
        "--bad",
        "--json",
    ]))
    .unwrap_err();
    assert_eq!(error.class, "parse");
}

#[test]
fn render_intent_json_marks_parse_only() {
    let intent = parse_cli_args(strings(&[
        "config",
        "check",
        "--config",
        "docs/config/config.toml.example",
        "--json",
    ]))
    .unwrap();
    let rendered = render_intent_json(&intent).unwrap();
    assert!(rendered.contains("\"local_parse_only\":true"));
    assert!(rendered.contains("\"command_name\":\"config.check\""));
}

#[test]
fn render_local_result_json_does_not_mark_parse_only() {
    let result = LocalCommandResult::ConfigCheck {
        normalized: ConfigCheckResult {
            path: "docs/config/config.toml.example".to_owned(),
            project_slug: "agentteam".to_owned(),
            project_root: "/tmp/agentteam".to_owned(),
            runtime_home: "/tmp/runtime".to_owned(),
            local_domain_id: "local".to_owned(),
            team_count: 1,
            member_count: 3,
            zterm_endpoint: "127.0.0.1:3333".to_owned(),
            remote_domain_count: 1,
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(!rendered.contains("local_parse_only"));
    assert!(rendered.contains("\"node\":\"TeamResp05DaemonResult\""));
    assert!(rendered.contains("\"project_slug\":\"agentteam\""));
}

#[test]
fn render_local_start_result_uses_start_command_name() {
    let result = LocalCommandResult::StartupStart {
        bootstrap: StartupStartResult {
            project_slug: "agentteam".to_owned(),
            project_root: "/Users/fanzhang/Documents/github/agentteam".to_owned(),
            runtime_home: "/Users/fanzhang/.agentteam/runtime/agentteam".to_owned(),
            cwd: "/Users/fanzhang/Documents/github/agentteam".to_owned(),
            team_id: "default".to_owned(),
            manager_name: "Kevin".to_owned(),
            manager_role: "manager".to_owned(),
            manager_session_name: "TA_local_agentteam_Kevin".to_owned(),
            manager_agent_session_id: "019eaad6-49c3-7e31-964b-e9bcae139702".to_owned(),
            manager_seed_turn_id: Some("turn-1".to_owned()),
            session_dir: "/Users/fanzhang/.agentteam/sessions/agentteam".to_owned(),
            planned_worker_count: 2,
            worker_names: vec!["Alice".to_owned(), "Bob".to_owned()],
            launch_status: "launched".to_owned(),
            session_lifecycle: "created".to_owned(),
            bootstrap_prompt_status: "seeded_before_tui_resume".to_owned(),
            agent_session_status: "idle".to_owned(),
            control_handoff_status: "attach_tui_resumed_agent_session".to_owned(),
            tui_resume_command: "codex".to_owned(),
            tui_resume_arg_count: 2,
            resource_lease_id: "lease-000000000000000001".to_owned(),
            resource_snapshot_id: "snapshot-1".to_owned(),
            tmux_session_observed: true,
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(rendered.contains("\"command_name\":\"start\""));
    assert!(!rendered.contains("local_parse_only"));
}

#[test]
fn render_task_result_json_uses_task_command_name() {
    let result = LocalCommandResult::TaskList {
        board: TaskBoardResult {
            task_count: 0,
            latest_sequence: 0,
            tasks: Vec::new(),
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(rendered.contains("\"command_name\":\"task.list\""));
}

#[test]
fn render_message_result_json_uses_msg_command_name() {
    let result = LocalCommandResult::MessageSend {
        delivery: agentteam_runtime::local_projection::MessageSendResult {
            delivery_id: "delivery:Alice:message".to_owned(),
            target: "Alice".to_owned(),
            action: "message".to_owned(),
            event_id: "event-1".to_owned(),
            sequence: 1,
            log_path: "target/agentteam-smoke/events/agentteam.jsonl".to_owned(),
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(rendered.contains("\"command_name\":\"msg.send\""));
}

#[test]
fn render_broadcast_result_json_uses_msg_command_name() {
    let result = LocalCommandResult::BroadcastSend {
        delivery: agentteam_runtime::local_projection::BroadcastSendResult {
            delivery_id: "delivery:default:broadcast".to_owned(),
            team_id: "default".to_owned(),
            recipient_count: 2,
            event_id: "event-1".to_owned(),
            sequence: 1,
            log_path: "target/agentteam-smoke/events/agentteam.jsonl".to_owned(),
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(rendered.contains("\"command_name\":\"msg.broadcast\""));
}

#[test]
fn render_ready_result_json_uses_ready_command_name() {
    let result = LocalCommandResult::ReadyReport {
        delivery: agentteam_runtime::local_projection::ReadyReportResult {
            delivery_id: "delivery:Alice:ready.report".to_owned(),
            team_id: "default".to_owned(),
            agent_name: "Alice".to_owned(),
            event_id: "event-1".to_owned(),
            sequence: 1,
            log_path: "target/agentteam-smoke/events/agentteam.jsonl".to_owned(),
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(rendered.contains("\"command_name\":\"ready.report\""));
}

#[test]
fn render_control_result_json_uses_dynamic_control_command_name() {
    let result = LocalCommandResult::Control {
        control: agentteam_runtime::local_projection::ControlResult {
            action: "attach".to_owned(),
            agent_name: "Kevin".to_owned(),
            team_id: "default".to_owned(),
            session_name: "TA_local_agentteam_Kevin".to_owned(),
            mode: "attach_tui".to_owned(),
            adapter_kind: "tmux".to_owned(),
            state: "idle".to_owned(),
            details: "bound".to_owned(),
            receipt_id: "control-attach-000001".to_owned(),
            observed_bytes: 5,
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(rendered.contains("\"command_name\":\"control.attach\""));
    assert!(rendered.contains("\"state\":\"idle\""));
}

#[test]
fn render_headless_run_result_json_uses_dynamic_control_command_name() {
    let result = LocalCommandResult::Control {
        control: agentteam_runtime::local_projection::ControlResult {
            action: "headless-run".to_owned(),
            agent_name: "Kevin".to_owned(),
            team_id: "default".to_owned(),
            session_name: "TA_headless_Kevin".to_owned(),
            mode: "headless".to_owned(),
            adapter_kind: "sdk".to_owned(),
            state: "idle".to_owned(),
            details: "ready".to_owned(),
            receipt_id: "control-headless-run-000001".to_owned(),
            observed_bytes: 5,
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(rendered.contains("\"command_name\":\"control.headless-run\""));
    assert!(rendered.contains("\"details\":\"ready\""));
}
