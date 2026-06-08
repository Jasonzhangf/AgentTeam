#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigCenterError {
    Load { path: String, reason: String },
    Parse { path: String, reason: String },
    Validation { path: String, reason: String },
}

pub type ConfigCenterResult<T> = Result<T, ConfigCenterError>;

impl ConfigCenterError {
    pub fn path(&self) -> &str {
        match self {
            Self::Load { path, .. } | Self::Parse { path, .. } | Self::Validation { path, .. } => {
                path
            }
        }
    }

    pub fn reason(&self) -> &str {
        match self {
            Self::Load { reason, .. }
            | Self::Parse { reason, .. }
            | Self::Validation { reason, .. } => reason,
        }
    }
}
