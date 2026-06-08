use std::path::Path;

use agentteam_contracts::event_hash::event_payload_hash;
use agentteam_contracts::persist::PersistResp03AppendReceipt;
use agentteam_persist::{append_event_log, PersistedEventDraft};
use serde::Serialize;

use crate::error::{persistence_error, DebugError, DebugResult};
use crate::FEATURE_ID;

pub fn persist_debug_bundle<T: Serialize>(
    log_path: impl AsRef<Path>,
    payload: &T,
) -> DebugResult<PersistResp03AppendReceipt> {
    let payload_json = encode_payload(payload)?;
    append_event_log(
        log_path,
        PersistedEventDraft {
            feature_id: FEATURE_ID.to_owned(),
            event_kind: "debug_bundle".to_owned(),
            payload_hash: event_payload_hash(&payload_json),
            payload_json,
        },
    )
    .map_err(persistence_error)
}

fn encode_payload<T: Serialize>(payload: &T) -> DebugResult<String> {
    serde_json::to_string(payload).map_err(|error| DebugError::Validation {
        reason: format!("failed to encode debug bundle payload: {error}"),
    })
}
