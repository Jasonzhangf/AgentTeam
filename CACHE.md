# CACHE

2026-06-08:
- Current verified slice: Communication Center routing for message, broadcast, ready report, task-board query, and task claim.
- `agentteam-contracts::comm` now has adjacent-node contracts for all comm routes; tests were split out of `comm/mod.rs` to satisfy the 500-line Rust leaf limit.
- `agentteam-comm` exports `route_message`, `route_broadcast`, `route_ready_report`, `route_task_board_query`, and `route_task_claim`.
- Verified: `cargo test -p agentteam-contracts -p agentteam-comm`, `cargo xtask verify-function-map`, `cargo xtask verify`.
- `cargo xtask verify` passed after the comm split and code-size gate is clean.
