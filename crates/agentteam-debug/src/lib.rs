mod bundle;
mod error;
mod model;
mod persist;
#[cfg(test)]
mod tests;

pub use bundle::capture_debug_bundle;
pub use error::{DebugError, DebugResult};
pub use model::{DebugBundleInput, DebugBundlePayload};

pub const FEATURE_ID: &str = "debug.center";
