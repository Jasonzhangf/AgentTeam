use super::*;

#[test]
fn comm_message_chain_uses_adjacent_nodes() {
    let accepted = CommReq01RouteIntent::new("Kevin", "Alice", "message", "hello")
        .resolve_target()
        .delivery_envelope()
        .accept("delivery-1");

    assert_eq!(accepted.delivery_id, "delivery-1");
    assert_eq!(accepted.target, "Alice");
    assert_eq!(CommReq01RouteIntent::NODE.number, 1);
    assert_eq!(CommResp04DeliveryAccepted::NODE.number, 4);
}

#[test]
fn comm_ready_chain_uses_adjacent_nodes() {
    let accepted = CommReq05ReadyReport::new("Alice", "default", "Alice", "ready")
        .resolve_agent()
        .delivery_envelope()
        .accept("delivery-5");

    assert_eq!(accepted.team_id, "default");
    assert_eq!(accepted.agent_name, "Alice");
    assert_eq!(CommReq05ReadyReport::NODE.number, 5);
    assert_eq!(CommResp08ReadyAccepted::NODE.number, 8);
}

#[test]
fn comm_broadcast_chain_uses_adjacent_nodes() {
    let accepted = CommReq11BroadcastIntent::new("Kevin", "default", "broadcast", "hello")
        .resolve_team_members(vec!["Alice".to_owned(), "Bob".to_owned()])
        .delivery_envelope()
        .accept("delivery-2");

    assert_eq!(accepted.team_id, "default");
    assert_eq!(accepted.recipient_count, 2);
    assert_eq!(CommReq11BroadcastIntent::NODE.number, 11);
    assert_eq!(CommResp14BroadcastAccepted::NODE.number, 14);
}

#[test]
fn comm_task_board_chain_uses_adjacent_nodes() {
    let accepted = CommReq21TaskBoardQuery::new("Kevin", "default", "board", "show board")
        .resolve_team()
        .delivery_envelope()
        .accept("delivery-3");

    assert_eq!(accepted.team_id, "default");
    assert_eq!(accepted.query, "board");
    assert_eq!(CommReq21TaskBoardQuery::NODE.number, 21);
    assert_eq!(CommResp24TaskBoardQueryAccepted::NODE.number, 24);
}

#[test]
fn comm_task_claim_chain_uses_adjacent_nodes() {
    let accepted = CommReq31TaskClaim::new("Alice", "default", "Alice", "builder", "claim")
        .resolve_claim()
        .delivery_envelope()
        .accept("delivery-4");

    assert_eq!(accepted.team_id, "default");
    assert_eq!(accepted.worker_name, "Alice");
    assert_eq!(CommReq31TaskClaim::NODE.number, 31);
    assert_eq!(CommResp34TaskClaimAccepted::NODE.number, 34);
}

#[test]
fn comm_feature_id_is_stable() {
    assert_eq!(FEATURE_ID, "comm.center");
}
