use agentteam_contracts::persist::{PersistReq04Replay, PersistResp05MaterializedState};

use crate::model::ReplayedEventLog;

pub fn materialize_event_log(replayed: &ReplayedEventLog) -> PersistResp05MaterializedState {
    let latest_sequence = replayed
        .events
        .last()
        .map_or(replayed.from_sequence, |record| record.sequence);
    let snapshot_id = snapshot_id_for_sequence(latest_sequence);
    PersistReq04Replay::new(replayed.log_path.clone(), replayed.from_sequence)
        .materialize(latest_sequence, snapshot_id)
}

fn snapshot_id_for_sequence(sequence: u64) -> String {
    format!("snapshot-{sequence:020}")
}
