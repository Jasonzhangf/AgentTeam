use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use agentteam_contracts::team::TeamReq03ValidatedIntent;

use crate::local::LocalCommandError;
use crate::local::{execute_local_intent, LocalCommandResult};

fn example_config_path() -> String {
    format!(
        "{}/../../docs/config/config.toml.example",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn temp_runtime_home(test_name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("agentteam-runtime-{test_name}-{nanos}"))
}

#[test]
fn local_config_check_executes_config_center() {
    let result = execute_local_intent(TeamReq03ValidatedIntent::ConfigCheck {
        config_path: example_config_path(),
        json: true,
    })
    .unwrap();

    match result {
        LocalCommandResult::ConfigCheck { normalized } => {
            assert_eq!(normalized.project_slug, "agentteam");
            assert_eq!(normalized.remote_domain_count, 1);
        }
        other => panic!("unexpected result {other:?}"),
    }
}

#[test]
fn local_daemon_check_reports_routeability_without_starting_processes() {
    let result = execute_local_intent(TeamReq03ValidatedIntent::DaemonCheck {
        config_path: example_config_path(),
        json: true,
    })
    .unwrap();

    match result {
        LocalCommandResult::DaemonCheck { daemon } => {
            assert_eq!(daemon.config_status, "valid");
            assert_eq!(daemon.domain_registry_status, "routeable");
            assert_eq!(daemon.routeable_endpoint_count, 2);
            assert_eq!(daemon.daemon_process_status, "not_started_by_check");
            assert_eq!(daemon.tmux_status, "not_touched_by_check");
            assert_eq!(daemon.zterm_status, "not_touched_by_check");
        }
        other => panic!("unexpected result {other:?}"),
    }
}

#[test]
fn local_domain_resolve_executes_domain_registry() {
    let result = execute_local_intent(TeamReq03ValidatedIntent::DomainResolve {
        target: "Alice@review-daemon".to_owned(),
        config_path: example_config_path(),
        json: true,
    })
    .unwrap();

    match result {
        LocalCommandResult::DomainResolve {
            target,
            registry_snapshot,
        } => {
            assert_eq!(target.domain_id, "review-daemon");
            assert_eq!(target.route_kind, "remote");
            assert_eq!(target.target_kind, "agent");
            assert_eq!(target.target_value, "Alice");
            assert_eq!(registry_snapshot.token_redaction_status, "redacted");
        }
        other => panic!("unexpected result {other:?}"),
    }
}

#[test]
fn local_debug_snapshot_persists_event_log() {
    let runtime_home = temp_runtime_home("debug");
    let result = execute_local_intent(TeamReq03ValidatedIntent::DebugSnapshot {
        config_path: example_config_path(),
        runtime_home: runtime_home.display().to_string(),
        json: true,
    })
    .unwrap();

    match result {
        LocalCommandResult::DebugSnapshot { bundle } => {
            assert_eq!(
                bundle.persistence_receipt_id,
                "receipt-00000000000000000002"
            );
            assert!(fs::metadata(bundle.event_log_path).unwrap().is_file());
        }
        other => panic!("unexpected result {other:?}"),
    }
}

#[test]
fn local_task_commands_persist_and_replay_state() {
    let runtime_home = temp_runtime_home("task");
    let home = runtime_home.display().to_string();
    let send = execute_local_intent(TeamReq03ValidatedIntent::TaskSend {
        runtime_home: home.clone(),
        team_id: "default".to_owned(),
        created_by: "Kevin".to_owned(),
        target_kind: "role".to_owned(),
        target: "builder".to_owned(),
        title: "Build local task engine".to_owned(),
        body: "Persist task event truth".to_owned(),
        json: true,
    })
    .unwrap();

    match send {
        LocalCommandResult::TaskSend { task } => {
            assert_eq!(task.task_id, "AT-000001");
            assert_eq!(task.status, "queued");
        }
        other => panic!("unexpected result {other:?}"),
    }

    let done = execute_local_intent(TeamReq03ValidatedIntent::TaskDone {
        runtime_home: home.clone(),
        task_id: "AT-000001".to_owned(),
        actor: "Alice".to_owned(),
        detail: "completed by explicit CLI signal".to_owned(),
        json: true,
    })
    .unwrap();
    match done {
        LocalCommandResult::TaskDone { task } => assert_eq!(task.status, "done"),
        other => panic!("unexpected result {other:?}"),
    }

    let status = execute_local_intent(TeamReq03ValidatedIntent::TaskStatus {
        runtime_home: home,
        task_id: "AT-000001".to_owned(),
        json: true,
    })
    .unwrap();
    match status {
        LocalCommandResult::TaskStatus { board } => {
            assert_eq!(board.task_count, 1);
            assert_eq!(board.tasks[0].status, "done");
            assert_eq!(
                board.tasks[0].latest_detail,
                "completed by explicit CLI signal"
            );
        }
        other => panic!("unexpected result {other:?}"),
    }
}

#[test]
fn local_tmux_loopback_rejects_invalid_session_count() {
    let result = execute_local_intent(TeamReq03ValidatedIntent::TmuxLoopback {
        runtime_home: "target/agentteam-tmux-test".to_owned(),
        session_count: "not-a-number".to_owned(),
        json: true,
    });
    let error = result.unwrap_err();
    assert!(matches!(error, LocalCommandError::Tmux { .. }));
}
