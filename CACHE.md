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
