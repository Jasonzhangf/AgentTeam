use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FlowReportResult {
    pub log_path: String,
    pub event_count: usize,
    pub latest_sequence: u64,
    pub unknown_event_count: usize,
    pub steps: Vec<FlowReportStepResult>,
    pub ascii_flow: String,
    pub mermaid_flow: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FlowReportStepResult {
    pub sequence: u64,
    pub event_id: String,
    pub event_kind: String,
    pub feature_id: String,
    pub from: String,
    pub to: String,
    pub kind: String,
    pub label: String,
}
