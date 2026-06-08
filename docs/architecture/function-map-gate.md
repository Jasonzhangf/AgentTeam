# Function Map Gate

## Purpose

The function map is a hard implementation gate. A feature may not be implemented or modified unless the map already names its owner, contracts, allowed paths, forbidden paths, and required gates.

## Required Truths

- Every critical feature has a `feature_id`.
- Every `feature_id` appears in `docs/architecture/function-map.md`.
- Every `feature_id` appears in `docs/architecture/verification-map.md`.
- Every `feature_id` appears in `agentteam-contracts::feature_map::REQUIRED_FEATURE_IDS`.
- Every module document has a `## Module Function Map` section.
- Every module document has a `## Module Help Contract` section.
- Shared helpers belong in `agentteam-contracts` or explicit block modules, not duplicated by owners.

## Gate

```text
cargo xtask verify-function-map
```

The gate fails if a feature exists in one truth source but not the others.
