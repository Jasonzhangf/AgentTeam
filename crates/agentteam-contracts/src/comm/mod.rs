use serde::Serialize;

use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "comm.center";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq01RouteIntent {
    pub sender: String,
    pub target: String,
    pub action: String,
    pub body: String,
}

impl CommReq01RouteIntent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 1, "RouteIntent");

    pub fn new(
        sender: impl Into<String>,
        target: impl Into<String>,
        action: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            target: target.into(),
            action: action.into(),
            body: body.into(),
        }
    }

    pub fn resolve_target(self) -> CommReq02ResolvedTarget {
        CommReq02ResolvedTarget {
            sender: self.sender,
            target: self.target,
            action: self.action,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq02ResolvedTarget {
    pub sender: String,
    pub target: String,
    pub action: String,
    pub body: String,
}

impl CommReq02ResolvedTarget {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 2, "ResolvedTarget");

    pub fn delivery_envelope(self) -> CommReq03DeliveryEnvelope {
        CommReq03DeliveryEnvelope {
            sender: self.sender,
            target: self.target,
            action: self.action,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq03DeliveryEnvelope {
    pub sender: String,
    pub target: String,
    pub action: String,
    pub body: String,
}

impl CommReq03DeliveryEnvelope {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 3, "DeliveryEnvelope");

    pub fn accept(self, delivery_id: impl Into<String>) -> CommResp04DeliveryAccepted {
        CommResp04DeliveryAccepted {
            delivery_id: delivery_id.into(),
            target: self.target,
            action: self.action,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommResp04DeliveryAccepted {
    pub delivery_id: String,
    pub target: String,
    pub action: String,
}

impl CommResp04DeliveryAccepted {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Resp", 4, "DeliveryAccepted");
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq11BroadcastIntent {
    pub sender: String,
    pub team_id: String,
    pub action: String,
    pub body: String,
}

impl CommReq11BroadcastIntent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 11, "BroadcastIntent");

    pub fn new(
        sender: impl Into<String>,
        team_id: impl Into<String>,
        action: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            team_id: team_id.into(),
            action: action.into(),
            body: body.into(),
        }
    }

    pub fn resolve_team_members(self, members: Vec<String>) -> CommReq12ResolvedTeamMembers {
        CommReq12ResolvedTeamMembers {
            sender: self.sender,
            team_id: self.team_id,
            members,
            action: self.action,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq12ResolvedTeamMembers {
    pub sender: String,
    pub team_id: String,
    pub members: Vec<String>,
    pub action: String,
    pub body: String,
}

impl CommReq12ResolvedTeamMembers {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Comm", "Req", 12, "ResolvedTeamMembers");

    pub fn delivery_envelope(self) -> CommReq13BroadcastEnvelope {
        CommReq13BroadcastEnvelope {
            sender: self.sender,
            team_id: self.team_id,
            members: self.members,
            action: self.action,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq13BroadcastEnvelope {
    pub sender: String,
    pub team_id: String,
    pub members: Vec<String>,
    pub action: String,
    pub body: String,
}

impl CommReq13BroadcastEnvelope {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Comm", "Req", 13, "BroadcastEnvelope");

    pub fn accept(self, delivery_id: impl Into<String>) -> CommResp14BroadcastAccepted {
        CommResp14BroadcastAccepted {
            delivery_id: delivery_id.into(),
            team_id: self.team_id,
            recipient_count: self.members.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommResp14BroadcastAccepted {
    pub delivery_id: String,
    pub team_id: String,
    pub recipient_count: usize,
}

impl CommResp14BroadcastAccepted {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Comm", "Resp", 14, "BroadcastAccepted");
}

#[cfg(test)]
mod tests {
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
    fn comm_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "comm.center");
    }
}
