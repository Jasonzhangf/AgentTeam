use agentteam_persist::PersistenceError;
use agentteam_resource::ResourceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebugError {
    Validation { reason: String },
    Persistence { reason: String },
    Resource { reason: String },
}

pub type DebugResult<T> = Result<T, DebugError>;

impl DebugError {
    pub fn reason(&self) -> &str {
        match self {
            Self::Validation { reason }
            | Self::Persistence { reason }
            | Self::Resource { reason } => reason,
        }
    }
}

pub fn persistence_error(error: PersistenceError) -> DebugError {
    DebugError::Persistence {
        reason: error.reason().to_owned(),
    }
}

pub fn resource_error(error: ResourceError) -> DebugError {
    DebugError::Resource {
        reason: error.reason().to_owned(),
    }
}
