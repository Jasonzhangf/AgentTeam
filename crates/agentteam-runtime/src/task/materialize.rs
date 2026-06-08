use std::collections::BTreeMap;
use std::path::Path;

use agentteam_persist::{replay_event_log, PersistedEventRecord};

use crate::task::error::{persistence_error, TaskEngineError, TaskEngineResult};
use crate::task::model::{TaskBoard, TaskEventPayload, TaskRecord, TaskStatus};
use crate::TASK_ENGINE_FEATURE_ID;

pub fn materialize_task_board(log_path: impl AsRef<Path>) -> TaskEngineResult<TaskBoard> {
    let replayed = replay_event_log(log_path, 0).map_err(persistence_error)?;
    let mut tasks = BTreeMap::new();
    let mut latest_sequence = 0;
    for record in replayed.events {
        latest_sequence = latest_sequence.max(record.sequence);
        if record.feature_id == TASK_ENGINE_FEATURE_ID {
            apply_task_event(&mut tasks, record)?;
        }
    }
    let tasks = tasks.into_values().collect::<Vec<_>>();
    Ok(TaskBoard {
        task_count: tasks.len(),
        tasks,
        latest_sequence,
    })
}

fn apply_task_event(
    tasks: &mut BTreeMap<String, TaskRecord>,
    record: PersistedEventRecord,
) -> TaskEngineResult<()> {
    let payload = decode_payload(&record)?;
    if record.event_kind == "task_created" {
        insert_created(tasks, record, payload)
    } else if record.event_kind == "task_running"
        || record.event_kind == "task_done"
        || record.event_kind == "task_error"
    {
        apply_transition(tasks, record, payload)
    } else {
        Ok(())
    }
}

fn insert_created(
    tasks: &mut BTreeMap<String, TaskRecord>,
    record: PersistedEventRecord,
    payload: TaskEventPayload,
) -> TaskEngineResult<()> {
    if tasks.contains_key(&payload.task_id) {
        return Err(TaskEngineError::Validation {
            reason: format!("duplicate task event for {}", payload.task_id),
        });
    }
    tasks.insert(
        payload.task_id.clone(),
        TaskRecord {
            task_id: payload.task_id,
            team_id: payload.team_id,
            status: TaskStatus::Queued,
            target_kind: payload.target_kind,
            target: payload.target,
            title: payload.title,
            body: payload.body,
            created_by: payload.actor.clone(),
            latest_actor: payload.actor,
            latest_detail: payload.detail,
            latest_event_id: record.event_id,
            latest_sequence: record.sequence,
        },
    );
    Ok(())
}

fn apply_transition(
    tasks: &mut BTreeMap<String, TaskRecord>,
    record: PersistedEventRecord,
    payload: TaskEventPayload,
) -> TaskEngineResult<()> {
    let Some(task) = tasks.get_mut(&payload.task_id) else {
        return Err(TaskEngineError::Validation {
            reason: format!("transition references unknown task {}", payload.task_id),
        });
    };
    task.status = payload.status;
    task.latest_actor = payload.actor;
    task.latest_detail = payload.detail;
    task.latest_event_id = record.event_id;
    task.latest_sequence = record.sequence;
    Ok(())
}

fn decode_payload(record: &PersistedEventRecord) -> TaskEngineResult<TaskEventPayload> {
    serde_json::from_str(&record.payload_json).map_err(|error| TaskEngineError::Validation {
        reason: format!(
            "failed to decode task event {} payload: {error}",
            record.event_id
        ),
    })
}
