use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use agentteam_contracts::error::{ErrorSeverity, TeamErr01FaultFact};
use agentteam_persist::replay_event_log;

use crate::classify::{classify_fault, link_error_evidence};
use crate::*;

fn temp_log_path(test_name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("agentteam-error-{test_name}-{nanos}.jsonl"))
}

fn fault() -> TeamErr01FaultFact {
    TeamErr01FaultFact::new(
        "config",
        "validation",
        "missing_project",
        "project.slug is required",
    )
}

fn seed(sequence: u64) -> ErrorCodeSeed {
    ErrorCodeSeed::new("20260608T063012Z", sequence)
}

#[test]
fn classifies_with_severity_and_code() {
    let classified = classify_fault(fault(), ErrorSeverity::Error, &seed(1)).unwrap();
    assert_eq!(classified.severity, ErrorSeverity::Error);
    assert_eq!(
        classified.code,
        "config.validation.missing_project.20260608T063012Z.000001"
    );
}

#[test]
fn links_independent_evidence_id() {
    let classified = classify_fault(fault(), ErrorSeverity::Error, &seed(1)).unwrap();
    let linked = link_error_evidence(classified).unwrap();
    assert!(linked.evidence_id.starts_with("evidence-fnv1a64-"));
}

#[test]
fn persists_before_projection() {
    let path = temp_log_path("persist");
    let projection = handle_framework_fault(&path, fault(), ErrorSeverity::Fatal, seed(7)).unwrap();
    assert_eq!(projection.severity, ErrorSeverity::Fatal);
    assert!(projection.receipt_id.starts_with("receipt-"));
    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 1);
    assert_eq!(replayed.events[0].feature_id, FEATURE_ID);
    assert_eq!(replayed.events[0].event_kind, "framework_error");
    assert!(replayed.events[0].payload_json.contains(&projection.code));
    assert!(replayed.events[0]
        .payload_json
        .contains(&projection.evidence_id));
}

#[test]
fn malformed_code_seed_fails_validation() {
    let error = classify_fault(
        fault(),
        ErrorSeverity::Error,
        &ErrorCodeSeed::new("2026-06-08T06:30:12Z", 1),
    )
    .unwrap_err();
    assert!(matches!(error, ErrorCenterError::Validation { .. }));
}

#[test]
fn normal_agent_task_error_is_rejected() {
    let error = classify_fault(
        TeamErr01FaultFact::new(
            "task",
            "agent_task_error",
            "reported",
            "worker reported error",
        ),
        ErrorSeverity::Error,
        &seed(1),
    )
    .unwrap_err();
    assert!(error.reason().contains("Task Engine"));
}

#[test]
fn persist_failure_does_not_project_success() {
    let path = std::env::temp_dir();
    let error = handle_framework_fault(path, fault(), ErrorSeverity::Error, seed(1)).unwrap_err();
    assert!(matches!(error, ErrorCenterError::Persistence { .. }));
}
