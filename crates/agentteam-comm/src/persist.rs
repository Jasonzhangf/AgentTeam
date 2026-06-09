use std::path::Path;

use agentteam_contracts::event_hash::event_payload_hash;
use agentteam_contracts::persist::PersistResp03AppendReceipt;
use agentteam_persist::{append_event_log, PersistedEventDraft};
use serde::Serialize;

use crate::error::{persistence_error, CommCenterError, CommCenterResult};
use crate::FEATURE_ID;

pub fn persist_delivery_event<T: Serialize>(
    log_path: impl AsRef<Path>,
    event_kind: &str,
    payload: &T,
) -> CommCenterResult<PersistResp03AppendReceipt> {
    let payload_json =
        serde_json::to_string(payload).map_err(|error| CommCenterError::Validation {
            reason: format!("failed to encode delivery event payload: {error}"),
        })?;
    append_event_log(
        log_path,
        PersistedEventDraft {
            feature_id: FEATURE_ID.to_owned(),
            event_kind: event_kind.to_owned(),
            payload_hash: event_payload_hash(&payload_json),
            payload_json,
        },
    )
    .map_err(persistence_error)
}
