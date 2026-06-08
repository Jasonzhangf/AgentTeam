# Code Size Policy

## Purpose

AgentTeam uses file-size gates to prevent owner logic, shared blocks, and orchestration code from becoming unreviewable dumping grounds.

## Rust Source Rule

Hand-written Rust source leaf files have a hard limit:

```text
max_rust_source_file_lines = 500
```

This applies to:

- `crates/**/*.rs`
- `xtask/src/**/*.rs`

Shared functions, contracts, and blocks are not exempt. If a shared block approaches the limit, split it by domain, chain node, validator, builder, projector, or registry slice.

## Why Shared Code Is Not Exempt

Shared code has the highest blast radius. A large shared file makes ownership unclear and encourages unrelated helpers to accumulate.

`mod.rs` and `lib.rs` should stay thin re-export/composition files.

## Non-Rust Files

The 500-line hard limit does not apply to docs, examples, fixtures, `Cargo.lock`, generated files, snapshots, or persisted debug/evidence artifacts.

## Gate

```text
cargo xtask verify-code-size
```

The gate fails when a hand-written Rust source file exceeds 500 lines.

## Exceptions

Exceptions require an explicit allowlist in `xtask`, a comment explaining why split is unsafe, and a red-test or verification reason. No exception exists in the MVP scaffold.
