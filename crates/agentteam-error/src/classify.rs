use std::path::Path;

use agentteam_contracts::error::{
    ErrorSeverity, TeamErr01FaultFact, TeamErr02Classified, TeamErr02EvidenceLinked,
    TeamErr03RuntimeEvent, TeamErr04Projection,
};

use crate::code::{evidence_id_for_code, generate_error_code};
use crate::error::{ErrorCenterError, ErrorCenterResult};
use crate::model::ErrorCodeSeed;
use crate::persist::persist_error_event;

pub fn handle_framework_fault(
    log_path: impl AsRef<Path>,
    fault: TeamErr01FaultFact,
    severity: ErrorSeverity,
    seed: ErrorCodeSeed,
) -> ErrorCenterResult<TeamErr04Projection> {
    let classified = classify_fault(fault, severity, &seed)?;
    let linked = link_error_evidence(classified)?;
    let event = persist_error_event(log_path, linked)?;
    Ok(project_error(event))
}

pub fn classify_fault(
    fault: TeamErr01FaultFact,
    severity: ErrorSeverity,
    seed: &ErrorCodeSeed,
) -> ErrorCenterResult<TeamErr02Classified> {
    reject_normal_task_error(&fault)?;
    let code = generate_error_code(&fault.module, &fault.class, &fault.specific, seed)?;
    Ok(fault.classify(severity, code))
}

pub fn link_error_evidence(
    classified: TeamErr02Classified,
) -> ErrorCenterResult<TeamErr02EvidenceLinked> {
    let evidence_id = evidence_id_for_code(&classified.code)?;
    Ok(classified.link_evidence(evidence_id))
}

pub fn project_error(event: TeamErr03RuntimeEvent) -> TeamErr04Projection {
    event.project()
}

fn reject_normal_task_error(fault: &TeamErr01FaultFact) -> ErrorCenterResult<()> {
    if fault.module == "task" && fault.class == "agent_task_error" {
        Err(ErrorCenterError::Validation {
            reason: "normal agent-reported task error belongs to Task Engine".to_owned(),
        })
    } else {
        Ok(())
    }
}
