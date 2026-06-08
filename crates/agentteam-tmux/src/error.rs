#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TmuxAdapterError {
    Validation { reason: String },
    ProcessLaunch { reason: String },
    CommandFailed { operation: String, reason: String },
    Observation { logical_id: String, reason: String },
    Cleanup { logical_id: String, reason: String },
    CleanupAfterFailure { primary: String, cleanup: String },
}

pub type TmuxAdapterResult<T> = Result<T, TmuxAdapterError>;

impl TmuxAdapterError {
    pub fn reason(&self) -> String {
        match self {
            Self::Validation { reason }
            | Self::ProcessLaunch { reason }
            | Self::CommandFailed { reason, .. }
            | Self::Observation { reason, .. }
            | Self::Cleanup { reason, .. } => reason.clone(),
            Self::CleanupAfterFailure { primary, cleanup } => {
                format!("primary failure: {primary}; cleanup failure: {cleanup}")
            }
        }
    }
}
