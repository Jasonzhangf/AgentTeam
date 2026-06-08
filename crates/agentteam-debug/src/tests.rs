use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use agentteam_persist::replay_event_log;
use agentteam_resource::ResourceRegistry;

use crate::*;

fn temp_log_path(test_name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("agentteam-debug-{test_name}-{nanos}.jsonl"))
}

fn input() -> DebugBundleInput {
    DebugBundleInput {
        requested_by: "Kevin".to_owned(),
        scope: "project".to_owned(),
        module: "resource.lifecycle".to_owned(),
    }
}

#[test]
fn debug_bundle_persists_before_projection() {
    let path = temp_log_path("persist");
    let mut resources = ResourceRegistry::new();
    let bundle = capture_debug_bundle(&path, &mut resources, input()).unwrap();
    assert!(bundle.bundle_id.starts_with("debug-bundle-"));
    assert!(bundle.persistence_receipt_id.starts_with("receipt-"));
    assert!(bundle
        .resource_snapshot_id
        .starts_with("resource-snapshot-"));
    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 3);
    assert_eq!(replayed.events[0].event_kind, "resource_acquire");
    assert_eq!(replayed.events[1].event_kind, "debug_bundle");
    assert_eq!(replayed.events[2].event_kind, "resource_release");
}

#[test]
fn debug_bundle_includes_resource_snapshot() {
    let path = temp_log_path("resource-snapshot");
    let mut resources = ResourceRegistry::new();
    let bundle = capture_debug_bundle(&path, &mut resources, input()).unwrap();
    let replayed = replay_event_log(&path, 0).unwrap();
    assert!(replayed.events[1]
        .payload_json
        .contains(&bundle.resource_snapshot_id));
}

#[test]
fn invalid_debug_input_fails_validation() {
    let path = temp_log_path("invalid");
    let mut resources = ResourceRegistry::new();
    let error = capture_debug_bundle(
        &path,
        &mut resources,
        DebugBundleInput {
            requested_by: String::new(),
            scope: "project".to_owned(),
            module: "resource.lifecycle".to_owned(),
        },
    )
    .unwrap_err();
    assert!(matches!(error, DebugError::Validation { .. }));
}
