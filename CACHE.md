# CACHE

2026-06-08:
- Current verified slice: Task Engine claim routing and local CLI claim command.
- `TaskEngine::claim_task` now owns assigned-first / blocked-first / priority-then-age selection and persists `task_claimed`.
- `agentteam-gateway` parses `task claim --runtime-home ... --worker-name ... --worker-role ... --json`.
- `agentteam-runtime::local` executes claim through the task engine and projects it as `LocalCommandResult::TaskClaim`.
- Verified: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo xtask verify-code-size`, `cargo xtask verify-function-map`, `cargo xtask verify`.
