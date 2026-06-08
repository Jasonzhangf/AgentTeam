use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TeamReq01CliRaw {
    pub args: Vec<String>,
}

impl TeamReq01CliRaw {
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
    pub fn command_name(&self) -> &'static str {
        match self {
            Self::ConfigCheck { .. } => "config.check",
            Self::DomainResolve { .. } => "domain.resolve",
            Self::DebugSnapshot { .. } => "debug.snapshot",
        }
    }
}
