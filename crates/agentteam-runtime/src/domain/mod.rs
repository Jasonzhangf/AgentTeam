mod model;
mod registry;
mod resolve;
#[cfg(test)]
mod tests;

pub use model::{
    DomainEndpoint, DomainRegistrySnapshot, DomainRouteKind, DomainTargetKind, RegisteredDomain,
    ResolvedDomainTarget,
};
pub use registry::{registered_domain, DomainRegistry, DomainRegistryError, DomainRegistryResult};

pub const FEATURE_ID: &str = "domain.registry";
