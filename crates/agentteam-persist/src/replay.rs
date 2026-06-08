use std::fs;
use std::path::Path;

use crate::error::{PersistenceError, PersistenceResult};
use crate::model::{PersistedEventRecord, ReplayedEventLog};

pub fn replay_event_log(
    log_path: impl AsRef<Path>,
    from_sequence: u64,
) -> PersistenceResult<ReplayedEventLog> {
    let path = log_path.as_ref();
    if !path.exists() {
        return Ok(ReplayedEventLog {
            log_path: path.display().to_string(),
            from_sequence,
            events: Vec::new(),
        });
    }

    let content = fs::read_to_string(path).map_err(|error| PersistenceError::Io {
        path: path.display().to_string(),
        reason: error.to_string(),
    })?;
    let mut events = Vec::new();
    let mut expected_sequence = 1;
    for (index, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let record = parse_record(path, index + 1, line)?;
        validate_record_sequence(path, index + 1, &record, expected_sequence)?;
        expected_sequence += 1;
        if record.sequence >= from_sequence {
            events.push(record);
        }
    }
    Ok(ReplayedEventLog {
        log_path: path.display().to_string(),
        from_sequence,
        events,
    })
}

fn validate_record_sequence(
    path: &Path,
    line_number: usize,
    record: &PersistedEventRecord,
    expected_sequence: u64,
) -> PersistenceResult<()> {
    if record.sequence == expected_sequence {
        Ok(())
    } else {
        Err(PersistenceError::Corruption {
            path: path.display().to_string(),
            line: line_number,
            reason: format!(
                "event sequence mismatch: expected {expected_sequence}, got {}",
                record.sequence
            ),
        })
    }
}

fn parse_record(
    path: &Path,
    line_number: usize,
    line: &str,
) -> PersistenceResult<PersistedEventRecord> {
    serde_json::from_str(line).map_err(|error| PersistenceError::Corruption {
        path: path.display().to_string(),
        line: line_number,
        reason: error.to_string(),
    })
}
