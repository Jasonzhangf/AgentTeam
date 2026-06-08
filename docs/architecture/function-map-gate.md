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
- Every hand-written Rust function or method under `crates/` and `xtask/src/` has a function-level registry entry in `docs/architecture/function-map.md`.
- Every function-level registry entry states the symbol, owner, feature id, allowed paths, and required gates.
- Function changes must update the same registry entry in the same change set.
- Shared helpers belong in `agentteam-contracts` or explicit block modules, not duplicated by owners.

## Gate

```text
cargo xtask verify-function-map
```

The gate fails if a feature exists in one truth source but not the others.

It also fails if a Rust function or method declaration exists without a matching function-level registry entry.

The canonical full verifier `cargo xtask verify` must run this gate before fmt, clippy, test, or other compile-style checks.
