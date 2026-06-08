use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::*;

fn temp_log_path(test_name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("agentteam-{test_name}-{nanos}.jsonl"))
}

fn draft(kind: &str) -> PersistedEventDraft {
    PersistedEventDraft {
        feature_id: FEATURE_ID.to_owned(),
        event_kind: kind.to_owned(),
        payload_hash: format!("hash-{kind}"),
    }
}

#[test]
fn append_returns_receipt() {
    let path = temp_log_path("append");
    let receipt = append_event_log(&path, draft("debug_bundle")).unwrap();
    assert_eq!(receipt.sequence, 1);
    assert_eq!(receipt.log_path, path.display().to_string());
}

#[test]
fn replay_returns_events_in_sequence() {
    let path = temp_log_path("replay");
    append_event_log(&path, draft("first")).unwrap();
    append_event_log(&path, draft("second")).unwrap();
    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 2);
    assert_eq!(replayed.events[0].sequence, 1);
    assert_eq!(replayed.events[1].sequence, 2);
}

#[test]
fn replay_from_sequence_filters_older_events() {
    let path = temp_log_path("from-sequence");
    append_event_log(&path, draft("first")).unwrap();
    append_event_log(&path, draft("second")).unwrap();
    let replayed = replay_event_log(&path, 2).unwrap();
    assert_eq!(replayed.events.len(), 1);
    assert_eq!(replayed.events[0].sequence, 2);
}

#[test]
fn corrupt_record_fails_explicitly() {
    let path = temp_log_path("corrupt");
    fs::write(&path, "not-json\n").unwrap();
    let error = replay_event_log(&path, 0).unwrap_err();
    assert!(matches!(
        error,
        PersistenceError::Corruption { line: 1, .. }
    ));
}

#[test]
fn sequence_mismatch_fails_explicitly() {
    let path = temp_log_path("sequence-mismatch");
    fs::write(
        &path,
        r#"{"sequence":2,"event_id":"event-2","feature_id":"persist.event_log","event_kind":"bad","payload_hash":"hash"}"#,
    )
    .unwrap();
    let error = replay_event_log(&path, 0).unwrap_err();
    assert!(matches!(
        error,
        PersistenceError::Corruption { line: 1, .. }
    ));
    assert!(error.reason().contains("sequence mismatch"));
}

#[test]
fn empty_draft_fails_validation() {
    let path = temp_log_path("empty-draft");
    let error = append_event_log(
        &path,
        PersistedEventDraft {
            feature_id: String::new(),
            event_kind: "debug_bundle".to_owned(),
            payload_hash: "hash".to_owned(),
        },
    )
    .unwrap_err();
    assert!(matches!(error, PersistenceError::Validation { .. }));
}

#[test]
fn materialized_state_uses_latest_sequence() {
    let path = temp_log_path("materialize");
    append_event_log(&path, draft("first")).unwrap();
    append_event_log(&path, draft("second")).unwrap();
    let replayed = replay_event_log(&path, 0).unwrap();
    let state = materialize_event_log(&replayed);
    assert_eq!(state.latest_sequence, 2);
    assert_eq!(state.snapshot_id, "snapshot-00000000000000000002");
}
