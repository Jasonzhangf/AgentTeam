#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistenceError {
    Validation {
        reason: String,
    },
    Io {
        path: String,
        reason: String,
    },
    Corruption {
        path: String,
        line: usize,
        reason: String,
    },
}

pub type PersistenceResult<T> = Result<T, PersistenceError>;

impl PersistenceError {
    pub fn reason(&self) -> &str {
        match self {
            Self::Validation { reason }
            | Self::Io { reason, .. }
            | Self::Corruption { reason, .. } => reason,
        }
    }
}
