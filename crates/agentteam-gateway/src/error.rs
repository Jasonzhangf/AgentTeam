use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GatewayError {
    pub feature_id: &'static str,
    pub class: &'static str,
    pub reason: String,
}

pub type GatewayResult<T> = Result<T, GatewayError>;

impl GatewayError {
    pub fn parse(reason: impl Into<String>) -> Self {
        Self {
            feature_id: crate::INPUT_FEATURE_ID,
            class: "parse",
            reason: reason.into(),
        }
    }

    pub fn validation(reason: impl Into<String>) -> Self {
        Self {
            feature_id: crate::INPUT_FEATURE_ID,
            class: "validation",
            reason: reason.into(),
        }
    }

    pub fn output(reason: impl Into<String>) -> Self {
        Self {
            feature_id: crate::OUTPUT_FEATURE_ID,
            class: "render",
            reason: reason.into(),
        }
    }
}
