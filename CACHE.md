# CACHE

2026-06-08:
- AgentTeam is Rust-first, tmux/zterm-backed, but current completed scope is MVP scaffold Phase 0 + Phase 1 only: no business runtime, no daemon start, no real tmux/zterm execution.
- Use `agentteam-dev` for development. Required truth docs: `docs/goals/mvp-runtime-vertical-slice-plan.md`, `docs/architecture/function-map.md`, `docs/architecture/verification-map.md`.
- Phase 0 implemented `xtask/src/red_tests.rs` scans for broad kill, transport internal leaks, TOML owner, state-file owner, domain parsing owner, non-adjacent `From/TryFrom` conversions, and contract feature ids.
- User added hard rule: every new/changed Rust function must be in function map. Implemented in `xtask/src/function_map.rs`; `verify-function-map` scans `crates/` and `xtask/src/` functions. Canonical `cargo xtask verify` runs this before compile/test gates.
- Phase 1 implemented typed contracts in `agentteam-contracts`: config, domain, persist, error, debug, resource. Unit tests: 13 contract tests.
- All full gates passed after staging required new files via `cargo xtask verify`: function-map first, then fmt, clippy, test, red-tests, required-files, skill-frontmatter, resource-lifecycle, code-size.
- Commit `2196768` contains Phase 0/1.
- Phase 2 started after commit: Config Center now has owner files `error/load/model/parse/validate/normalize/snapshot/tests`, dependencies `serde` and `toml`, and 7 unit tests. `cargo xtask verify` passed before Phase 2 commit.
- Commit `d081d1d` contains Phase 2.
- Phase 3 started after commit: Daemon Domain Registry now lives under `agentteam-runtime/src/domain/` with model/registry/resolve/tests. It resolves local/remote domain targets and rejects unknown remote fallback. `cargo xtask verify` passed before Phase 3 commit.
- Commit `17f08f6` contains Phase 3.
- Phase 4 implemented `agentteam-persist` owner modules for typed JSONL append/replay/materialize. It validates event drafts, returns append receipts, detects corrupt JSONL and sequence mismatch, and materializes latest sequence/snapshot id.
- `cargo xtask verify` passed for Phase 4 before commit: function-map, fmt, clippy, workspace tests, red-tests, required-files, skill-frontmatter, resource-lifecycle, code-size.
- Next phase after Phase 4 commit: Phase 5 Error Center (`docs/modules/02-error-center.md`) using Persistence append instead of writing files directly.
- Commit `b82467d` contains Phase 4.
- Phase 5 implemented `agentteam-error` owner modules. External API is `handle_framework_fault`; internals classify faults, generate code/evidence id, persist framework_error via `agentteam-persist`, then project. Normal task error facts are rejected from Error Center.
- `cargo xtask verify` passed during Phase 5 implementation before commit. Next phase after commit: Phase 6 Debug Center and Resource Lifecycle MVP (`docs/modules/10-debug-center.md`, `docs/modules/17-resource-lifecycle-manager.md`).
