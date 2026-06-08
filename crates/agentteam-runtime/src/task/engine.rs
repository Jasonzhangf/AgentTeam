use std::path::{Path, PathBuf};

use crate::task::error::{TaskEngineError, TaskEngineResult};
use crate::task::materialize::materialize_task_board;
use crate::task::model::{
    TaskBoard, TaskClaimInput, TaskCreateInput, TaskEventPayload, TaskStateChanged, TaskStatus,
    TaskTransitionInput,
};
use crate::task::persist::persist_task_event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEngine {
    log_path: PathBuf,
}

impl TaskEngine {
    pub fn new(log_path: impl Into<PathBuf>) -> Self {
        Self {
            log_path: log_path.into(),
        }
    }

    pub fn create_task(&self, input: TaskCreateInput) -> TaskEngineResult<TaskStateChanged> {
        validate_create_input(&input)?;
        let board = self.board()?;
        let next_id = next_task_id(&board);
        let payload = TaskEventPayload {
            task_id: next_id,
            team_id: input.team_id,
            actor: input.created_by,
            status: TaskStatus::Queued,
            target_kind: input.target_kind,
            target: input.target,
            title: input.title,
            body: input.body,
            priority: input.priority,
            blocked: input.blocked,
            detail: "created".to_owned(),
        };
        persist_state_change(&self.log_path, "task_created", payload)
    }

    pub fn claim_task(&self, input: TaskClaimInput) -> TaskEngineResult<TaskStateChanged> {
        validate_claim_input(&input)?;
        let board = self.board()?;
        let mut same_scope_running = false;
        let mut candidates = board
            .tasks
            .iter()
            .filter(|task| {
                let same_scope = claim_scope_matches(task, &input);
                if same_scope && task.status == TaskStatus::Running {
                    same_scope_running = true;
                }
                same_scope && task.status == TaskStatus::Queued
            })
            .collect::<Vec<_>>();

        if same_scope_running {
            return Err(TaskEngineError::Validation {
                reason: format!(
                    "worker {} with role {} already has a running task",
                    input.worker_name, input.worker_role
                ),
            });
        }

        candidates.sort_by_key(|task| {
            (
                if task.target_kind == crate::task::model::TaskTargetKind::Agent
                    && task.target == input.worker_name
                {
                    0usize
                } else {
                    1usize
                },
                if task.blocked { 0usize } else { 1usize },
                std::cmp::Reverse(task.priority),
                task.latest_sequence,
            )
        });

        let Some(task) = candidates.first() else {
            return Err(TaskEngineError::Validation {
                reason: format!(
                    "no claimable task for worker {} with role {}",
                    input.worker_name, input.worker_role
                ),
            });
        };

        let detail = format!(
            "claimed by {} as {} priority={} blocked={}",
            input.worker_name,
            claim_scope_label(task, &input),
            task.priority,
            task.blocked
        );
        self.transition_task(
            TaskTransitionInput {
                task_id: task.task_id.clone(),
                actor: input.worker_name,
                detail,
            },
            TaskStatus::Running,
            "task_claimed",
        )
    }

    pub fn mark_running(&self, input: TaskTransitionInput) -> TaskEngineResult<TaskStateChanged> {
        self.transition_task(input, TaskStatus::Running, "task_running")
    }

    pub fn mark_done(&self, input: TaskTransitionInput) -> TaskEngineResult<TaskStateChanged> {
        self.transition_task(input, TaskStatus::Done, "task_done")
    }

    pub fn mark_error(&self, input: TaskTransitionInput) -> TaskEngineResult<TaskStateChanged> {
        self.transition_task(input, TaskStatus::Error, "task_error")
    }

    pub fn board(&self) -> TaskEngineResult<TaskBoard> {
        materialize_task_board(&self.log_path)
    }

    pub fn status(&self, task_id: &str) -> TaskEngineResult<TaskBoard> {
        let board = self.board()?;
        let tasks = board
            .tasks
            .into_iter()
            .filter(|task| task.task_id == task_id)
            .collect::<Vec<_>>();
        if tasks.is_empty() {
            Err(TaskEngineError::NotFound {
                task_id: task_id.to_owned(),
            })
        } else {
            Ok(TaskBoard {
                task_count: tasks.len(),
                tasks,
                latest_sequence: board.latest_sequence,
            })
        }
    }

