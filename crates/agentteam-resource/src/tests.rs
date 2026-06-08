use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use agentteam_persist::replay_event_log;

use crate::*;

fn temp_log_path(test_name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("agentteam-resource-{test_name}-{nanos}.jsonl"))
}

fn input(owner_module: &str) -> ResourceAcquireInput {
    ResourceAcquireInput {
        owner_module: owner_module.to_owned(),
        owner_entity_id: "bundle-1".to_owned(),
        resource_class: "debug_bundle".to_owned(),
        scope: "project".to_owned(),
        memory_bytes_estimate: 128,
        handle_count: 1,
    }
}

#[test]
fn acquire_registers_lease_and_persists_event() {
    let path = temp_log_path("acquire");
    let mut registry = ResourceRegistry::new();
    let lease = registry.acquire(&path, input("debug.center")).unwrap();
    assert!(lease.lease_id.starts_with("lease-"));
    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 1);
    assert_eq!(replayed.events[0].feature_id, FEATURE_ID);
    assert_eq!(replayed.events[0].event_kind, "resource_acquire");
}

#[test]
fn release_requires_owner_and_persists_event() {
    let path = temp_log_path("release");
    let mut registry = ResourceRegistry::new();
    let lease = registry.acquire(&path, input("debug.center")).unwrap();
    let released = registry
        .release(&path, &lease.lease_id, "debug.center")
        .unwrap();
    assert_eq!(released.lease_id, lease.lease_id);
    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 2);
    assert_eq!(replayed.events[1].event_kind, "resource_release");
}

#[test]
fn release_by_non_owner_fails() {
    let path = temp_log_path("owner");
    let mut registry = ResourceRegistry::new();
    let lease = registry.acquire(&path, input("debug.center")).unwrap();
    let error = registry
        .release(&path, &lease.lease_id, "config.center")
        .unwrap_err();
    assert!(matches!(error, ResourceError::NotOwner { .. }));
}

#[test]
fn leak_projection_is_persisted_and_visible_in_snapshot() {
    let path = temp_log_path("leak");
    let mut registry = ResourceRegistry::new();
    let lease = registry.acquire(&path, input("debug.center")).unwrap();
    registry
        .mark_leak(&path, &lease.lease_id, "stale heartbeat")
        .unwrap();
    let snapshot = registry.snapshot();
    assert_eq!(snapshot.leak_suspect_count, 1);
    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events[1].event_kind, "resource_leak_suspected");
}

#[test]
fn invalid_acquire_input_fails_validation() {
    let path = temp_log_path("invalid");
    let mut registry = ResourceRegistry::new();
    let error = registry.acquire(&path, input("")).unwrap_err();
    assert!(matches!(error, ResourceError::Validation { .. }));
}
