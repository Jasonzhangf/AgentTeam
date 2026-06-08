use crate::load::load_config_file;
use crate::parse::parse_config_toml;
use crate::snapshot::snapshot_config;
use crate::validate::validate_config;
use crate::{check_config_path, ConfigCenterError};

#[test]
fn example_config_normalizes() {
    let config = check_config_path(example_config_path()).unwrap();
    assert_eq!(config.project_slug, "agentteam");
    assert_eq!(config.local_domain_id, "local");
    assert_eq!(config.team_count, 1);
    assert_eq!(config.member_count, 3);
    assert_eq!(config.remote_domain_count, 1);
}

#[test]
fn snapshot_redacts_token_state() {
    let config = check_config_path(example_config_path()).unwrap();
    let snapshot = snapshot_config(&config, "snapshot-1");
    assert_eq!(snapshot.snapshot_id, "snapshot-1");
    assert!(snapshot.zterm_token_redacted);
    assert!(!format!("{snapshot:?}").contains("auth_token"));
}

#[test]
fn agent_count_mismatch_fails() {
    let raw = load_config_file(example_config_path()).unwrap();
    let raw = agentteam_contracts::config::ConfigReq02TomlRaw {
        path: raw.path,
        raw_toml: raw.raw_toml.replace("agent_count = 3", "agent_count = 2"),
    };
    let parsed = parse_config_toml(raw).unwrap();
    let error = validate_config(parsed).unwrap_err();
    assert_validation_reason(error, "agent_count");
}

#[test]
fn duplicate_member_name_fails() {
    let raw = load_config_file(example_config_path()).unwrap();
    let raw = agentteam_contracts::config::ConfigReq02TomlRaw {
        path: raw.path,
        raw_toml: raw.raw_toml.replace("name = \"Bob\"", "name = \"Alice\""),
    };
    let parsed = parse_config_toml(raw).unwrap();
    let error = validate_config(parsed).unwrap_err();
    assert_validation_reason(error, "duplicate member name");
}

#[test]
fn duplicate_domain_id_fails() {
    let raw = load_config_file(example_config_path()).unwrap();
    let raw = agentteam_contracts::config::ConfigReq02TomlRaw {
        path: raw.path,
        raw_toml: raw
            .raw_toml
            .replace("id = \"review-daemon\"", "id = \"local\""),
    };
    let parsed = parse_config_toml(raw).unwrap();
    let error = validate_config(parsed).unwrap_err();
    assert_validation_reason(error, "duplicate domain id");
}

fn assert_validation_reason(error: ConfigCenterError, expected: &str) {
    match error {
        ConfigCenterError::Validation { reason, .. } => assert!(
            reason.contains(expected),
            "expected {reason:?} to contain {expected:?}"
        ),
        other => panic!("expected validation error, got {other:?}"),
    }
}

fn example_config_path() -> String {
    format!(
        "{}/../../docs/config/config.toml.example",
        env!("CARGO_MANIFEST_DIR")
    )
}
