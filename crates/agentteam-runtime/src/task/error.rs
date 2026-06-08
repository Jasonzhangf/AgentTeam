use agentteam_persist::PersistenceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskEngineError {
    Validation { reason: String },
    NotFound { task_id: String },
    InvalidTransition { task_id: String, reason: String },
    Persistence { reason: String },
}

pub type TaskEngineResult<T> = Result<T, TaskEngineError>;

impl TaskEngineError {
    pub fn reason(&self) -> String {
        match self {
            Self::Validation { reason }
            | Self::InvalidTransition { reason, .. }
            | Self::Persistence { reason } => reason.clone(),
            Self::NotFound { task_id } => format!("task {task_id} was not found"),
        }
    }
}

pub fn persistence_error(error: PersistenceError) -> TaskEngineError {
    TaskEngineError::Persistence {
        reason: error.reason().to_owned(),
    }
}
