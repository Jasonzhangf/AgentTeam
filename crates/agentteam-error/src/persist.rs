use std::path::Path;

use agentteam_contracts::error::{TeamErr02EvidenceLinked, TeamErr03RuntimeEvent};
use agentteam_contracts::event_hash::event_payload_hash;
use agentteam_persist::{append_event_log, PersistedEventDraft};

use crate::code::severity_label;
use crate::error::{persistence_error, ErrorCenterError, ErrorCenterResult};
use crate::model::ErrorEventPayload;
use crate::FEATURE_ID;

pub fn persist_error_event(
    log_path: impl AsRef<Path>,
    linked: TeamErr02EvidenceLinked,
) -> ErrorCenterResult<TeamErr03RuntimeEvent> {
    let payload = payload_for_linked_error(&linked);
    let payload_json = encode_payload(&payload)?;
    let receipt = append_event_log(
        log_path,
        PersistedEventDraft {
            feature_id: FEATURE_ID.to_owned(),
            event_kind: "framework_error".to_owned(),
            payload_hash: event_payload_hash(&payload_json),
            payload_json,
        },
    )
    .map_err(persistence_error)?;
    let receipt_id = receipt_id_for_sequence(receipt.sequence);
    Ok(linked.persist_as_event(receipt.event_id, receipt_id))
}

fn payload_for_linked_error(linked: &TeamErr02EvidenceLinked) -> ErrorEventPayload {
    ErrorEventPayload {
        code: linked.code.clone(),
        severity: severity_label(linked.severity).to_owned(),
        evidence_id: linked.evidence_id.clone(),
        module: linked.module.clone(),
        class: linked.class.clone(),
        specific: linked.specific.clone(),
        detail: linked.detail.clone(),
    }
}

fn encode_payload(payload: &ErrorEventPayload) -> ErrorCenterResult<String> {
    serde_json::to_string(payload).map_err(|error| ErrorCenterError::Validation {
        reason: format!("failed to encode error event payload: {error}"),
    })
}

fn receipt_id_for_sequence(sequence: u64) -> String {
    format!("receipt-{sequence:020}")
}
