use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "persist.event_log";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistReq01EventDraft {
    pub feature_id: String,
    pub event_kind: String,
    pub payload_hash: String,
}

impl PersistReq01EventDraft {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Persist", "Req", 1, "EventDraft");

    pub fn new(
        feature_id: impl Into<String>,
        event_kind: impl Into<String>,
        payload_hash: impl Into<String>,
    ) -> Self {
        Self {
            feature_id: feature_id.into(),
            event_kind: event_kind.into(),
            payload_hash: payload_hash.into(),
        }
    }

    pub fn validate(self, event_id: impl Into<String>) -> PersistReq02ValidatedEvent {
        PersistReq02ValidatedEvent {
            event_id: event_id.into(),
            feature_id: self.feature_id,
            event_kind: self.event_kind,
            payload_hash: self.payload_hash,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistReq02ValidatedEvent {
    pub event_id: String,
    pub feature_id: String,
    pub event_kind: String,
    pub payload_hash: String,
}

impl PersistReq02ValidatedEvent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Persist", "Req", 2, "ValidatedEvent");

    pub fn append_receipt(
        self,
        sequence: u64,
        log_path: impl Into<String>,
    ) -> PersistResp03AppendReceipt {
        PersistResp03AppendReceipt {
            event_id: self.event_id,
            sequence,
            log_path: log_path.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistResp03AppendReceipt {
    pub event_id: String,
    pub sequence: u64,
    pub log_path: String,
}

impl PersistResp03AppendReceipt {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Persist", "Resp", 3, "AppendReceipt");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistReq04Replay {
    pub log_path: String,
    pub from_sequence: u64,
}

impl PersistReq04Replay {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Persist", "Req", 4, "Replay");

    pub fn new(log_path: impl Into<String>, from_sequence: u64) -> Self {
        Self {
            log_path: log_path.into(),
            from_sequence,
        }
    }

    pub fn materialize(
        self,
        latest_sequence: u64,
        snapshot_id: impl Into<String>,
    ) -> PersistResp05MaterializedState {
        PersistResp05MaterializedState {
            log_path: self.log_path,
            from_sequence: self.from_sequence,
            latest_sequence,
            snapshot_id: snapshot_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistResp05MaterializedState {
    pub log_path: String,
    pub from_sequence: u64,
    pub latest_sequence: u64,
    pub snapshot_id: String,
}

impl PersistResp05MaterializedState {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Persist", "Resp", 5, "MaterializedState");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persist_append_chain_uses_adjacent_nodes() {
        let receipt = PersistReq01EventDraft::new("debug.center", "debug_bundle", "hash-1")
            .validate("event-1")
            .append_receipt(1, "/tmp/agentteam/events.jsonl");

        assert_eq!(receipt.event_id, "event-1");
        assert_eq!(receipt.sequence, 1);
        assert_eq!(PersistReq01EventDraft::NODE.number, 1);
        assert_eq!(PersistResp03AppendReceipt::NODE.number, 3);
    }

    #[test]
    fn persist_replay_chain_materializes_state() {
        let state =
            PersistReq04Replay::new("/tmp/agentteam/events.jsonl", 0).materialize(7, "snapshot-7");

        assert_eq!(state.latest_sequence, 7);
        assert_eq!(state.snapshot_id, "snapshot-7");
    }

    #[test]
    fn persist_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "persist.event_log");
    }
}
