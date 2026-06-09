use agentteam_tmux::TmuxAdapterError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlError {
    Validation { reason: String },
    Tmux { reason: String },
    HeadlessUnavailable { reason: String },
    HeadlessBridge { reason: String },
    Unsupported { reason: String },
}

pub type ControlResult<T> = Result<T, ControlError>;

impl ControlError {
    pub fn reason(&self) -> String {
        match self {
            Self::Validation { reason }
            | Self::Tmux { reason }
            | Self::HeadlessUnavailable { reason }
            | Self::HeadlessBridge { reason }
            | Self::Unsupported { reason } => reason.clone(),
        }
    }
}

pub fn tmux_error(error: TmuxAdapterError) -> ControlError {
    ControlError::Tmux {
        reason: error.reason(),
    }
}
