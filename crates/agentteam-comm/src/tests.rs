use crate::error::CommCenterError;
use crate::model::{
    CommReadyReportRequest, CommRouteRequest, CommTaskBoardQueryRequest, CommTaskClaimRequest,
    CommTeamBroadcastRequest,
};
use crate::route::{
    route_broadcast, route_message, route_ready_report, route_task_board_query, route_task_claim,
    CommCenter,
};

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
