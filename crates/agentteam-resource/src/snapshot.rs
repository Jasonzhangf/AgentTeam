use crate::model::{LeaseState, ResourceRegistrySnapshot};
use crate::registry::ResourceRegistry;

pub fn snapshot_registry(registry: &ResourceRegistry) -> ResourceRegistrySnapshot {
    let mut active_count = 0;
    let mut released_count = 0;
    let mut leak_suspect_count = 0;
    let mut orphan_suspect_count = 0;
    let mut total_memory_bytes_estimate = 0u64;
    let mut total_handle_count = 0u32;
    let mut latest_event_id = String::new();

    for lease in registry.leases() {
        match lease.state {
            LeaseState::Active => active_count += 1,
            LeaseState::Released => released_count += 1,
            LeaseState::LeakSuspected => leak_suspect_count += 1,
            LeaseState::OrphanSuspected => orphan_suspect_count += 1,
        }
        total_memory_bytes_estimate =
            total_memory_bytes_estimate.saturating_add(lease.memory_bytes_estimate);
        total_handle_count = total_handle_count.saturating_add(lease.handle_count);
        latest_event_id = lease.last_event_id.clone();
    }

    ResourceRegistrySnapshot {
        snapshot_id: format!("resource-snapshot-{}", latest_event_id),
        active_count,
        released_count,
        leak_suspect_count,
        orphan_suspect_count,
        total_memory_bytes_estimate,
        total_handle_count,
        latest_event_id,
    }
}
