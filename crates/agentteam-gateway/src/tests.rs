use crate::*;
use agentteam_runtime::local::{ConfigCheckResult, LocalCommandResult};

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
