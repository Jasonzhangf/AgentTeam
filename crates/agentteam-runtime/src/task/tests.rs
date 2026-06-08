use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::task::{
    TaskCreateInput, TaskEngine, TaskEngineError, TaskStatus, TaskTargetKind, TaskTransitionInput,
};

fn temp_log_path(test_name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir()
        .join(format!("agentteam-task-{test_name}-{nanos}"))
        .join("events")
        .join("agentteam.jsonl")
}

fn create_input(title: &str) -> TaskCreateInput {
    TaskCreateInput {
        team_id: "default".to_owned(),
        created_by: "Kevin".to_owned(),
        target_kind: TaskTargetKind::Role,
        target: "builder".to_owned(),
        title: title.to_owned(),
        body: "Implement the requested slice".to_owned(),
    }
}

fn transition(task_id: &str) -> TaskTransitionInput {
    TaskTransitionInput {
        task_id: task_id.to_owned(),
        actor: "Alice".to_owned(),
        detail: "state evidence accepted".to_owned(),
    }
}

#[test]
fn create_task_persists_and_replays_board() {
    let path = temp_log_path("create");
    let engine = TaskEngine::new(path.clone());
    let changed = engine
        .create_task(create_input("Build task engine"))
        .unwrap();

    assert_eq!(changed.task_id, "AT-000001");
    assert_eq!(changed.status, TaskStatus::Queued);

    let reloaded = TaskEngine::new(path);
    let board = reloaded.board().unwrap();
    assert_eq!(board.task_count, 1);
    assert_eq!(board.tasks[0].title, "Build task engine");
    assert_eq!(board.tasks[0].status, TaskStatus::Queued);
}

#[test]
fn terminal_state_requires_explicit_done_or_error() {
    let path = temp_log_path("invalid_done");
    let engine = TaskEngine::new(path);
    engine.create_task(create_input("Finish docs")).unwrap();

    let changed = engine.mark_done(transition("AT-000001")).unwrap();
    let error = engine.mark_running(transition("AT-000001")).unwrap_err();

    assert_eq!(changed.status, TaskStatus::Done);
    assert!(matches!(error, TaskEngineError::InvalidTransition { .. }));
}

#[test]
fn task_error_is_normal_task_state() {
    let path = temp_log_path("task_error");
    let engine = TaskEngine::new(path);
    engine.create_task(create_input("Run risky task")).unwrap();
    engine.mark_running(transition("AT-000001")).unwrap();
    let changed = engine.mark_error(transition("AT-000001")).unwrap();

    assert_eq!(changed.status, TaskStatus::Error);
    let board = engine.status("AT-000001").unwrap();
    assert_eq!(board.tasks[0].status, TaskStatus::Error);
}

#[test]
fn unknown_task_status_fails() {
    let path = temp_log_path("missing");
    let engine = TaskEngine::new(path);

    let error = engine.status("AT-404").unwrap_err();

    assert_eq!(error.reason(), "task AT-404 was not found");
}
