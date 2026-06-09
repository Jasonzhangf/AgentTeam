use agentteam_persist::PersistenceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommCenterError {
    Validation { reason: String },
    Persistence { reason: String },
}

pub type CommCenterResult<T> = Result<T, CommCenterError>;

impl CommCenterError {
    pub fn reason(&self) -> &str {
        match self {
            Self::Validation { reason } | Self::Persistence { reason } => reason,
        }
    }
}

pub fn persistence_error(error: PersistenceError) -> CommCenterError {
    CommCenterError::Persistence {
        reason: error.reason().to_owned(),
    }
}
