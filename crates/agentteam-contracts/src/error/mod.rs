use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "error.center";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Fatal,
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamErr01FaultFact {
    pub module: String,
    pub class: String,
    pub specific: String,
    pub detail: String,
}

impl TeamErr01FaultFact {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Team", "Err", 1, "FaultFact");

    pub fn new(
        module: impl Into<String>,
        class: impl Into<String>,
        specific: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            module: module.into(),
            class: class.into(),
            specific: specific.into(),
            detail: detail.into(),
        }
    }

    pub fn classify(self, severity: ErrorSeverity, code: impl Into<String>) -> TeamErr02Classified {
        TeamErr02Classified {
            module: self.module,
            class: self.class,
            specific: self.specific,
            detail: self.detail,
            severity,
            code: code.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamErr02Classified {
    pub module: String,
    pub class: String,
    pub specific: String,
    pub detail: String,
    pub severity: ErrorSeverity,
    pub code: String,
}

impl TeamErr02Classified {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Team", "Err", 2, "Classified");

    pub fn link_evidence(self, evidence_id: impl Into<String>) -> TeamErr02EvidenceLinked {
        TeamErr02EvidenceLinked {
            module: self.module,
            class: self.class,
            specific: self.specific,
            detail: self.detail,
            severity: self.severity,
            code: self.code,
            evidence_id: evidence_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamErr02EvidenceLinked {
    pub module: String,
    pub class: String,
    pub specific: String,
    pub detail: String,
    pub severity: ErrorSeverity,
    pub code: String,
    pub evidence_id: String,
}

impl TeamErr02EvidenceLinked {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Team", "Err", 2, "EvidenceLinked");

    pub fn persist_as_event(
        self,
        event_id: impl Into<String>,
        receipt_id: impl Into<String>,
    ) -> TeamErr03RuntimeEvent {
        TeamErr03RuntimeEvent {
            event_id: event_id.into(),
            receipt_id: receipt_id.into(),
            code: self.code,
            severity: self.severity,
            evidence_id: self.evidence_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamErr03RuntimeEvent {
    pub event_id: String,
    pub receipt_id: String,
    pub code: String,
    pub severity: ErrorSeverity,
    pub evidence_id: String,
}

impl TeamErr03RuntimeEvent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Team", "Err", 3, "RuntimeEvent");

    pub fn project(self) -> TeamErr04Projection {
        TeamErr04Projection {
            code: self.code,
            severity: self.severity,
            evidence_id: self.evidence_id,
            event_id: self.event_id,
            receipt_id: self.receipt_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamErr04Projection {
    pub code: String,
    pub severity: ErrorSeverity,
    pub evidence_id: String,
    pub event_id: String,
    pub receipt_id: String,
}

impl TeamErr04Projection {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Team", "Err", 4, "Projection");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_chain_keeps_evidence_and_receipt() {
        let projection = TeamErr01FaultFact::new(
            "config",
            "validation",
            "missing_project",
            "project.slug is required",
        )
        .classify(
            ErrorSeverity::Error,
            "config.validation.missing_project.20260608T000000Z.000001",
        )
        .link_evidence("evidence-1")
        .persist_as_event("event-1", "receipt-1")
        .project();

        assert_eq!(projection.evidence_id, "evidence-1");
        assert_eq!(projection.receipt_id, "receipt-1");
        assert_eq!(TeamErr01FaultFact::NODE.number, 1);
        assert_eq!(TeamErr04Projection::NODE.number, 4);
    }

    #[test]
    fn error_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "error.center");
    }
}
