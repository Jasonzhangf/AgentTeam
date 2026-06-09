use serde::Serialize;

use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "agent.control_center";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentControlMode {
    AttachTui,
    Headless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentControlAction {
    Attach,
    Send,
    Observe,
    Pause,
    Stop,
    Wait,
    Retry,
    Status,
    Headless,
    HeadlessRun,
    HeadlessStatus,
    HeadlessInterrupt,
    HeadlessStop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCtlReq01ModeIntent {
    pub agent_name: String,
    pub team_id: String,
    pub mode: AgentControlMode,
    pub session_name: String,
}

impl AgentCtlReq01ModeIntent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("AgentCtl", "Req", 1, "ModeIntent");

    pub fn new(
        agent_name: impl Into<String>,
        team_id: impl Into<String>,
        mode: AgentControlMode,
        session_name: impl Into<String>,
    ) -> Self {
        Self {
            agent_name: agent_name.into(),
            team_id: team_id.into(),
            mode,
            session_name: session_name.into(),
        }
    }

    pub fn resolve_mode(self) -> AgentCtlReq02ResolvedMode {
        let adapter_kind = match self.mode {
            AgentControlMode::AttachTui => "tmux",
            AgentControlMode::Headless => "sdk",
        };
        AgentCtlReq02ResolvedMode {
            agent_name: self.agent_name,
            team_id: self.team_id,
            mode: self.mode,
            session_name: self.session_name,
            adapter_kind: adapter_kind.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCtlReq02ResolvedMode {
    pub agent_name: String,
    pub team_id: String,
    pub mode: AgentControlMode,
    pub session_name: String,
    pub adapter_kind: String,
}

impl AgentCtlReq02ResolvedMode {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("AgentCtl", "Req", 2, "ResolvedMode");

    pub fn bind_session(self) -> AgentCtlReq03SessionBinding {
        AgentCtlReq03SessionBinding {
            agent_name: self.agent_name,
            team_id: self.team_id,
            mode: self.mode,
            session_name: self.session_name,
            adapter_kind: self.adapter_kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCtlReq03SessionBinding {
    pub agent_name: String,
    pub team_id: String,
    pub mode: AgentControlMode,
    pub session_name: String,
    pub adapter_kind: String,
}

impl AgentCtlReq03SessionBinding {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("AgentCtl", "Req", 3, "SessionBinding");

    pub fn apply_action(
        self,
        action: AgentControlAction,
        state: impl Into<String>,
        details: impl Into<String>,
    ) -> AgentCtlReq04ControlAction {
        AgentCtlReq04ControlAction {
            agent_name: self.agent_name,
            team_id: self.team_id,
            mode: self.mode,
            session_name: self.session_name,
            adapter_kind: self.adapter_kind,
            action,
            state: state.into(),
            details: details.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCtlReq04ControlAction {
    pub agent_name: String,
    pub team_id: String,
    pub mode: AgentControlMode,
    pub session_name: String,
    pub adapter_kind: String,
    pub action: AgentControlAction,
    pub state: String,
    pub details: String,
}

impl AgentCtlReq04ControlAction {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("AgentCtl", "Req", 4, "ControlAction");

    pub fn project(self, receipt_id: impl Into<String>) -> AgentCtlResp05ControlProjection {
        AgentCtlResp05ControlProjection {
            agent_name: self.agent_name,
            team_id: self.team_id,
            mode: self.mode,
            session_name: self.session_name,
            adapter_kind: self.adapter_kind,
            action: self.action,
            state: self.state,
            details: self.details,
            receipt_id: receipt_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCtlResp05ControlProjection {
    pub agent_name: String,
    pub team_id: String,
    pub mode: AgentControlMode,
    pub session_name: String,
    pub adapter_kind: String,
    pub action: AgentControlAction,
    pub state: String,
    pub details: String,
    pub receipt_id: String,
}

impl AgentCtlResp05ControlProjection {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("AgentCtl", "Resp", 5, "ControlProjection");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_chain_keeps_mode_and_receipt() {
        let projection = AgentCtlReq01ModeIntent::new(
            "Kevin",
            "default",
            AgentControlMode::AttachTui,
            "TA_local_agentteam_Kevin",
        )
        .resolve_mode()
        .bind_session()
        .apply_action(AgentControlAction::Attach, "idle", "bound")
        .project("receipt-1");

        assert_eq!(projection.adapter_kind, "tmux");
        assert_eq!(projection.state, "idle");
        assert_eq!(projection.receipt_id, "receipt-1");
        assert_eq!(AgentCtlReq01ModeIntent::NODE.number, 1);
        assert_eq!(AgentCtlResp05ControlProjection::NODE.number, 5);
    }

    #[test]
    fn control_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "agent.control_center");
    }
}
