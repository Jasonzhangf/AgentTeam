use std::path::Path;

use agentteam_contracts::persist::PersistResp03AppendReceipt;
use agentteam_persist::{append_event_log, PersistedEventDraft};
use serde::Serialize;

use crate::error::{persistence_error, ResourceResult};
use crate::FEATURE_ID;

pub fn persist_resource_event<T: Serialize>(
    log_path: impl AsRef<Path>,
    event_kind: &str,
    payload: &T,
) -> ResourceResult<PersistResp03AppendReceipt> {
    let payload_json = encode_payload(payload)?;
    append_event_log(
        log_path,
        PersistedEventDraft {
            feature_id: FEATURE_ID.to_owned(),
            event_kind: event_kind.to_owned(),
            payload_hash: payload_hash(&payload_json),
            payload_json,
        },
    )
    .map_err(persistence_error)
}

fn encode_payload<T: Serialize>(payload: &T) -> ResourceResult<String> {
    serde_json::to_string(payload).map_err(|error| crate::error::ResourceError::Validation {
        reason: format!("failed to encode resource event payload: {error}"),
    })
}

fn payload_hash(payload: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in payload.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64-{hash:016x}")
}
