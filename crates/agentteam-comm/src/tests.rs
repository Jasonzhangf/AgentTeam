use crate::error::CommCenterError;
use crate::model::{CommRouteRequest, CommTeamBroadcastRequest};
use crate::route::{route_broadcast, route_message, CommCenter};

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
fn route_message_rejects_empty_target() {
    let error = CommCenter::new()
        .route_message(CommRouteRequest::new("Kevin", "", "message", "hello"))
        .unwrap_err();
    assert!(matches!(error, CommCenterError::Validation { .. }));
}
