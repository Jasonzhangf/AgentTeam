use crate::{parse_cli_args, render_local_result_json};
use agentteam_runtime::local::LocalCommandResult;
use agentteam_runtime::local_projection::FlowReportResult;

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn parses_report_flow_intent() {
    let intent = parse_cli_args(strings(&[
        "report",
        "flow",
        "--runtime-home",
        "target/agentteam-smoke",
        "--json",
    ]))
    .unwrap();
    assert_eq!(intent.command_name(), "report.flow");
}

#[test]
fn render_report_flow_result_json_uses_report_command_name() {
    let result = LocalCommandResult::ReportFlow {
        report: FlowReportResult {
            log_path: "target/agentteam-smoke/events/agentteam.jsonl".to_owned(),
            event_count: 0,
            latest_sequence: 0,
            unknown_event_count: 0,
            steps: Vec::new(),
            ascii_flow: "event log is empty".to_owned(),
            mermaid_flow: "sequenceDiagram\n  Note over AgentTeam: event log is empty".to_owned(),
        },
    };

    let rendered = render_local_result_json(&result).unwrap();
    assert!(rendered.contains("\"command_name\":\"report.flow\""));
    assert!(rendered.contains("\"ascii_flow\":\"event log is empty\""));
}
