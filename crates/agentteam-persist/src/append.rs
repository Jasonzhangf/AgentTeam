use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use agentteam_contracts::persist::{PersistReq01EventDraft, PersistResp03AppendReceipt};
use fs2::FileExt;

use crate::error::{PersistenceError, PersistenceResult};
use crate::model::{event_id_for_sequence, PersistedEventDraft, PersistedEventRecord};
use crate::replay::{parse_record, validate_record_sequence};

pub fn append_event_log(
    log_path: impl AsRef<Path>,
    draft: PersistedEventDraft,
) -> PersistenceResult<PersistResp03AppendReceipt> {
    let path = log_path.as_ref();
    validate_draft(&draft)?;
    ensure_parent_dir(path)?;
    let mut file = open_locked_append_file(path)?;
    let next_sequence = locked_next_sequence(path, &mut file)?;
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
        payload_json: draft.payload_json.clone(),
        payload_hash: validated.payload_hash.clone(),
    };
    let encoded = encode_record(&record)?;
    write_locked_record(path, &mut file, &encoded)?;
    Ok(validated.append_receipt(next_sequence, path.display().to_string()))
}

fn validate_draft(draft: &PersistedEventDraft) -> PersistenceResult<()> {
    for (field, value) in [
        ("feature_id", draft.feature_id.as_str()),
        ("event_kind", draft.event_kind.as_str()),
        ("payload_json", draft.payload_json.as_str()),
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

fn encode_record(record: &PersistedEventRecord) -> PersistenceResult<String> {
    serde_json::to_string(record).map_err(|error| PersistenceError::Validation {
        reason: format!("failed to encode event record: {error}"),
    })
}

fn open_locked_append_file(path: &Path) -> PersistenceResult<File> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .map_err(|error| io_error(path, error))?;
    file.lock_exclusive()
        .map_err(|error| io_error(path, error))?;
    Ok(file)
}

fn locked_next_sequence(path: &Path, file: &mut File) -> PersistenceResult<u64> {
    file.seek(SeekFrom::Start(0))
        .map_err(|error| io_error(path, error))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|error| io_error(path, error))?;
    next_sequence_from_locked_content(path, &content)
}

fn next_sequence_from_locked_content(path: &Path, content: &str) -> PersistenceResult<u64> {
    let mut expected_sequence = 1;
    let mut latest_sequence = None;
    for (index, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let record = parse_record(path, index + 1, line)?;
        validate_record_sequence(path, index + 1, &record, expected_sequence)?;
        expected_sequence += 1;
        latest_sequence = Some(record.sequence);
    }
    latest_sequence.map_or(Ok(1), next_sequence_after)
}

fn next_sequence_after(sequence: u64) -> PersistenceResult<u64> {
    sequence
        .checked_add(1)
        .ok_or_else(|| PersistenceError::Validation {
            reason: "event sequence overflow".to_owned(),
        })
}

fn write_locked_record(path: &Path, file: &mut File, encoded: &str) -> PersistenceResult<()> {
    file.seek(SeekFrom::End(0))
        .map_err(|error| io_error(path, error))?;
    file.write_all(encoded.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.flush())
        .map_err(|error| io_error(path, error))
}

fn io_error(path: &Path, error: std::io::Error) -> PersistenceError {
    PersistenceError::Io {
        path: path.display().to_string(),
        reason: error.to_string(),
    }
}
