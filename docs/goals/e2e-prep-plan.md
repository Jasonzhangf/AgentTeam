# E2E Prep Plan

## Goal

Finish all pre-E2E work for AgentTeam so the local operator can run a minimal end-to-end loop in `~/code/playground` using `agentteam start` from the current `cwd` as the bootstrap entrypoint.

## Acceptance Criteria

- Basic runtime/test gates pass locally.
- Communication paths work for one-to-one, ready report, and broadcast delivery receipts.
- A simple task loop works end to end: create task, claim task, update task, complete/error task, and inspect status.
- CLI/help surfaces reflect the implemented commands.
- All required docs, function-map entries, verification-map entries, and red-test expectations are updated.

## Scope

### In scope

- Local verification and smoke coverage for:
  - config check
  - domain resolution
  - debug snapshot
  - message send
  - ready report
  - broadcast delivery
  - task send/list/status/claim/done/error
  - tmux loopback smoke
- Communication Center persistence receipts for delivery events.
- Gateway/Runtime/CLI wiring for the above commands.
- Required docs, function map, verification map, and red-test updates.
- Playground-based local execution in `~/code/playground`.

### Out of scope

- Full daemon deployment hardening.
- WebUI product work beyond input/output projection boundaries.
- Multi-daemon runtime expansion beyond the current domain contract.
- Advanced scheduling, memory limits, or non-MVP optimization work.

## Target Docs

- `docs/architecture/overview.md`
- `docs/architecture/ascii-flows.md`
- `docs/architecture/function-map.md`
- `docs/architecture/verification-map.md`
- `docs/architecture/mvp-start-gate.md`
- `docs/architecture/mvp-debug-build.md`
- `docs/modules/03-communication-center.md`
- `docs/modules/09-task-engine.md`
- `docs/modules/13-cli-agent-skill.md`
- `docs/modules/15-startup-session-manager.md`
- `docs/modules/17-resource-lifecycle-manager.md`
- `docs/goals/mvp-runtime-vertical-slice-plan.md`

## Execution Rules

- Use Rust-first implementation only.
- Keep function-map coverage ahead of implementation.
- Do not add fallback, downgrade, swallow, or success-wrapped errors.
- Keep module ownership isolated; do not let gateways or CLI own business truth.
- Use `~/code/playground` as the working directory for live smoke and E2E prep.
- Start agents with `agentteam start` from the target `cwd`; the configured manager then uses skills/CLI to initialize workers.
- Prefer a single owner path per feature and keep shared helpers in the shared contract/block layer.
- Preserve payload semantics across all transport and message paths.

## Suggested Implementation Order

1. Verify current gates and the existing ready/message/task/tmux slices.
2. Finish any missing broadcast persistence or receipt plumbing.
3. Close any remaining task-board query/claim delivery receipt gaps.
4. Refresh CLI/help/docs for the final command surface.
5. Run the full verification stack and one playground smoke loop.
6. Record stable findings in `MEMORY.md` and short-run context in `CACHE.md`.

## Verification

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo xtask red-tests`
- `cargo xtask verify-required-files`
- `cargo xtask verify-skill-frontmatter`
- `cargo xtask verify-resource-lifecycle`
- `cargo xtask verify-function-map`
- `cargo xtask verify-code-size`
- `cargo xtask verify`
- playground smoke in `~/code/playground`

## Completion Signal

The pre-E2E phase is complete only when:

- the required verification stack passes locally,
- the command surface for basic communication and simple tasks is complete,
- and a minimal playground smoke can run from `~/code/playground` with explicit receipts and no hidden fallback.
