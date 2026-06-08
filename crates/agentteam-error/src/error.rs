use agentteam_persist::PersistenceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCenterError {
    Validation { reason: String },
    Persistence { reason: String },
}

pub type ErrorCenterResult<T> = Result<T, ErrorCenterError>;

impl ErrorCenterError {
    pub fn reason(&self) -> &str {
        match self {
            Self::Validation { reason } | Self::Persistence { reason } => reason,
        }
    }
}

pub fn persistence_error(error: PersistenceError) -> ErrorCenterError {
    ErrorCenterError::Persistence {
        reason: error.reason().to_owned(),
    }
}
