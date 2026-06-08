use agentteam_resource::ResourceRegistrySnapshot;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugBundleInput {
    pub requested_by: String,
    pub scope: String,
    pub module: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DebugBundlePayload {
    pub bundle_id: String,
    pub requested_by: String,
    pub scope: String,
    pub module: String,
    pub resource_snapshot: ResourceRegistrySnapshot,
}
