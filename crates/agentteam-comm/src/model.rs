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
