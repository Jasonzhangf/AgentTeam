use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LeaseState {
    Active,
    Released,
    LeakSuspected,
    OrphanSuspected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceAcquireInput {
    pub owner_module: String,
    pub owner_entity_id: String,
    pub resource_class: String,
    pub scope: String,
    pub memory_bytes_estimate: u64,
    pub handle_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceLeaseRecord {
    pub resource_id: String,
    pub lease_id: String,
    pub owner_module: String,
    pub owner_entity_id: String,
    pub resource_class: String,
    pub scope: String,
    pub state: LeaseState,
    pub memory_bytes_estimate: u64,
    pub handle_count: u32,
    pub last_event_id: String,
    pub receipt_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceRegistrySnapshot {
    pub snapshot_id: String,
    pub active_count: usize,
    pub released_count: usize,
    pub leak_suspect_count: usize,
    pub orphan_suspect_count: usize,
    pub total_memory_bytes_estimate: u64,
    pub total_handle_count: u32,
    pub latest_event_id: String,
}
