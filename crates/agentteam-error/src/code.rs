use agentteam_contracts::error::ErrorSeverity;
use agentteam_contracts::event_hash::event_payload_hash;

use crate::error::{ErrorCenterError, ErrorCenterResult};
use crate::model::ErrorCodeSeed;

pub fn generate_error_code(
    module: &str,
    class: &str,
    specific: &str,
    seed: &ErrorCodeSeed,
) -> ErrorCenterResult<String> {
    validate_segment("module", module)?;
    validate_segment("class", class)?;
    validate_segment("specific", specific)?;
    validate_timestamp(&seed.timestamp_utc)?;
    if seed.sequence == 0 {
        return Err(ErrorCenterError::Validation {
            reason: "error sequence must start at 1".to_owned(),
        });
    }
    Ok(format!(
        "{module}.{class}.{specific}.{}.{:06}",
        seed.timestamp_utc, seed.sequence
    ))
}

pub fn evidence_id_for_code(code: &str) -> ErrorCenterResult<String> {
    if code.trim().is_empty() {
        return Err(ErrorCenterError::Validation {
            reason: "error code must not be empty before evidence id generation".to_owned(),
        });
    }
    Ok(format!("evidence-{}", event_payload_hash(code)))
}

pub fn severity_label(severity: ErrorSeverity) -> &'static str {
    match severity {
        ErrorSeverity::Fatal => "fatal",
        ErrorSeverity::Error => "error",
        ErrorSeverity::Warn => "warn",
        ErrorSeverity::Info => "info",
    }
}

fn validate_segment(field: &str, value: &str) -> ErrorCenterResult<()> {
    if value.is_empty() {
        return Err(ErrorCenterError::Validation {
            reason: format!("{field} must not be empty"),
        });
    }
    if value
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        Ok(())
    } else {
        Err(ErrorCenterError::Validation {
            reason: format!("{field} must use [a-z0-9_] only"),
        })
    }
}

fn validate_timestamp(timestamp: &str) -> ErrorCenterResult<()> {
    let bytes = timestamp.as_bytes();
    let valid = bytes.len() == 16
        && bytes[8] == b'T'
        && bytes[15] == b'Z'
        && bytes[..8].iter().all(u8::is_ascii_digit)
        && bytes[9..15].iter().all(u8::is_ascii_digit);
    if valid {
        Ok(())
    } else {
        Err(ErrorCenterError::Validation {
            reason: "timestamp must use YYYYMMDDTHHMMSSZ".to_owned(),
        })
    }
}
