use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use agentteam_contracts::comm::CommReq03DeliveryEnvelope;
use agentteam_persist::replay_event_log;

use crate::error::CommCenterError;
use crate::model::{
    CommReadyReportRequest, CommRouteRequest, CommTaskBoardQueryRequest, CommTaskClaimRequest,
    CommTeamBroadcastRequest,
};
use crate::route::{
    route_broadcast, route_message, route_ready_report, route_task_board_query, route_task_claim,
    send_broadcast, send_message, send_ready_report, CommCenter,
};
use crate::{persist_delivery_event, FEATURE_ID};

fn temp_log_path(test_name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("agentteam-{test_name}-{nanos}.jsonl"))
}

#[test]
fn route_message_accepts_non_empty_target() {
    let result =
        route_message(CommRouteRequest::new("Kevin", "Alice", "message", "hello")).unwrap();
    assert_eq!(result.target, "Alice");
    assert_eq!(result.action, "message");
}

#[test]
fn route_broadcast_accepts_exact_member_list() {
    let result = route_broadcast(CommTeamBroadcastRequest::new(
        "Kevin",
        "default",
        "broadcast",
        "hello",
        vec!["Alice".to_owned(), "Bob".to_owned()],
    ))
    .unwrap();
    assert_eq!(result.team_id, "default");
    assert_eq!(result.recipient_count, 2);
}

#[test]
fn route_ready_report_accepts_agent_name() {
    let result = route_ready_report(CommReadyReportRequest::new(
        "Kevin", "default", "Alice", "ready",
    ))
    .unwrap();
    assert_eq!(result.team_id, "default");
    assert_eq!(result.agent_name, "Alice");
}

#[test]
fn route_task_board_query_preserves_query() {
    let result = route_task_board_query(CommTaskBoardQueryRequest::new(
        "Kevin",
        "default",
        "board",
        "show board",
    ))
    .unwrap();
    assert_eq!(result.team_id, "default");
    assert_eq!(result.query, "board");
}

#[test]
fn route_task_claim_accepts_worker_scope() {
    let result = route_task_claim(CommTaskClaimRequest::new(
        "Alice",
        "default",
        "Alice",
        "builder",
        "claim next task",
    ))
    .unwrap();
    assert_eq!(result.team_id, "default");
    assert_eq!(result.worker_name, "Alice");
    assert_eq!(result.worker_role, "builder");
}

#[test]
fn route_message_rejects_empty_target() {
    let error = CommCenter::new()
        .route_message(CommRouteRequest::new("Kevin", "", "message", "hello"))
        .unwrap_err();
    assert!(matches!(error, CommCenterError::Validation { .. }));
}

#[test]
fn persist_delivery_event_writes_replayable_jsonl() {
    let path = temp_log_path("comm-delivery");
    let envelope = CommReq03DeliveryEnvelope {
        sender: "Kevin".to_owned(),
        target: "Alice".to_owned(),
        action: "message".to_owned(),
        body: "hello".to_owned(),
    };

    let receipt = persist_delivery_event(&path, "comm_message_delivery", &envelope).unwrap();
    assert_eq!(receipt.sequence, 1);
    assert_eq!(receipt.log_path, path.display().to_string());

    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 1);
    assert_eq!(replayed.events[0].feature_id, FEATURE_ID);
    assert_eq!(replayed.events[0].event_kind, "comm_message_delivery");
    assert_eq!(
        replayed.events[0].payload_json,
        serde_json::to_string(&envelope).unwrap()
    );
}

#[test]
fn persist_delivery_event_reports_persistence_failure() {
    let path = PathBuf::from(".");
    let envelope = CommReq03DeliveryEnvelope {
        sender: "Kevin".to_owned(),
        target: "Alice".to_owned(),
        action: "message".to_owned(),
        body: "hello".to_owned(),
    };

    let error = persist_delivery_event(&path, "comm_message_delivery", &envelope).unwrap_err();
    assert!(matches!(error, CommCenterError::Persistence { .. }));
    assert!(!error.reason().is_empty());
}

#[test]
fn send_message_persists_delivery_and_returns_receipt() {
    let path = temp_log_path("comm-send");
    let result = send_message(
        &path,
        CommRouteRequest::new("Kevin", "Alice", "message", "hello"),
    )
    .unwrap();

    assert_eq!(result.delivery_id, "delivery:Alice:message");
    assert_eq!(result.target, "Alice");
    assert_eq!(result.action, "message");
    assert_eq!(result.sequence, 1);
    assert_eq!(result.log_path, path.display().to_string());

    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 1);
    assert_eq!(replayed.events[0].feature_id, FEATURE_ID);
    assert_eq!(replayed.events[0].event_kind, "comm_message_delivery");
}

#[test]
fn send_broadcast_persists_delivery_and_returns_receipt() {
    let path = temp_log_path("comm-broadcast");
    let result = send_broadcast(
        &path,
        CommTeamBroadcastRequest::new(
            "Kevin",
            "default",
            "broadcast",
            "hello",
            vec!["Alice".to_owned(), "Bob".to_owned()],
        ),
    )
    .unwrap();

    assert_eq!(result.delivery_id, "delivery:default:broadcast");
    assert_eq!(result.team_id, "default");
    assert_eq!(result.recipient_count, 2);
    assert_eq!(result.sequence, 1);
    assert_eq!(result.log_path, path.display().to_string());

    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 1);
    assert_eq!(replayed.events[0].feature_id, FEATURE_ID);
    assert_eq!(replayed.events[0].event_kind, "comm_broadcast_delivery");
}

#[test]
fn send_ready_report_persists_delivery_and_returns_receipt() {
    let path = temp_log_path("comm-ready");
    let result = send_ready_report(
        &path,
        CommReadyReportRequest::new("Alice", "default", "Alice", "ready"),
    )
    .unwrap();

    assert_eq!(result.delivery_id, "delivery:Alice:ready.report");
    assert_eq!(result.team_id, "default");
    assert_eq!(result.agent_name, "Alice");
    assert_eq!(result.sequence, 1);
    assert_eq!(result.log_path, path.display().to_string());

    let replayed = replay_event_log(&path, 0).unwrap();
    assert_eq!(replayed.events.len(), 1);
    assert_eq!(replayed.events[0].feature_id, FEATURE_ID);
    assert_eq!(replayed.events[0].event_kind, "comm_ready_report_delivery");
}
