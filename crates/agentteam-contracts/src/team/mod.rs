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
}

impl TeamReq03ValidatedIntent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Team", "Req", 3, "ValidatedIntent");

    pub fn command_name(&self) -> &'static str {
        match self {
            Self::ConfigCheck { .. } => "config.check",
            Self::DaemonCheck { .. } => "daemon.check",
            Self::DomainResolve { .. } => "domain.resolve",
            Self::DebugSnapshot { .. } => "debug.snapshot",
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
        assert_eq!(TeamReq01CliRaw::NODE.number, 1);
        assert_eq!(TeamReq03ValidatedIntent::NODE.number, 3);
    }

    #[test]
    fn team_feature_id_is_gateway_input() {
        assert_eq!(FEATURE_ID, "gateway.input");
    }
}
