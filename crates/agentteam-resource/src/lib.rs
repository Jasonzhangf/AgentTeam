mod error;
mod model;
mod persist;
mod registry;
mod snapshot;
#[cfg(test)]
mod tests;

pub use error::{ResourceError, ResourceResult};
pub use model::{LeaseState, ResourceAcquireInput, ResourceLeaseRecord, ResourceRegistrySnapshot};
pub use registry::ResourceRegistry;
pub use snapshot::snapshot_registry;

pub const FEATURE_ID: &str = "resource.lifecycle";
