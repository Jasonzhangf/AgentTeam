use serde::Serialize;

use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "gateway.input";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TeamReq01CliRaw {
    pub args: Vec<String>,
}

impl TeamReq01CliRaw {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Team", "Req", 1, "CliRaw");

    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum TeamReq02ParsedCommand {
    ConfigCheck {
        config_path: Option<String>,
        json: bool,
    },
    DaemonCheck {
        config_path: Option<String>,
        json: bool,
    },
    DomainResolve {
        target: Option<String>,
        config_path: Option<String>,
        json: bool,
    },
    DebugSnapshot {
        config_path: Option<String>,
        runtime_home: Option<String>,
        json: bool,
    },
    TaskSend {
        runtime_home: Option<String>,
        team_id: Option<String>,
        created_by: Option<String>,
        target_kind: Option<String>,
        target: Option<String>,
        title: Option<String>,
        body: Option<String>,
        json: bool,
    },
    TaskList {
        runtime_home: Option<String>,
        json: bool,
    },
    TaskStatus {
        runtime_home: Option<String>,
        task_id: Option<String>,
        json: bool,
    },
    TaskDone {
        runtime_home: Option<String>,
        task_id: Option<String>,
        actor: Option<String>,
        detail: Option<String>,
        json: bool,
    },
    TaskError {
        runtime_home: Option<String>,
        task_id: Option<String>,
        actor: Option<String>,
        detail: Option<String>,
        json: bool,
    },
    TaskClaim {
        runtime_home: Option<String>,
        worker_name: Option<String>,
        worker_role: Option<String>,
        json: bool,
    },
    TmuxLoopback {
        runtime_home: Option<String>,
        session_count: Option<String>,
        json: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum TeamReq03ValidatedIntent {
    ConfigCheck {
        config_path: String,
        json: bool,
    },
    DaemonCheck {
        config_path: String,
        json: bool,
    },
    DomainResolve {
        target: String,
        config_path: String,
        json: bool,
    },
    DebugSnapshot {
        config_path: String,
        runtime_home: String,
        json: bool,
    },
    TaskSend {
        runtime_home: String,
        team_id: String,
        created_by: String,
        target_kind: String,
        target: String,
        title: String,
        body: String,
        json: bool,
    },
    TaskList {
        runtime_home: String,
        json: bool,
    },
    TaskStatus {
        runtime_home: String,
        task_id: String,
        json: bool,
    },
    TaskDone {
        runtime_home: String,
        task_id: String,
        actor: String,
        detail: String,
        json: bool,
    },
    TaskError {
        runtime_home: String,
        task_id: String,
        actor: String,
        detail: String,
        json: bool,
    },
    TaskClaim {
        runtime_home: String,
        worker_name: String,
        worker_role: String,
        json: bool,
    },
    TmuxLoopback {
        runtime_home: String,
        session_count: String,
        json: bool,
    },
}

impl TeamReq03ValidatedIntent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Team", "Req", 3, "ValidatedIntent");

    pub fn command_name(&self) -> &'static str {
        match self {
            Self::ConfigCheck { .. } => "config.check",
            Self::DaemonCheck { .. } => "daemon.check",
            Self::DomainResolve { .. } => "domain.resolve",
            Self::DebugSnapshot { .. } => "debug.snapshot",
            Self::TaskSend { .. } => "task.send",
            Self::TaskList { .. } => "task.list",
            Self::TaskStatus { .. } => "task.status",
            Self::TaskDone { .. } => "task.done",
            Self::TaskError { .. } => "task.error",
            Self::TaskClaim { .. } => "task.claim",
            Self::TmuxLoopback { .. } => "tmux.loopback",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn team_request_command_name_is_stable() {
        let intent = TeamReq03ValidatedIntent::ConfigCheck {
            config_path: "docs/config/config.toml.example".to_owned(),
            json: true,
        };

        assert_eq!(intent.command_name(), "config.check");
        let intent = TeamReq03ValidatedIntent::DaemonCheck {
            config_path: "docs/config/config.toml.example".to_owned(),
            json: true,
        };
        assert_eq!(intent.command_name(), "daemon.check");
        let intent = TeamReq03ValidatedIntent::TaskList {
            runtime_home: "target/agentteam-smoke".to_owned(),
            json: true,
        };
        assert_eq!(intent.command_name(), "task.list");
        let intent = TeamReq03ValidatedIntent::TaskClaim {
            runtime_home: "target/agentteam-smoke".to_owned(),
            worker_name: "Alice".to_owned(),
            worker_role: "builder".to_owned(),
            json: true,
        };
        assert_eq!(intent.command_name(), "task.claim");
        let intent = TeamReq03ValidatedIntent::TmuxLoopback {
            runtime_home: "target/agentteam-smoke".to_owned(),
            session_count: "2".to_owned(),
            json: true,
        };
        assert_eq!(intent.command_name(), "tmux.loopback");
        assert_eq!(TeamReq01CliRaw::NODE.number, 1);
        assert_eq!(TeamReq03ValidatedIntent::NODE.number, 3);
    }

    #[test]
    fn team_feature_id_is_gateway_input() {
        assert_eq!(FEATURE_ID, "gateway.input");
    }
}
