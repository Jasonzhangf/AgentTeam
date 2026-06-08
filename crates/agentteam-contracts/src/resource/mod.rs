use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "resource.lifecycle";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceReq01AcquireIntent {
    pub owner_module: String,
    pub owner_entity_id: String,
    pub resource_class: String,
}

impl ResourceReq01AcquireIntent {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Resource", "Req", 1, "AcquireIntent");

    pub fn new(
        owner_module: impl Into<String>,
        owner_entity_id: impl Into<String>,
        resource_class: impl Into<String>,
    ) -> Self {
        Self {
            owner_module: owner_module.into(),
            owner_entity_id: owner_entity_id.into(),
            resource_class: resource_class.into(),
        }
    }

    pub fn validate_scope(self, scope: impl Into<String>) -> ResourceReq02ValidatedScope {
        ResourceReq02ValidatedScope {
            owner_module: self.owner_module,
            owner_entity_id: self.owner_entity_id,
            resource_class: self.resource_class,
            scope: scope.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceReq02ValidatedScope {
    pub owner_module: String,
    pub owner_entity_id: String,
    pub resource_class: String,
    pub scope: String,
}

impl ResourceReq02ValidatedScope {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Resource", "Req", 2, "ValidatedScope");

    pub fn initial_metric(
        self,
        memory_bytes_estimate: u64,
        handle_count: u32,
    ) -> ResourceMetric03Initial {
        ResourceMetric03Initial {
            owner_module: self.owner_module,
            owner_entity_id: self.owner_entity_id,
            resource_class: self.resource_class,
            scope: self.scope,
            memory_bytes_estimate,
            handle_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceMetric03Initial {
    pub owner_module: String,
    pub owner_entity_id: String,
    pub resource_class: String,
    pub scope: String,
    pub memory_bytes_estimate: u64,
    pub handle_count: u32,
}

impl ResourceMetric03Initial {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("ResourceMetric", "", 3, "Initial");

    pub fn activate(
        self,
        resource_id: impl Into<String>,
        lease_id: impl Into<String>,
    ) -> ResourceLease04Active {
        ResourceLease04Active {
            resource_id: resource_id.into(),
            lease_id: lease_id.into(),
            owner_module: self.owner_module,
            owner_entity_id: self.owner_entity_id,
            resource_class: self.resource_class,
            scope: self.scope,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLease04Active {
    pub resource_id: String,
    pub lease_id: String,
    pub owner_module: String,
    pub owner_entity_id: String,
    pub resource_class: String,
    pub scope: String,
}

impl ResourceLease04Active {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("ResourceLease", "", 4, "Active");

    pub fn release(
        self,
        release_event_id: impl Into<String>,
        receipt_id: impl Into<String>,
    ) -> ResourceResp05Released {
        ResourceResp05Released {
            resource_id: self.resource_id,
            lease_id: self.lease_id,
            release_event_id: release_event_id.into(),
            receipt_id: receipt_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceResp05Released {
    pub resource_id: String,
    pub lease_id: String,
    pub release_event_id: String,
    pub receipt_id: String,
}

impl ResourceResp05Released {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Resource", "Resp", 5, "Released");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_chain_requires_active_lease_before_release() {
        let released = ResourceReq01AcquireIntent::new("debug.center", "bundle-1", "debug_bundle")
            .validate_scope("project")
            .initial_metric(128, 1)
            .activate("resource-1", "lease-1")
            .release("event-1", "receipt-1");

        assert_eq!(released.resource_id, "resource-1");
        assert_eq!(released.receipt_id, "receipt-1");
        assert_eq!(ResourceReq01AcquireIntent::NODE.number, 1);
        assert_eq!(ResourceResp05Released::NODE.number, 5);
    }

    #[test]
    fn resource_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "resource.lifecycle");
    }
}
