use serde::Serialize;

use agentteam_contracts::control::{
    AgentControlAction, AgentControlMode, AgentCtlResp05ControlProjection,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlSessionInput {
    pub agent_name: String,
    pub team_id: String,
    pub session_name: String,
    pub cwd: Option<String>,
    pub project_slug: Option<String>,
}

impl ControlSessionInput {
    pub fn new(
        agent_name: impl Into<String>,
        team_id: impl Into<String>,
        session_name: impl Into<String>,
    ) -> Self {
        Self {
            agent_name: agent_name.into(),
            team_id: team_id.into(),
            session_name: session_name.into(),
            cwd: None,
            project_slug: None,
        }
    }

    pub fn with_scope(mut self, cwd: impl Into<String>, project_slug: impl Into<String>) -> Self {
        self.cwd = Some(cwd.into());
        self.project_slug = Some(project_slug.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlSendInput {
    pub session: ControlSessionInput,
    pub input: String,
}

impl ControlSendInput {
    pub fn new(session: ControlSessionInput, input: impl Into<String>) -> Self {
        Self {
            session,
            input: input.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlRetryInput {
    pub session: ControlSessionInput,
    pub task_id: String,
    pub error_fact_id: String,
}

impl ControlRetryInput {
    pub fn new(
        session: ControlSessionInput,
        task_id: impl Into<String>,
        error_fact_id: impl Into<String>,
    ) -> Self {
        Self {
            session,
            task_id: task_id.into(),
            error_fact_id: error_fact_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlSnapshot {
    pub agent_name: String,
    pub team_id: String,
    pub session_name: String,
    pub mode: AgentControlMode,
    pub action: AgentControlAction,
    pub state: String,
    pub details: String,
    pub adapter_kind: String,
    pub receipt_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlAgentSessionBinding {
    pub agent_name: String,
    pub team_id: String,
    pub session_name: String,
    pub project_slug: String,
    pub agent_session_id: String,
    pub seed_turn_id: Option<String>,
    pub state: String,
    pub details: String,
}

impl ControlAgentSessionBinding {
    pub fn new(
        session: &ControlSessionInput,
        project_slug: impl Into<String>,
        agent_session_id: impl Into<String>,
        seed_turn_id: Option<String>,
        state: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            agent_name: session.agent_name.clone(),
            team_id: session.team_id.clone(),
            session_name: session.session_name.clone(),
            project_slug: project_slug.into(),
            agent_session_id: agent_session_id.into(),
            seed_turn_id,
            state: state.into(),
            details: details.into(),
        }
    }
}

impl ControlSnapshot {
    pub fn from_projection(projection: &AgentCtlResp05ControlProjection) -> Self {
        Self {
            agent_name: projection.agent_name.clone(),
            team_id: projection.team_id.clone(),
            session_name: projection.session_name.clone(),
            mode: projection.mode,
            action: projection.action,
            state: projection.state.clone(),
            details: projection.details.clone(),
            adapter_kind: projection.adapter_kind.clone(),
            receipt_id: projection.receipt_id.clone(),
        }
    }
}
