use serde::Serialize;

use crate::pipeline::PipelineNodeName;

#[cfg(test)]
mod tests;

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
pub struct CommReq05ReadyReport {
    pub sender: String,
    pub team_id: String,
    pub agent_name: String,
    pub body: String,
}

impl CommReq05ReadyReport {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 5, "ReadyReport");

    pub fn new(
        sender: impl Into<String>,
        team_id: impl Into<String>,
        agent_name: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            team_id: team_id.into(),
            agent_name: agent_name.into(),
            body: body.into(),
        }
    }

    pub fn resolve_agent(self) -> CommReq06ResolvedAgent {
        CommReq06ResolvedAgent {
            sender: self.sender,
            team_id: self.team_id,
            agent_name: self.agent_name,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq06ResolvedAgent {
    pub sender: String,
    pub team_id: String,
    pub agent_name: String,
    pub body: String,
}

impl CommReq06ResolvedAgent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 6, "ResolvedAgent");

    pub fn delivery_envelope(self) -> CommReq07ReadyEnvelope {
        CommReq07ReadyEnvelope {
            sender: self.sender,
            team_id: self.team_id,
            agent_name: self.agent_name,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq07ReadyEnvelope {
    pub sender: String,
    pub team_id: String,
    pub agent_name: String,
    pub body: String,
}

impl CommReq07ReadyEnvelope {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 7, "ReadyEnvelope");

    pub fn accept(self, delivery_id: impl Into<String>) -> CommResp08ReadyAccepted {
        CommResp08ReadyAccepted {
            delivery_id: delivery_id.into(),
            team_id: self.team_id,
            agent_name: self.agent_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommResp08ReadyAccepted {
    pub delivery_id: String,
    pub team_id: String,
    pub agent_name: String,
}

impl CommResp08ReadyAccepted {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Resp", 8, "ReadyAccepted");
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq21TaskBoardQuery {
    pub sender: String,
    pub team_id: String,
    pub query: String,
    pub body: String,
}

impl CommReq21TaskBoardQuery {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 21, "TaskBoardQuery");

    pub fn new(
        sender: impl Into<String>,
        team_id: impl Into<String>,
        query: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            team_id: team_id.into(),
            query: query.into(),
            body: body.into(),
        }
    }

    pub fn resolve_team(self) -> CommReq22AuthorizedQuery {
        CommReq22AuthorizedQuery {
            sender: self.sender,
            team_id: self.team_id,
            query: self.query,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq22AuthorizedQuery {
    pub sender: String,
    pub team_id: String,
    pub query: String,
    pub body: String,
}

impl CommReq22AuthorizedQuery {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 22, "AuthorizedQuery");

    pub fn delivery_envelope(self) -> CommReq23TaskBoardQueryEnvelope {
        CommReq23TaskBoardQueryEnvelope {
            sender: self.sender,
            team_id: self.team_id,
            query: self.query,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq23TaskBoardQueryEnvelope {
    pub sender: String,
    pub team_id: String,
    pub query: String,
    pub body: String,
}

impl CommReq23TaskBoardQueryEnvelope {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Comm", "Req", 23, "TaskBoardQueryEnvelope");

    pub fn accept(self, delivery_id: impl Into<String>) -> CommResp24TaskBoardQueryAccepted {
        CommResp24TaskBoardQueryAccepted {
            delivery_id: delivery_id.into(),
            team_id: self.team_id,
            query: self.query,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommResp24TaskBoardQueryAccepted {
    pub delivery_id: String,
    pub team_id: String,
    pub query: String,
}

impl CommResp24TaskBoardQueryAccepted {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Comm", "Resp", 24, "TaskBoardQueryAccepted");
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq31TaskClaim {
    pub sender: String,
    pub team_id: String,
    pub worker_name: String,
    pub worker_role: String,
    pub body: String,
}

impl CommReq31TaskClaim {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 31, "TaskClaim");

    pub fn new(
        sender: impl Into<String>,
        team_id: impl Into<String>,
        worker_name: impl Into<String>,
        worker_role: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            team_id: team_id.into(),
            worker_name: worker_name.into(),
            worker_role: worker_role.into(),
            body: body.into(),
        }
    }

    pub fn resolve_claim(self) -> CommReq32AuthorizedClaim {
        CommReq32AuthorizedClaim {
            sender: self.sender,
            team_id: self.team_id,
            worker_name: self.worker_name,
            worker_role: self.worker_role,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq32AuthorizedClaim {
    pub sender: String,
    pub team_id: String,
    pub worker_name: String,
    pub worker_role: String,
    pub body: String,
}

impl CommReq32AuthorizedClaim {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Comm", "Req", 32, "AuthorizedClaim");

    pub fn delivery_envelope(self) -> CommReq33TaskClaimEnvelope {
        CommReq33TaskClaimEnvelope {
            sender: self.sender,
            team_id: self.team_id,
            worker_name: self.worker_name,
            worker_role: self.worker_role,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReq33TaskClaimEnvelope {
    pub sender: String,
    pub team_id: String,
    pub worker_name: String,
    pub worker_role: String,
    pub body: String,
}

impl CommReq33TaskClaimEnvelope {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Comm", "Req", 33, "TaskClaimEnvelope");

    pub fn accept(self, delivery_id: impl Into<String>) -> CommResp34TaskClaimAccepted {
        CommResp34TaskClaimAccepted {
            delivery_id: delivery_id.into(),
            team_id: self.team_id,
            worker_name: self.worker_name,
            worker_role: self.worker_role,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommResp34TaskClaimAccepted {
    pub delivery_id: String,
    pub team_id: String,
    pub worker_name: String,
    pub worker_role: String,
}

impl CommResp34TaskClaimAccepted {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Comm", "Resp", 34, "TaskClaimAccepted");
}
