use std::collections::BTreeSet;
use std::path::Path;

use agentteam_persist::{replay_event_log, PersistedEventRecord};
use serde::Deserialize;

use crate::local_projection::{LocalCommandError, LocalCommandResult};
use crate::local_report_projection::{FlowReportResult, FlowReportStepResult};

pub fn execute_report_flow(runtime_home: String) -> Result<LocalCommandResult, LocalCommandError> {
    let event_log = crate::local::event_log_path(&runtime_home);
    let replayed = replay_event_log(&event_log, 0).map_err(report_persistence_error)?;
    Ok(LocalCommandResult::ReportFlow {
        report: build_flow_report(&event_log, replayed.events)?,
    })
}

pub fn build_flow_report(
    event_log: &Path,
    events: Vec<PersistedEventRecord>,
) -> Result<FlowReportResult, LocalCommandError> {
    let latest_sequence = events.last().map(|event| event.sequence).unwrap_or(0);
    let steps = events
        .iter()
        .map(classify_flow_step)
        .collect::<Result<Vec<_>, _>>()?;
    let unknown_event_count = steps
        .iter()
        .filter(|step| step.kind == "generic_event")
        .count();
    Ok(FlowReportResult {
        log_path: event_log.display().to_string(),
        event_count: events.len(),
        latest_sequence,
        unknown_event_count,
        steps: steps.clone(),
        ascii_flow: render_ascii_flow(&steps),
        mermaid_flow: render_mermaid_flow(&steps),
    })
}

pub fn classify_flow_step(
    event: &PersistedEventRecord,
) -> Result<FlowReportStepResult, LocalCommandError> {
    let payload = parse_payload(event)?;
    let step = match event.event_kind.as_str() {
        "comm_ready_report_delivery" => ready_step(event, &payload),
        "comm_message_delivery" => message_step(event, &payload),
        "comm_broadcast_delivery" => broadcast_step(event, &payload),
        "task_created" | "task_claimed" | "task_running" | "task_done" | "task_error" => {
            task_step(event, &payload)
        }
        "resource_acquire" | "resource_release" | "resource_leak" => resource_step(event, &payload),
        "debug_bundle" | "framework_error" => diagnostic_step(event, &payload),
        _ => generic_step(event, &payload),
    };
    Ok(step)
}

