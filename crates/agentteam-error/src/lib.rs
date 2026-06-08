mod classify;
mod code;
mod error;
mod model;
mod persist;
#[cfg(test)]
mod tests;

pub use classify::handle_framework_fault;
pub use error::{ErrorCenterError, ErrorCenterResult};
pub use model::ErrorCodeSeed;

pub const FEATURE_ID: &str = "error.center";
