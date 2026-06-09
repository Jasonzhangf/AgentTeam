---
name: agentteam-dev
description: Use when developing AgentTeam modules, adding or changing Rust code, module docs, function maps, verification maps, red tests, gates, or architecture contracts in the AgentTeam repository.
---

# AgentTeam Dev Skill

Use this skill for module development inside the AgentTeam repository.

This is the local development-agent playbook. It is separate from the `agentteam` skill, which is the future runtime/CLI collaboration surface for the configured manager and worker agents.

## Required Reading

1. `AGENTS.md`
2. `CACHE.md`
3. `MEMORY.md`
4. `note.md`
5. `docs/architecture/overview.md`
6. `docs/architecture/ascii-flows.md`
7. `docs/architecture/function-map.md`
8. `docs/architecture/verification-map.md`
9. target module doc under `docs/modules/`

## Module Development Cycle

```text
restore context
  -> lock function map
  -> review module doc
  -> define contracts
  -> add red tests
  -> implement owner module
  -> run gates
  -> persist knowledge
```

## Step 1: Restore Context

- Read required files before changing code or docs.
- Record exploration findings in `note.md`.
- Do not start implementation from memory alone.

## Step 2: Lock Function Map

Before implementation or modification, confirm:

- `feature_id` exists in `docs/architecture/function-map.md`
- `feature_id` exists in `docs/architecture/verification-map.md`
- `feature_id` exists in `agentteam-contracts::feature_map::REQUIRED_FEATURE_IDS`
- target module doc has `## Module Function Map`
- target module doc has `## Module Help Contract`
- owner, allowed paths, forbidden paths, and required gates are explicit

Run:

```text
cargo xtask verify-function-map
```

If the map is missing or ambiguous, fix the map before implementation.

## Step 3: Review Module Doc

Each module doc must define:

- purpose
- owns / does not own
- module function map
- module help contract
- public API boundary
- required behavior
- error behavior
- debug snapshot
- resource lifecycle
- red tests
- open decisions

## Step 4: Define Contracts

Pipeline nodes must use:

```text
<Domain><Direction><NN><Node>
```

Rules:

- one builder/parser/projector owner per node
- adjacent-node conversion only
- no scattered non-adjacent `From` conversions
- shared semantics live in `agentteam-contracts` or explicit block modules
- errors enter the `TeamErr*` chain and Error Center
- debug metadata never becomes business payload

## Step 5: Add Red Tests

Add or update red tests before behavior implementation.

Cover:

- duplicate owner logic
- bypassing the owning module
- private-state access
- fallback/downgrade/success-wrapped error
- non-adjacent pipeline conversion
- missing event persistence
- missing debug/evidence/resource lifecycle path
- missing required tracked file

## Step 6: Implement Owner Module

- Keep orchestration pure.
- Keep module APIs isolated.
- Put reusable helpers in shared contracts/blocks.
- Reuse `agentteam-contracts::event_hash::event_payload_hash` for persisted event payload hashes; do not add per-module duplicate hash helpers.
- Treat configured sample names such as `Kevin` as data only; Rust declarations, schema fields, feature ids, function ids, and red-test ids must use neutral manager/root-manager/configured-agent names.
- Do not duplicate owner logic in adapters, gateways, UI, or runtime.
- Keep hand-written Rust leaf files at 500 lines or less.
- Do not implement business behavior for a module whose docs/red tests are incomplete.

## Step 7: Run Gates

Required gate:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask red-tests
cargo xtask verify-required-files
cargo xtask verify-skill-frontmatter
cargo xtask verify-resource-lifecycle
cargo xtask verify-function-map
cargo xtask verify-code-size
```

Do not claim completion until relevant gates pass.

## Step 8: Persist Knowledge

- Keep raw exploration in `note.md`.
- Add stable verified truths to `MEMORY.md`.
- Compress current working context into `CACHE.md`.
- Update local skills only when the reusable workflow changes.

## Done Definition

Module development is done only when:

- docs are complete
- function map and verification map agree
- red tests name expected failure modes
- owner paths and forbidden paths are explicit
- resource lifecycle and debug snapshot are specified
- implementation, if any, is inside the owner boundary
- required gates pass
