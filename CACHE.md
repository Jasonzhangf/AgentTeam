# CACHE

2026-06-08:
- Use `agentteam-dev` for repo development. Required truth docs: `docs/goals/mvp-runtime-vertical-slice-plan.md`, `docs/architecture/function-map.md`, `docs/architecture/verification-map.md`.
- Hard gates: every Rust function under `crates/` and `xtask/src/` must be in `docs/architecture/function-map.md`; required new files must be tracked before `verify-required-files`; hand-written Rust leaf files stay under 500 lines.
- Completed commits through daemon check: `e3b7055 feat(cli): add daemon check`.
- Current completed slice: Phase 8 Task Engine local MVP. `agentteam-runtime::task` owns task event state; local CLI supports `task send/list/status/done/error` with `--runtime-home`, persists through `agentteam-persist`, and replays `events/agentteam.jsonl` for board/status.
- Shared event hash truth: persisted event payload hash uses `agentteam-contracts::event_hash::event_payload_hash`; per-module duplicate hash helpers were removed from debug/error/resource.
- Current CLI task smoke passed: `task send` created `AT-000001`, `task list` replayed it, `task done` wrote sequence 2, `task status` returned done, and invalid `task error AT-404` returned explicit task error.
- Latest full verification passed: `cargo xtask verify`.
- Still out of scope: no daemon loop, no tmux/zterm launch, no Communication Center routing, no TANote implementation, no task claim/scheduling/role concurrency yet.
- Next main phase: Communication Center envelope routing or Task Engine claim/schedule slice, depending on desired vertical path.