    fn transition_task(
        &self,
        input: TaskTransitionInput,
        next_status: TaskStatus,
        event_kind: &str,
    ) -> TaskEngineResult<TaskStateChanged> {
        validate_transition_input(&input)?;
        let board = self.board()?;
        let Some(task) = board
            .tasks
            .iter()
            .find(|task| task.task_id == input.task_id)
        else {
            return Err(TaskEngineError::NotFound {
                task_id: input.task_id,
            });
        };
        validate_transition(task.status, next_status, &task.task_id)?;
        let payload = TaskEventPayload {
            task_id: task.task_id.clone(),
            team_id: task.team_id.clone(),
            actor: input.actor,
            status: next_status,
            target_kind: task.target_kind,
            target: task.target.clone(),
            title: task.title.clone(),
            body: task.body.clone(),
            priority: task.priority,
            blocked: task.blocked,
            detail: input.detail,
        };
        persist_state_change(&self.log_path, event_kind, payload)
    }
}

fn validate_create_input(input: &TaskCreateInput) -> TaskEngineResult<()> {
    for (field, value) in [
        ("team_id", input.team_id.as_str()),
        ("created_by", input.created_by.as_str()),
        ("target", input.target.as_str()),
        ("title", input.title.as_str()),
        ("body", input.body.as_str()),
    ] {
        require_non_empty(field, value)?;
    }
    Ok(())
}

fn validate_claim_input(input: &TaskClaimInput) -> TaskEngineResult<()> {
    for (field, value) in [
        ("worker_name", input.worker_name.as_str()),
        ("worker_role", input.worker_role.as_str()),
    ] {
        require_non_empty(field, value)?;
    }
    Ok(())
}

fn validate_transition_input(input: &TaskTransitionInput) -> TaskEngineResult<()> {
    for (field, value) in [
        ("task_id", input.task_id.as_str()),
        ("actor", input.actor.as_str()),
        ("detail", input.detail.as_str()),
    ] {
        require_non_empty(field, value)?;
    }
    Ok(())
}

fn require_non_empty(field: &str, value: &str) -> TaskEngineResult<()> {
    if value.trim().is_empty() {
        Err(TaskEngineError::Validation {
            reason: format!("{field} must not be empty"),
        })
    } else {
        Ok(())
    }
}

fn validate_transition(
    current: TaskStatus,
    next: TaskStatus,
    task_id: &str,
) -> TaskEngineResult<()> {
    let valid = matches!(
        (current, next),
        (TaskStatus::Queued, TaskStatus::Running)
            | (TaskStatus::Queued, TaskStatus::Done)
            | (TaskStatus::Queued, TaskStatus::Error)
            | (TaskStatus::Running, TaskStatus::Done)
            | (TaskStatus::Running, TaskStatus::Error)
    );
    if valid {
        Ok(())
    } else {
        Err(TaskEngineError::InvalidTransition {
            task_id: task_id.to_owned(),
            reason: format!(
                "cannot transition from {} to {}",
                current.label(),
                next.label()
            ),
        })
    }
}

fn persist_state_change(
    log_path: &Path,
    event_kind: &str,
    payload: TaskEventPayload,
) -> TaskEngineResult<TaskStateChanged> {
    let status = payload.status;
    let task_id = payload.task_id.clone();
    let receipt = persist_task_event(log_path, event_kind, &payload)?;
    Ok(TaskStateChanged {
        task_id,
        status,
        event_id: receipt.event_id,
        sequence: receipt.sequence,
    })
}

fn next_task_id(board: &TaskBoard) -> String {
    format!("AT-{:06}", board.task_count + 1)
}

fn claim_scope_matches(task: &crate::task::model::TaskRecord, input: &TaskClaimInput) -> bool {
    match task.target_kind {
        crate::task::model::TaskTargetKind::Agent => task.target == input.worker_name,
        crate::task::model::TaskTargetKind::Role => task.target == input.worker_role,
    }
}

fn claim_scope_label(
    task: &crate::task::model::TaskRecord,
    input: &TaskClaimInput,
) -> &'static str {
    match task.target_kind {
        crate::task::model::TaskTargetKind::Agent if task.target == input.worker_name => "assigned",
        crate::task::model::TaskTargetKind::Role if task.target == input.worker_role => {
            "role-matching"
        }
        _ => "unknown",
    }
}
