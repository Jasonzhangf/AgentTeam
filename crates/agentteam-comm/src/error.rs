#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommCenterError {
    Validation { reason: String },
}

pub type CommCenterResult<T> = Result<T, CommCenterError>;

impl CommCenterError {
    pub fn reason(&self) -> &str {
        match self {
            Self::Validation { reason } => reason,
        }
    }
}
