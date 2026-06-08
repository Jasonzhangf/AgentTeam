use std::path::Path;

use agentteam_contracts::debug::{DebugReq01SnapshotIntent, DebugResp03Bundle};
use agentteam_resource::{ResourceAcquireInput, ResourceRegistry};

use crate::error::{resource_error, DebugError, DebugResult};
use crate::model::{DebugBundleInput, DebugBundlePayload};
use crate::persist::persist_debug_bundle;

pub fn capture_debug_bundle(
    log_path: impl AsRef<Path>,
    resources: &mut ResourceRegistry,
    input: DebugBundleInput,
) -> DebugResult<DebugResp03Bundle> {
    validate_input(&input)?;
    let bundle_id = bundle_id(&input);
    let lease = resources
        .acquire(&log_path, resource_input(&input, &bundle_id))
        .map_err(resource_error)?;
    let resource_snapshot = resources.snapshot();
    let payload = DebugBundlePayload {
        bundle_id: bundle_id.clone(),
        requested_by: input.requested_by.clone(),
        scope: input.scope.clone(),
        module: input.module.clone(),
        resource_snapshot: resource_snapshot.clone(),
    };
    let receipt = persist_debug_bundle(&log_path, &payload)?;
    resources
        .release(&log_path, &lease.lease_id, "debug.center")
        .map_err(resource_error)?;
    Ok(
        DebugReq01SnapshotIntent::new(input.requested_by, input.scope)
            .request_module(input.module)
            .bundle(
                bundle_id,
                receipt_id_for_sequence(receipt.sequence),
                resource_snapshot.snapshot_id,
            ),
    )
}

fn validate_input(input: &DebugBundleInput) -> DebugResult<()> {
    for (field, value) in [
        ("requested_by", input.requested_by.as_str()),
        ("scope", input.scope.as_str()),
        ("module", input.module.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(DebugError::Validation {
                reason: format!("{field} must not be empty"),
            });
        }
    }
    Ok(())
}

fn resource_input(input: &DebugBundleInput, bundle_id: &str) -> ResourceAcquireInput {
    ResourceAcquireInput {
        owner_module: "debug.center".to_owned(),
        owner_entity_id: bundle_id.to_owned(),
        resource_class: "debug_bundle".to_owned(),
        scope: input.scope.clone(),
        memory_bytes_estimate: 256,
        handle_count: 1,
    }
}

fn bundle_id(input: &DebugBundleInput) -> String {
    format!("debug-bundle-{}-{}", input.scope, input.module)
}

fn receipt_id_for_sequence(sequence: u64) -> String {
    format!("receipt-{sequence:020}")
}