pub fn render_ascii_flow(steps: &[FlowReportStepResult]) -> String {
    if steps.is_empty() {
        return "event log is empty".to_owned();
    }
    steps
        .iter()
        .map(|step| {
            format!(
                "[{sequence:03}] {from} -> {to}: {label}",
                sequence = step.sequence,
                from = step.from,
                to = step.to,
                label = step.label
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render_mermaid_flow(steps: &[FlowReportStepResult]) -> String {
    let mut lines = vec!["sequenceDiagram".to_owned()];
    if steps.is_empty() {
        lines.push("  Note over AgentTeam: event log is empty".to_owned());
        return lines.join("\n");
    }
    let actors = unique_actors(steps);
    for actor in actors {
        lines.push(format!(
            "  participant {} as {}",
            mermaid_actor_id(&actor),
            mermaid_escape(&actor)
        ));
    }
    for step in steps {
        lines.push(format!(
            "  {}->>{}: {}",
            mermaid_actor_id(&step.from),
            mermaid_actor_id(&step.to),
            mermaid_escape(&step.label)
        ));
    }
    lines.join("\n")
}

fn parse_payload(event: &PersistedEventRecord) -> Result<FlowPayload, LocalCommandError> {
    serde_json::from_str(&event.payload_json).map_err(|error| LocalCommandError::Report {
        reason: format!(
            "event {} payload for {} is invalid JSON: {error}",
            event.event_id, event.event_kind
        ),
    })
}

fn ready_step(event: &PersistedEventRecord, payload: &FlowPayload) -> FlowReportStepResult {
    let agent = payload.agent_name.as_ref().or(payload.sender.as_ref());
    flow_step(
        event,
        payload_value(agent, "agent"),
        "Team",
        "ready_report",
        format!(
            "ready report: {}",
            payload_value(payload.body.as_ref(), "ready")
        ),
    )
}

fn message_step(event: &PersistedEventRecord, payload: &FlowPayload) -> FlowReportStepResult {
    flow_step(
        event,
        payload_value(payload.sender.as_ref(), "sender"),
        payload_value(payload.target.as_ref(), "target"),
        "message",
        format!(
            "{}: {}",
            payload_value(payload.action.as_ref(), "message"),
            payload_value(payload.body.as_ref(), "body")
        ),
    )
}

fn broadcast_step(event: &PersistedEventRecord, payload: &FlowPayload) -> FlowReportStepResult {
    let recipients = payload
        .members
        .as_ref()
        .map(|members| members.join(","))
        .unwrap_or_else(|| "team".to_owned());
    flow_step(
        event,
        payload_value(payload.sender.as_ref(), "sender"),
        recipients,
        "broadcast",
        format!(
            "broadcast {}: {}",
            payload_value(payload.action.as_ref(), "action"),
            payload_value(payload.body.as_ref(), "body")
        ),
    )
}

fn task_step(event: &PersistedEventRecord, payload: &FlowPayload) -> FlowReportStepResult {
    let to = match event.event_kind.as_str() {
        "task_created" => format!(
            "{}:{}",
            payload_value(payload.target_kind.as_ref(), "target"),
            payload_value(payload.target.as_ref(), "unknown")
        ),
        _ => "TaskBoard".to_owned(),
    };
    flow_step(
        event,
        payload_value(payload.actor.as_ref(), "actor"),
        to,
        "task",
        format!(
            "{} {} {}",
            payload_value(payload.task_id.as_ref(), "task"),
            payload_value(payload.status.as_ref(), "status"),
            payload_value(payload.detail.as_ref(), "detail")
        ),
    )
}

fn resource_step(event: &PersistedEventRecord, payload: &FlowPayload) -> FlowReportStepResult {
    flow_step(
        event,
        payload_value(payload.owner_module.as_ref(), "resource"),
        payload_value(payload.owner_entity_id.as_ref(), "owner"),
        "resource",
        format!(
            "{} {}",
            event.event_kind,
            payload_value(payload.resource_class.as_ref(), "resource")
        ),
    )
}

fn diagnostic_step(event: &PersistedEventRecord, payload: &FlowPayload) -> FlowReportStepResult {
    flow_step(
        event,
        payload_value(payload.module.as_ref(), "framework"),
        "Debug",
        "diagnostic",
        event.event_kind.clone(),
    )
}

fn generic_step(event: &PersistedEventRecord, payload: &FlowPayload) -> FlowReportStepResult {
    flow_step(
        event,
        payload_value(
            payload.actor.as_ref().or(payload.sender.as_ref()),
            &event.feature_id,
        ),
        "EventLog",
        "generic_event",
        format!("{} {}", event.feature_id, event.event_kind),
    )
}

fn flow_step(
    event: &PersistedEventRecord,
    from: impl Into<String>,
    to: impl Into<String>,
    kind: impl Into<String>,
    label: impl Into<String>,
) -> FlowReportStepResult {
    FlowReportStepResult {
        sequence: event.sequence,
        event_id: event.event_id.clone(),
        event_kind: event.event_kind.clone(),
        feature_id: event.feature_id.clone(),
        from: from.into(),
        to: to.into(),
        kind: kind.into(),
        label: label.into(),
    }
}

fn payload_value(value: Option<&String>, missing: &str) -> String {
    value
        .filter(|text| !text.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| format!("unknown-{missing}"))
}

fn unique_actors(steps: &[FlowReportStepResult]) -> BTreeSet<String> {
    let mut actors = BTreeSet::new();
    for step in steps {
        actors.insert(step.from.clone());
        actors.insert(step.to.clone());
    }
    actors
}

fn mermaid_actor_id(actor: &str) -> String {
    let mut id = String::from("A");
    for ch in actor.chars() {
        if ch.is_ascii_alphanumeric() {
            id.push(ch);
        } else {
            id.push('_');
        }
    }
    id
}

fn mermaid_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\n', " ")
}

fn report_persistence_error(error: agentteam_persist::PersistenceError) -> LocalCommandError {
    LocalCommandError::Report {
        reason: error.reason().to_owned(),
    }
}

#[derive(Debug, Deserialize)]
struct FlowPayload {
    sender: Option<String>,
    target: Option<String>,
    action: Option<String>,
    body: Option<String>,
    members: Option<Vec<String>>,
    agent_name: Option<String>,
    actor: Option<String>,
    task_id: Option<String>,
    status: Option<String>,
    target_kind: Option<String>,
    detail: Option<String>,
    owner_module: Option<String>,
    owner_entity_id: Option<String>,
    resource_class: Option<String>,
    module: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_renders_ready_message_and_task_flow() {
        let report = build_flow_report(
            Path::new("target/report/events/agentteam.jsonl"),
            vec![
                event(1, "comm_ready_report_delivery", r#"{"sender":"Alice","team_id":"default","agent_name":"Alice","body":"ready"}"#),
                event(2, "comm_message_delivery", r#"{"sender":"Kevin","target":"Alice","action":"assign","body":"Claim task"}"#),
                event(3, "task_created", r#"{"task_id":"AT-000001","team_id":"default","actor":"Kevin","status":"queued","target_kind":"role","target":"builder","title":"smoke","body":"body","priority":100,"blocked":false,"detail":"created"}"#),
                event(4, "task_done", r#"{"task_id":"AT-000001","team_id":"default","actor":"Alice","status":"done","target_kind":"role","target":"builder","title":"smoke","body":"body","priority":100,"blocked":false,"detail":"done"}"#),
            ],
        )
        .unwrap();

        assert_eq!(report.event_count, 4);
        assert_eq!(report.latest_sequence, 4);
        assert!(report.ascii_flow.contains("[002] Kevin -> Alice"));
        assert!(report.mermaid_flow.contains("sequenceDiagram"));
        assert!(report.mermaid_flow.contains("AKevin->>AAlice"));
    }

    #[test]
    fn invalid_payload_is_report_error() {
        let error = build_flow_report(
            Path::new("target/report/events/agentteam.jsonl"),
            vec![event(1, "task_created", "{not-json")],
        )
        .unwrap_err();

        match error {
            LocalCommandError::Report { reason } => assert!(reason.contains("invalid JSON")),
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn unknown_event_is_explicit_generic_step() {
        let report = build_flow_report(
            Path::new("target/report/events/agentteam.jsonl"),
            vec![event(1, "custom_event", r#"{"actor":"Alice"}"#)],
        )
        .unwrap();

        assert_eq!(report.unknown_event_count, 1);
        assert!(report.ascii_flow.contains("comm.center custom_event"));
    }

    fn event(sequence: u64, kind: &str, payload_json: &str) -> PersistedEventRecord {
        PersistedEventRecord {
            sequence,
            event_id: format!("event-{sequence:020}"),
            feature_id: "comm.center".to_owned(),
            event_kind: kind.to_owned(),
            payload_json: payload_json.to_owned(),
            payload_hash: "hash".to_owned(),
        }
    }
}
