use crate::error::{CommCenterError, CommCenterResult};
use crate::model::{
    CommBroadcastResult, CommBroadcastTarget, CommMessageResult, CommRouteRequest, CommRouteTarget,
    CommTeamBroadcastRequest,
};

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
}

pub fn route_message(request: CommRouteRequest) -> CommCenterResult<CommMessageResult> {
    CommCenter::new().route_message(request)
}

pub fn route_broadcast(request: CommTeamBroadcastRequest) -> CommCenterResult<CommBroadcastResult> {
    CommCenter::new().route_broadcast(request)
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

fn delivery_id_for(target: &str, action: &str) -> String {
    format!("delivery:{target}:{action}")
}
