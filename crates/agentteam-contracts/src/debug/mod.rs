use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "debug.center";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugReq01SnapshotIntent {
    pub requested_by: String,
    pub scope: String,
}

impl DebugReq01SnapshotIntent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Debug", "Req", 1, "SnapshotIntent");

    pub fn new(requested_by: impl Into<String>, scope: impl Into<String>) -> Self {
        Self {
            requested_by: requested_by.into(),
            scope: scope.into(),
        }
    }

    pub fn request_module(self, module: impl Into<String>) -> DebugReq02ModuleSnapshotRequest {
        DebugReq02ModuleSnapshotRequest {
            requested_by: self.requested_by,
            scope: self.scope,
            module: module.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugReq02ModuleSnapshotRequest {
    pub requested_by: String,
    pub scope: String,
    pub module: String,
}

impl DebugReq02ModuleSnapshotRequest {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Debug", "Req", 2, "ModuleSnapshotRequest");

    pub fn bundle(
        self,
        bundle_id: impl Into<String>,
        persistence_receipt_id: impl Into<String>,
        resource_snapshot_id: impl Into<String>,
    ) -> DebugResp03Bundle {
        DebugResp03Bundle {
            bundle_id: bundle_id.into(),
            persistence_receipt_id: persistence_receipt_id.into(),
            resource_snapshot_id: resource_snapshot_id.into(),
            module_count: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugResp03Bundle {
    pub bundle_id: String,
    pub persistence_receipt_id: String,
    pub resource_snapshot_id: String,
    pub module_count: usize,
}

impl DebugResp03Bundle {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Debug", "Resp", 3, "Bundle");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_bundle_requires_persistence_receipt() {
        let bundle = DebugReq01SnapshotIntent::new("Kevin", "project")
            .request_module("resource.lifecycle")
            .bundle("bundle-1", "receipt-1", "resource-snapshot-1");

        assert_eq!(bundle.persistence_receipt_id, "receipt-1");
        assert_eq!(bundle.resource_snapshot_id, "resource-snapshot-1");
        assert_eq!(DebugReq01SnapshotIntent::NODE.number, 1);
        assert_eq!(DebugResp03Bundle::NODE.number, 3);
    }

    #[test]
    fn debug_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "debug.center");
    }
}
