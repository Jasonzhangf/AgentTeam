use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedEventRecord {
    pub sequence: u64,
    pub event_id: String,
    pub feature_id: String,
    pub event_kind: String,
    pub payload_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedEventDraft {
    pub feature_id: String,
    pub event_kind: String,
    pub payload_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayedEventLog {
    pub log_path: String,
    pub from_sequence: u64,
    pub events: Vec<PersistedEventRecord>,
}

pub fn event_id_for_sequence(sequence: u64) -> String {
    format!("event-{sequence:020}")
}
