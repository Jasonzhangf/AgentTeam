use agentteam_config::ConfigCenterError;
use agentteam_control::ControlError;
use agentteam_resource::ResourceError;
use agentteam_tmux::TmuxAdapterError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupError {
    Config { reason: String },
    Team { reason: String },
    Control { reason: String },
    Launch { reason: String },
    Tmux { reason: String },
    Resource { reason: String },
    Cleanup { primary: String, cleanup: String },
}

pub type StartupResult<T> = Result<T, StartupError>;

impl StartupError {
    pub fn reason(&self) -> String {
        match self {
            Self::Config { reason }
            | Self::Team { reason }
            | Self::Control { reason }
            | Self::Launch { reason }
            | Self::Tmux { reason }
            | Self::Resource { reason } => reason.clone(),
            Self::Cleanup { primary, cleanup } => {
                format!("primary failure: {primary}; cleanup failure: {cleanup}")
            }
        }
    }
}

pub(crate) fn config_error(error: ConfigCenterError) -> StartupError {
    StartupError::Config {
        reason: error.reason().to_owned(),
    }
}

pub(crate) fn resource_error(error: ResourceError) -> StartupError {
    StartupError::Resource {
        reason: error.reason().to_owned(),
    }
}

pub(crate) fn control_error(error: ControlError) -> StartupError {
    StartupError::Control {
        reason: error.reason(),
    }
}

pub(crate) fn tmux_error(error: TmuxAdapterError) -> StartupError {
    StartupError::Tmux {
        reason: error.reason(),
    }
}
