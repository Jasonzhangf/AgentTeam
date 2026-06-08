# AgentTeam Project Rules

## Scope

AgentTeam is a Rust-first multi-agent runtime framework built on tmux and the zterm daemon/terminal mirror mechanism.

Current phase: MVP scaffold may be implemented. Business runtime behavior still requires module-specific docs, function map, verification map, and red tests before implementation.

## Hard Guards

1. Address Jason in every response.
2. Verify before claiming completion.
3. No fallback, downgrade, silent compensation, swallowed exception, or success-wrapped error.
4. No broad process kill. Stop daemon only by explicit PID or service-scoped shutdown.
5. No duplicated owner logic. Shared behavior must live in one shared block/contract layer.
6. No module may read or mutate another module's internal state directly.
7. Every critical feature needs an owner, public API boundary, red test, and verification gate.
8. Every tracked build path must be explicit. Future build scripts must fail when required source/config/test files are missing or untracked.
9. zterm terminal transport remains an external adapter truth. AgentTeam owns team/task/message/config/debug/error orchestration truth.
10. Rust is the implementation language for runtime, daemon, CLI, gateways, contracts, and governance.
11. Local skills must have valid YAML frontmatter or Kevin/worker operating instructions are considered unavailable.
12. Every long-lived resource must have an owner-scoped lifecycle lease and debug-visible release/orphan/leak evidence.
13. Function map is a hard gate: no feature implementation or modification without `feature_id`, owner, allowed paths, forbidden paths, required gates, and verification mapping.
14. Hand-written Rust source leaf files are limited to 500 lines. Shared blocks/contracts are not exempt; split them by owner/domain/node instead.
15. Module development cycle belongs to the local `agentteam-dev` skill. The `agentteam` skill remains the runtime/CLI collaboration surface.
16. Daemon domains are the cross-daemon naming boundary. Agent Registry owns domain-local names only; Daemon Domain Registry owns `agent@domain` parsing and route resolution.

## Rust Naming Contract

Pipeline types use:

`<Domain><Direction><NN><Node>`

Examples:

- `TeamReq01CliRaw`
- `TeamReq02ParsedCommand`
- `TeamReq03ValidatedIntent`
- `TeamReq04DaemonCommand`
- `TeamResp05DaemonResult`
- `TeamResp06CliRendered`
- `TeamErr01CliParse`
- `TeamErr02Validation`
- `TeamErr03DaemonRuntime`
- `TeamErr04Transport`

Rules:

- One builder/parser/projector owner per node.
- Only adjacent node conversion is allowed.
- No scattered `From` conversions across non-adjacent nodes.
- Errors must enter `TeamErr*` chain and the Error Center.
- Debug/snapshot metadata must never become normal business payload.

## Module Ownership

Primary owner map: `docs/architecture/function-map.md`

Verification map: `docs/architecture/verification-map.md`

Module requirements:

- `docs/modules/01-config-center.md`
- `docs/modules/02-error-center.md`
- `docs/modules/03-communication-center.md`
- `docs/modules/04-input-gateway.md`
- `docs/modules/05-output-gateway.md`
- `docs/modules/06-ui-gateway.md`
- `docs/modules/07-agent-registry-naming-pool.md`
- `docs/modules/08-team-orchestrator.md`
- `docs/modules/09-task-engine.md`
- `docs/modules/10-debug-center.md`
- `docs/modules/11-persistence-event-log.md`
- `docs/modules/12-zterm-tmux-adapter.md`
- `docs/modules/13-cli-agent-skill.md`
- `docs/modules/14-tui-agent-adapter-center.md`
- `docs/modules/15-startup-session-manager.md`
- `docs/modules/16-tanote-collaboration-board.md`
- `docs/modules/17-resource-lifecycle-manager.md`
- `docs/modules/18-daemon-domain-registry.md`

## Required Flow Before Code

1. Read this file.
2. Read `CACHE.md`, `MEMORY.md`, and `note.md`.
3. Read `docs/architecture/overview.md`.
4. Read `docs/architecture/ascii-flows.md`.
5. Read the target module requirement doc.
6. Check function map and verification map.
7. Only then design or implement.
8. For MVP scaffold work, read `docs/architecture/mvp-start-gate.md` before editing Rust.

## Documentation Rule

Exploration findings go to `note.md`.

Stable verified project truths go to `MEMORY.md`.

Short context restoration goes to `CACHE.md`.

## Build Gate Direction

Future Rust implementation must include:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- architecture/red-test runner
- untracked-required-file gate
- local skill frontmatter gate
- resource lifecycle gate
- function map gate
- code-size gate

During MVP scaffold, docs remain the behavior truth. Rust code may create workspace/crate/gate skeletons only until a module implementation is explicitly started.
