use std::path::Path;

use agentteam_contracts::comm::CommReq03DeliveryEnvelope;
use agentteam_contracts::persist::PersistResp03AppendReceipt;

use crate::error::{CommCenterError, CommCenterResult};
use crate::model::{
    CommBroadcastResult, CommBroadcastSendResult, CommBroadcastTarget, CommMessageResult,
    CommMessageSendResult, CommReadyReportRequest, CommReadyReportResult,
    CommReadyReportSendResult, CommReadyReportTarget, CommRouteRequest, CommRouteTarget,
    CommTaskBoardQueryRequest, CommTaskBoardQueryResult, CommTaskBoardQueryTarget,
    CommTaskClaimRequest, CommTaskClaimResult, CommTaskClaimTarget, CommTeamBroadcastRequest,
};
use crate::persist::persist_delivery_event;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CommCenter;

impl CommCenter {
    pub fn new() -> Self {
        Self
    }

    pub fn route_message(&self, request: CommRouteRequest) -> CommCenterResult<CommMessageResult> {
        let target = validate_message_target(request)?;
        Ok(CommMessageResult {
            delivery_id: delivery_id_for(&target.target, &target.action),
            target: target.target,
            action: target.action,
        })
    }

    pub fn send_message(
        &self,
        log_path: impl AsRef<Path>,
        request: CommRouteRequest,
    ) -> CommCenterResult<CommMessageSendResult> {
        let target = validate_message_target(request)?;
        let delivery_envelope = target.delivery_envelope();
        let delivery_id = delivery_id_for(&delivery_envelope.target, &delivery_envelope.action);
        let receipt =
            persist_delivery_event(log_path, "comm_message_delivery", &delivery_envelope)?;
        Ok(message_send_result(delivery_id, delivery_envelope, receipt))
    }

    pub fn route_broadcast(
        &self,
        request: CommTeamBroadcastRequest,
    ) -> CommCenterResult<CommBroadcastResult> {
        let resolved = validate_broadcast_members(request)?;
        Ok(CommBroadcastResult {
            delivery_id: delivery_id_for(&resolved.team_id, &resolved.action),
            team_id: resolved.team_id,
            recipient_count: resolved.members.len(),
        })
    }

    pub fn send_broadcast(
        &self,
        log_path: impl AsRef<Path>,
        request: CommTeamBroadcastRequest,
    ) -> CommCenterResult<CommBroadcastSendResult> {
        let resolved = validate_broadcast_members(request)?;
        let delivery_id = delivery_id_for(&resolved.team_id, &resolved.action);
        let receipt = persist_delivery_event(log_path, "comm_broadcast_delivery", &resolved)?;
        Ok(broadcast_send_result(delivery_id, resolved, receipt))
    }

    pub fn route_ready_report(
        &self,
        request: CommReadyReportRequest,
    ) -> CommCenterResult<CommReadyReportResult> {
        let target = validate_ready_report(request)?;
        Ok(CommReadyReportResult {
            delivery_id: delivery_id_for(&target.agent_name, "ready.report"),
            team_id: target.team_id,
            agent_name: target.agent_name,
        })
    }

    pub fn send_ready_report(
        &self,
        log_path: impl AsRef<Path>,
        request: CommReadyReportRequest,
    ) -> CommCenterResult<CommReadyReportSendResult> {
        let target = validate_ready_report(request)?;
        let delivery_envelope = target.delivery_envelope();
        let delivery_id = delivery_id_for(&delivery_envelope.agent_name, "ready.report");
        let receipt =
            persist_delivery_event(log_path, "comm_ready_report_delivery", &delivery_envelope)?;
        Ok(ready_report_send_result(
            delivery_id,
            delivery_envelope,
            receipt,
        ))
    }

    pub fn route_task_board_query(
        &self,
        request: CommTaskBoardQueryRequest,
    ) -> CommCenterResult<CommTaskBoardQueryResult> {
        let target = validate_task_board_query(request)?;
        Ok(CommTaskBoardQueryResult {
            delivery_id: delivery_id_for(&target.team_id, "task.board.query"),
            team_id: target.team_id,
            query: target.query,
        })
    }

    pub fn route_task_claim(
        &self,
        request: CommTaskClaimRequest,
    ) -> CommCenterResult<CommTaskClaimResult> {
        let target = validate_task_claim(request)?;
        Ok(CommTaskClaimResult {
            delivery_id: delivery_id_for(&target.worker_name, "task.claim"),
            team_id: target.team_id,
            worker_name: target.worker_name,
            worker_role: target.worker_role,
        })
    }
}

pub fn route_message(request: CommRouteRequest) -> CommCenterResult<CommMessageResult> {
    CommCenter::new().route_message(request)
}

pub fn send_message(
    log_path: impl AsRef<Path>,
    request: CommRouteRequest,
) -> CommCenterResult<CommMessageSendResult> {
    CommCenter::new().send_message(log_path, request)
}

pub fn route_broadcast(request: CommTeamBroadcastRequest) -> CommCenterResult<CommBroadcastResult> {
    CommCenter::new().route_broadcast(request)
}

