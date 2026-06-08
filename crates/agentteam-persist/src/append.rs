use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use agentteam_contracts::persist::{PersistReq01EventDraft, PersistResp03AppendReceipt};

use crate::error::{PersistenceError, PersistenceResult};
use crate::model::{event_id_for_sequence, PersistedEventDraft, PersistedEventRecord};
use crate::replay::replay_event_log;

pub fn append_event_log(
    log_path: impl AsRef<Path>,
    draft: PersistedEventDraft,
) -> PersistenceResult<PersistResp03AppendReceipt> {
    let path = log_path.as_ref();
    validate_draft(&draft)?;
    ensure_parent_dir(path)?;
    let next_sequence = next_sequence(path)?;
    let event_id = event_id_for_sequence(next_sequence);
    let validated = PersistReq01EventDraft::new(
        draft.feature_id.clone(),
        draft.event_kind.clone(),
        draft.payload_hash.clone(),
    )
    .validate(event_id.clone());
    let record = PersistedEventRecord {
        sequence: next_sequence,
        event_id: validated.event_id.clone(),
        feature_id: validated.feature_id.clone(),
        event_kind: validated.event_kind.clone(),
        payload_hash: validated.payload_hash.clone(),
    };
    let encoded = encode_record(&record)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| io_error(path, error))?;
    file.write_all(encoded.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.flush())
        .map_err(|error| io_error(path, error))?;
    Ok(validated.append_receipt(next_sequence, path.display().to_string()))
}

fn validate_draft(draft: &PersistedEventDraft) -> PersistenceResult<()> {
    for (field, value) in [
        ("feature_id", draft.feature_id.as_str()),
        ("event_kind", draft.event_kind.as_str()),
        ("payload_hash", draft.payload_hash.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(PersistenceError::Validation {
                reason: format!("{field} must not be empty"),
            });
        }
    }
    Ok(())
}

fn ensure_parent_dir(path: &Path) -> PersistenceResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| io_error(parent, error))?;
    }
    Ok(())
}

fn next_sequence(path: &Path) -> PersistenceResult<u64> {
    if !path.exists() {
        return Ok(1);
    }
    let replayed = replay_event_log(path, 0)?;
    replayed.events.last().map_or(Ok(1), |record| {
        record
            .sequence
            .checked_add(1)
            .ok_or_else(|| PersistenceError::Validation {
                reason: "event sequence overflow".to_owned(),
            })
    })
}

fn encode_record(record: &PersistedEventRecord) -> PersistenceResult<String> {
    serde_json::to_string(record).map_err(|error| PersistenceError::Validation {
        reason: format!("failed to encode event record: {error}"),
    })
}

fn io_error(path: &Path, error: std::io::Error) -> PersistenceError {
    PersistenceError::Io {
        path: path.display().to_string(),
        reason: error.to_string(),
    }
}
