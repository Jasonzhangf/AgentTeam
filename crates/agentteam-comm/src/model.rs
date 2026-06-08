use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommRouteRequest {
    pub sender: String,
    pub target: String,
    pub action: String,
    pub body: String,
}

impl CommRouteRequest {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommRouteTarget {
    pub sender: String,
    pub target: String,
    pub action: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReadyReportRequest {
    pub sender: String,
    pub team_id: String,
    pub agent_name: String,
    pub body: String,
}

impl CommReadyReportRequest {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReadyReportTarget {
    pub sender: String,
    pub team_id: String,
    pub agent_name: String,
    pub body: String,
}

impl CommReadyReportRequest {
    pub fn resolve_agent(self) -> CommReadyReportTarget {
        CommReadyReportTarget {
            sender: self.sender,
            team_id: self.team_id,
            agent_name: self.agent_name,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReadyReportEnvelope {
    pub sender: String,
    pub team_id: String,
    pub agent_name: String,
    pub body: String,
}

impl CommReadyReportTarget {
    pub fn delivery_envelope(self) -> CommReadyReportEnvelope {
        CommReadyReportEnvelope {
            sender: self.sender,
            team_id: self.team_id,
            agent_name: self.agent_name,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommReadyReportResult {
    pub delivery_id: String,
    pub team_id: String,
    pub agent_name: String,
}

impl CommReadyReportEnvelope {
    pub fn accept(self, delivery_id: impl Into<String>) -> CommReadyReportResult {
        CommReadyReportResult {
            delivery_id: delivery_id.into(),
            team_id: self.team_id,
            agent_name: self.agent_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommBroadcastTarget {
    pub sender: String,
    pub team_id: String,
    pub members: Vec<String>,
    pub action: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommMessageResult {
    pub delivery_id: String,
    pub target: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTeamBroadcastRequest {
    pub sender: String,
    pub team_id: String,
    pub action: String,
    pub body: String,
    pub members: Vec<String>,
}

impl CommTeamBroadcastRequest {
    pub fn new(
        sender: impl Into<String>,
        team_id: impl Into<String>,
        action: impl Into<String>,
        body: impl Into<String>,
        members: Vec<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            team_id: team_id.into(),
            action: action.into(),
            body: body.into(),
            members,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommBroadcastResult {
    pub delivery_id: String,
    pub team_id: String,
    pub recipient_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTaskBoardQueryRequest {
    pub sender: String,
    pub team_id: String,
    pub query: String,
    pub body: String,
}

impl CommTaskBoardQueryRequest {
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

    pub fn resolve_team(self) -> CommTaskBoardQueryTarget {
        CommTaskBoardQueryTarget {
            sender: self.sender,
            team_id: self.team_id,
            query: self.query,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTaskBoardQueryTarget {
    pub sender: String,
    pub team_id: String,
    pub query: String,
    pub body: String,
}

impl CommTaskBoardQueryTarget {
    pub fn delivery_envelope(self) -> CommTaskBoardQueryEnvelope {
        CommTaskBoardQueryEnvelope {
            sender: self.sender,
            team_id: self.team_id,
            query: self.query,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTaskBoardQueryEnvelope {
    pub sender: String,
    pub team_id: String,
    pub query: String,
    pub body: String,
}

impl CommTaskBoardQueryEnvelope {
    pub fn accept(self, delivery_id: impl Into<String>) -> CommTaskBoardQueryResult {
        CommTaskBoardQueryResult {
            delivery_id: delivery_id.into(),
            team_id: self.team_id,
            query: self.query,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTaskBoardQueryResult {
    pub delivery_id: String,
    pub team_id: String,
    pub query: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTaskClaimRequest {
    pub sender: String,
    pub team_id: String,
    pub worker_name: String,
    pub worker_role: String,
    pub body: String,
}

impl CommTaskClaimRequest {
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

    pub fn resolve_claim(self) -> CommTaskClaimTarget {
        CommTaskClaimTarget {
            sender: self.sender,
            team_id: self.team_id,
            worker_name: self.worker_name,
            worker_role: self.worker_role,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTaskClaimTarget {
    pub sender: String,
    pub team_id: String,
    pub worker_name: String,
    pub worker_role: String,
    pub body: String,
}

impl CommTaskClaimTarget {
    pub fn delivery_envelope(self) -> CommTaskClaimEnvelope {
        CommTaskClaimEnvelope {
            sender: self.sender,
            team_id: self.team_id,
            worker_name: self.worker_name,
            worker_role: self.worker_role,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTaskClaimEnvelope {
    pub sender: String,
    pub team_id: String,
    pub worker_name: String,
    pub worker_role: String,
    pub body: String,
}

impl CommTaskClaimEnvelope {
    pub fn accept(self, delivery_id: impl Into<String>) -> CommTaskClaimResult {
        CommTaskClaimResult {
            delivery_id: delivery_id.into(),
            team_id: self.team_id,
            worker_name: self.worker_name,
            worker_role: self.worker_role,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommTaskClaimResult {
    pub delivery_id: String,
    pub team_id: String,
    pub worker_name: String,
    pub worker_role: String,
}