pub fn send_broadcast(
    log_path: impl AsRef<Path>,
    request: CommTeamBroadcastRequest,
) -> CommCenterResult<CommBroadcastSendResult> {
    CommCenter::new().send_broadcast(log_path, request)
}

pub fn route_ready_report(
    request: CommReadyReportRequest,
) -> CommCenterResult<CommReadyReportResult> {
    CommCenter::new().route_ready_report(request)
}

pub fn send_ready_report(
    log_path: impl AsRef<Path>,
    request: CommReadyReportRequest,
) -> CommCenterResult<CommReadyReportSendResult> {
    CommCenter::new().send_ready_report(log_path, request)
}

pub fn route_task_board_query(
    request: CommTaskBoardQueryRequest,
) -> CommCenterResult<CommTaskBoardQueryResult> {
    CommCenter::new().route_task_board_query(request)
}

pub fn route_task_claim(request: CommTaskClaimRequest) -> CommCenterResult<CommTaskClaimResult> {
    CommCenter::new().route_task_claim(request)
}

fn validate_message_target(request: CommRouteRequest) -> CommCenterResult<CommRouteTarget> {
    if request.sender.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "sender is required".to_owned(),
        });
    }
    if request.target.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "target is required".to_owned(),
        });
    }
    if request.action.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "action is required".to_owned(),
        });
    }
    Ok(CommRouteTarget {
        sender: request.sender,
        target: request.target,
        action: request.action,
        body: request.body,
    })
}

fn validate_broadcast_members(
    request: CommTeamBroadcastRequest,
) -> CommCenterResult<CommBroadcastTarget> {
    if request.sender.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "sender is required".to_owned(),
        });
    }
    if request.team_id.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "team_id is required".to_owned(),
        });
    }
    if request.members.is_empty() {
        return Err(CommCenterError::Validation {
            reason: "broadcast requires at least one member".to_owned(),
        });
    }
    Ok(CommBroadcastTarget {
        sender: request.sender,
        team_id: request.team_id,
        members: request.members,
        action: request.action,
        body: request.body,
    })
}

fn validate_ready_report(
    request: CommReadyReportRequest,
) -> CommCenterResult<CommReadyReportTarget> {
    if request.sender.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "sender is required".to_owned(),
        });
    }
    if request.team_id.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "team_id is required".to_owned(),
        });
    }
    if request.agent_name.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "agent_name is required".to_owned(),
        });
    }
    Ok(CommReadyReportTarget {
        sender: request.sender,
        team_id: request.team_id,
        agent_name: request.agent_name,
        body: request.body,
    })
}

fn validate_task_board_query(
    request: CommTaskBoardQueryRequest,
) -> CommCenterResult<CommTaskBoardQueryTarget> {
    if request.sender.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "sender is required".to_owned(),
        });
    }
    if request.team_id.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "team_id is required".to_owned(),
        });
    }
    if request.query.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "query is required".to_owned(),
        });
    }
    Ok(CommTaskBoardQueryTarget {
        sender: request.sender,
        team_id: request.team_id,
        query: request.query,
        body: request.body,
    })
}

fn validate_task_claim(request: CommTaskClaimRequest) -> CommCenterResult<CommTaskClaimTarget> {
    if request.sender.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "sender is required".to_owned(),
        });
    }
    if request.team_id.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "team_id is required".to_owned(),
        });
    }
    if request.worker_name.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "worker_name is required".to_owned(),
        });
    }
    if request.worker_role.trim().is_empty() {
        return Err(CommCenterError::Validation {
            reason: "worker_role is required".to_owned(),
        });
    }
    Ok(CommTaskClaimTarget {
        sender: request.sender,
        team_id: request.team_id,
        worker_name: request.worker_name,
        worker_role: request.worker_role,
        body: request.body,
    })
}

fn delivery_id_for(target: &str, action: &str) -> String {
    format!("delivery:{target}:{action}")
}

fn ready_report_send_result(
    delivery_id: String,
    envelope: crate::model::CommReadyReportEnvelope,
    receipt: PersistResp03AppendReceipt,
) -> CommReadyReportSendResult {
    CommReadyReportSendResult {
        delivery_id,
        team_id: envelope.team_id,
        agent_name: envelope.agent_name,
        event_id: receipt.event_id,
        sequence: receipt.sequence,
        log_path: receipt.log_path,
    }
}

fn broadcast_send_result(
    delivery_id: String,
    envelope: CommBroadcastTarget,
    receipt: PersistResp03AppendReceipt,
) -> CommBroadcastSendResult {
    CommBroadcastSendResult {
        delivery_id,
        team_id: envelope.team_id,
        recipient_count: envelope.members.len(),
        event_id: receipt.event_id,
        sequence: receipt.sequence,
        log_path: receipt.log_path,
    }
}

fn message_send_result(
    delivery_id: String,
    envelope: CommReq03DeliveryEnvelope,
    receipt: PersistResp03AppendReceipt,
) -> CommMessageSendResult {
    CommMessageSendResult {
        delivery_id,
        target: envelope.target,
        action: envelope.action,
        event_id: receipt.event_id,
        sequence: receipt.sequence,
        log_path: receipt.log_path,
    }
}
