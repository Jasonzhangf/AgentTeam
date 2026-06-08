use std::path::Path;

use agentteam_contracts::event_hash::event_payload_hash;
use agentteam_contracts::persist::PersistResp03AppendReceipt;
use agentteam_persist::{append_event_log, PersistedEventDraft};
use serde::Serialize;

use crate::task::error::{persistence_error, TaskEngineError, TaskEngineResult};
use crate::TASK_ENGINE_FEATURE_ID;

pub fn persist_task_event<T: Serialize>(
    log_path: impl AsRef<Path>,
    event_kind: &str,
    payload: &T,
) -> TaskEngineResult<PersistResp03AppendReceipt> {
    let payload_json = encode_payload(payload)?;
    append_event_log(
        log_path,
        PersistedEventDraft {
            feature_id: TASK_ENGINE_FEATURE_ID.to_owned(),
            event_kind: event_kind.to_owned(),
            payload_hash: event_payload_hash(&payload_json),
            payload_json,
        },
    )
    .map_err(persistence_error)
}

fn encode_payload<T: Serialize>(payload: &T) -> TaskEngineResult<String> {
    serde_json::to_string(payload).map_err(|error| TaskEngineError::Validation {
        reason: format!("failed to encode task event payload: {error}"),
    })
}
