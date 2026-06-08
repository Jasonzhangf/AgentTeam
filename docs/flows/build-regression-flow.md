# Build Regression Flow

## Current Phase

MVP scaffold may include Rust workspace/crate/gate skeletons. Current verification is:

```text
find docs -type f | sort
git status --short
```

## Future Rust Flow

Canonical full verification:

```text
cargo xtask verify
```

`cargo xtask verify` runs `cargo xtask verify-function-map` first. If a new or changed Rust function is missing from the function map, verification stops before compile/test gates.

Expanded gate list:

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

## Required File Gate

`cargo xtask verify-required-files` must prove:

- all crate manifests exist
- all crate entry files exist
- all module docs exist
- function map exists
- verification map exists
- red-test plan exists
- config example exists
- local skills exist
- local skills have valid YAML frontmatter
- required files are tracked by git

## Function Map Gate

`cargo xtask verify-function-map` must prove:

- each critical `feature_id` is listed in function map
- each critical `feature_id` is listed in verification map
- contracts constants include the same required feature ids
- every module doc has a module function map
- every module doc has a module help contract
- every Rust function or method under `crates/` and `xtask/src/` has a matching function-level registry entry
- new or changed functions cannot pass the gate until `docs/architecture/function-map.md` is updated

## Code Size Gate

`cargo xtask verify-code-size` must prove:

- every hand-written Rust source leaf file is 500 lines or less
- shared functions/contracts/blocks are not exempt
- large shared behavior is split by owner/domain/node

## Resource Lifecycle Gate

`cargo xtask verify-resource-lifecycle` must prove:

- every long-lived resource class has an owner module
- every owner module documents acquire/release behavior
- every resource class has orphan/leak red tests
- cleanup uses exact handles, not broad process or file operations
- MVP debug bundle includes Resource Lifecycle snapshot

## No Completion Without Evidence

Valid reports must include:

- changed files
- command evidence
- failed gates if any
- remaining risks
