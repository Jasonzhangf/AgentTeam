use std::collections::BTreeMap;
use std::path::Path;

use agentteam_contracts::resource::{
    ResourceLease04Active, ResourceReq01AcquireIntent, ResourceResp05Released,
};

use crate::error::{ResourceError, ResourceResult};
use crate::model::{LeaseState, ResourceAcquireInput, ResourceLeaseRecord};
use crate::persist::persist_resource_event;
use crate::snapshot::snapshot_registry;

#[derive(Debug, Default)]
pub struct ResourceRegistry {
    leases: BTreeMap<String, ResourceLeaseRecord>,
    next_sequence: u64,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            leases: BTreeMap::new(),
            next_sequence: 1,
        }
    }

    pub fn acquire(
        &mut self,
        log_path: impl AsRef<Path>,
        input: ResourceAcquireInput,
    ) -> ResourceResult<ResourceLease04Active> {
        validate_acquire_input(&input)?;
        let resource_id = format!("resource-{:020}", self.next_sequence);
        let lease_id = format!("lease-{:020}", self.next_sequence);
        self.next_sequence += 1;
        let active = ResourceReq01AcquireIntent::new(
            input.owner_module.clone(),
            input.owner_entity_id.clone(),
            input.resource_class.clone(),
        )
        .validate_scope(input.scope.clone())
        .initial_metric(input.memory_bytes_estimate, input.handle_count)
        .activate(resource_id.clone(), lease_id.clone());
        let mut record = record_from_active(&active, &input);
        let receipt = persist_resource_event(&log_path, "resource_acquire", &record)?;
        record.last_event_id = receipt.event_id;
        record.receipt_id = receipt_id_for_sequence(receipt.sequence);
        self.leases.insert(lease_id, record);
        Ok(active)
    }

    pub fn release(
        &mut self,
        log_path: impl AsRef<Path>,
        lease_id: &str,
        owner_module: &str,
    ) -> ResourceResult<ResourceResp05Released> {
        let record = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| ResourceError::NotFound {
                lease_id: lease_id.to_owned(),
            })?;
        if record.owner_module != owner_module {
            return Err(ResourceError::NotOwner {
                lease_id: lease_id.to_owned(),
                owner_module: owner_module.to_owned(),
            });
        }
        record.state = LeaseState::Released;
        let receipt = persist_resource_event(&log_path, "resource_release", record)?;
        record.last_event_id = receipt.event_id.clone();
        record.receipt_id = receipt_id_for_sequence(receipt.sequence);
        Ok(ResourceLease04Active {
            resource_id: record.resource_id.clone(),
            lease_id: record.lease_id.clone(),
            owner_module: record.owner_module.clone(),
            owner_entity_id: record.owner_entity_id.clone(),
            resource_class: record.resource_class.clone(),
            scope: record.scope.clone(),
        }
        .release(receipt.event_id, record.receipt_id.clone()))
    }

    pub fn mark_leak(
        &mut self,
        log_path: impl AsRef<Path>,
        lease_id: &str,
        reason: &str,
    ) -> ResourceResult<()> {
        if reason.trim().is_empty() {
            return Err(ResourceError::Validation {
                reason: "leak reason must not be empty".to_owned(),
            });
        }
        let record = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| ResourceError::NotFound {
                lease_id: lease_id.to_owned(),
            })?;
        record.state = LeaseState::LeakSuspected;
        let receipt = persist_resource_event(&log_path, "resource_leak_suspected", record)?;
        record.last_event_id = receipt.event_id;
        record.receipt_id = receipt_id_for_sequence(receipt.sequence);
        Ok(())
    }

    pub fn snapshot(&self) -> crate::model::ResourceRegistrySnapshot {
        snapshot_registry(self)
    }

    pub(crate) fn leases(&self) -> impl Iterator<Item = &ResourceLeaseRecord> {
        self.leases.values()
    }
}

fn validate_acquire_input(input: &ResourceAcquireInput) -> ResourceResult<()> {
    for (field, value) in [
        ("owner_module", input.owner_module.as_str()),
        ("owner_entity_id", input.owner_entity_id.as_str()),
        ("resource_class", input.resource_class.as_str()),
        ("scope", input.scope.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(ResourceError::Validation {
                reason: format!("{field} must not be empty"),
            });
        }
    }
    Ok(())
}

fn record_from_active(
    active: &ResourceLease04Active,
    input: &ResourceAcquireInput,
) -> ResourceLeaseRecord {
    ResourceLeaseRecord {
        resource_id: active.resource_id.clone(),
        lease_id: active.lease_id.clone(),
        owner_module: active.owner_module.clone(),
        owner_entity_id: active.owner_entity_id.clone(),
        resource_class: active.resource_class.clone(),
        scope: active.scope.clone(),
        state: LeaseState::Active,
        memory_bytes_estimate: input.memory_bytes_estimate,
        handle_count: input.handle_count,
        last_event_id: String::new(),
        receipt_id: String::new(),
    }
}

fn receipt_id_for_sequence(sequence: u64) -> String {
    format!("receipt-{sequence:020}")
}
