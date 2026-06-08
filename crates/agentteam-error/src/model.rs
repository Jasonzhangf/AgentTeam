use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCodeSeed {
    pub timestamp_utc: String,
    pub sequence: u64,
}

impl ErrorCodeSeed {
    pub fn new(timestamp_utc: impl Into<String>, sequence: u64) -> Self {
        Self {
            timestamp_utc: timestamp_utc.into(),
            sequence,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorEventPayload {
    pub code: String,
    pub severity: String,
    pub evidence_id: String,
    pub module: String,
    pub class: String,
    pub specific: String,
    pub detail: String,
}
