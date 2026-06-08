use agentteam_persist::PersistenceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceError {
    Validation {
        reason: String,
    },
    NotFound {
        lease_id: String,
    },
    NotOwner {
        lease_id: String,
        owner_module: String,
    },
    Persistence {
        reason: String,
    },
}

pub type ResourceResult<T> = Result<T, ResourceError>;

impl ResourceError {
    pub fn reason(&self) -> &str {
        match self {
            Self::Validation { reason } | Self::Persistence { reason } => reason,
            Self::NotFound { .. } => "lease not found",
            Self::NotOwner { .. } => "lease owner mismatch",
        }
    }
}

pub fn persistence_error(error: PersistenceError) -> ResourceError {
    ResourceError::Persistence {
        reason: error.reason().to_owned(),
    }
}
