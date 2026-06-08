# MVP Runtime Vertical Slice Plan

## Goal

Build the first executable AgentTeam runtime slice in Rust, moving from documentation scaffold to a verifiable local daemon/CLI runtime without implementing tmux/zterm process control yet.

The slice must establish durable truth paths first:

```text
config -> domain -> persistence -> error -> debug -> CLI/daemon API
```

Then add team/task/message/note/resource behavior behind the same gates.

## Acceptance Criteria

- `agentteam config check --config docs/config/config.toml.example --json` validates user config and domain config.
- `agentteam daemon check --config docs/config/config.toml.example --json` exercises daemon command routing without starting tmux.
- `agentteam debug snapshot --config docs/config/config.toml.example --json` persists a debug bundle and returns a bundle id plus receipt.
- `agentteam domain resolve --target Alice@review-daemon --config docs/config/config.toml.example --json` returns a typed route plan.
- Event log append/replay is implemented for config/domain/debug/error/resource events.
- Error Center produces persisted classified errors with severity, code, and evidence id.
- All touched features have unit tests plus red tests.
- Full gate passes:

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

## Scope

### In Scope

- Config Center TOML parse, validation, normalization, redaction.
- Daemon Domain Registry domain id validation, aliases, `agent@domain` parsing, local/remote route plan.
- Persistence Event Log typed JSONL append and replay for MVP event types.
- Error Center classified error event creation with evidence id.
- Debug Center persisted debug bundle for implemented modules.
- Resource Lifecycle Manager minimum lease registry for implemented resources.
- CLI commands for config/domain/debug/daemon-check.
- `agentteamd` minimal loopback daemon API skeleton only if needed for the slice.
- Contract types and adjacent pipeline builders/parsers.
- Architecture red tests implemented in `xtask` where static scanning is appropriate.

### Out Of Scope

- Real tmux session creation.
- Real zterm bridge connection.
- Real TUI provider SDK integration.
- Full WebUI.
- Full multi-agent scheduling.
- Automatic Kevin worker spawn.
- Long-running daemon service supervision.
- Broad cleanup or process management.

## Design Principles

- One owner per feature.
- No fallback, downgrade, or success-wrapped error.
- Error path is a first-class chain and persists events.
- Debug bundles are persisted before output.
- User config stores user choices only; runtime state stays under runtime home.
- Domain names are daemon boundaries; `agent@domain` parsing belongs only to Daemon Domain Registry.
- Shared contracts live in `agentteam-contracts`; owner crates implement behavior.
- `agentteam-runtime` coordinates but does not parse TOML, render output, write state files, or classify errors.

## Implementation Phases

### Phase 0: Gate Tightening

Purpose: make red tests executable enough to block obvious architecture violations before business code grows.

Files:

- `xtask/src/main.rs`
- `docs/red-tests/red-test-plan.md`
- `docs/architecture/function-map.md`
- `docs/architecture/verification-map.md`

Work:

- Add static scans for broad kill patterns.
- Add static scans for hidden tmux/zterm internals in agent-facing skill/docs.
- Add static scans for direct TOML parsing outside Config Center.
- Add static scans for direct state-file writes outside Persistence.
- Add static scans for missing `feature_id` in new contract modules.

Verification:

- `cargo xtask red-tests`
- `cargo xtask verify-function-map`

### Phase 1: Contracts And Error Types

Purpose: create typed chain nodes before owner implementations.

Files:

- `crates/agentteam-contracts/src/lib.rs`
- `crates/agentteam-contracts/src/pipeline/`
- `crates/agentteam-contracts/src/feature_map/`
- `crates/agentteam-contracts/src/verification_map/`

Work:

- Add `ConfigReq*`, `ConfigResp*`, `ConfigErr*`.
- Add `DomainReq*`, `DomainAgentAddr*`, `DomainRoute*`.
- Add `PersistReq*`, `PersistResp*`.
- Add `TeamErr*` MVP chain nodes.
- Add `DebugReq*`, `DebugResp*`.
- Add `ResourceReq*`, `ResourceLease*` MVP types.

Verification:

- unit tests for adjacent conversions only
- compile-fail/static red tests for forbidden non-adjacent conversions where practical

### Phase 2: Config Center

Purpose: make `~/.agentteam/config.toml` schema executable using the example file.

Files:

- `crates/agentteam-config/src/lib.rs`
- `docs/config/config.toml.example`
- `docs/modules/01-config-center.md`

Work:

- Parse TOML only in Config Center.
- Validate project/runtime/tmux/zterm/domain/team/member config.
- Reject runtime-state-looking keys.
- Reject duplicate agent names inside one domain/team scope.
- Reject duplicate daemon domain ids or aliases.
- Redact zterm/remote daemon tokens in snapshots.

CLI:

```text
agentteam config check --config <path> --json
```

Verification:

- config unit tests
- config red tests
- sample config smoke

### Phase 3: Daemon Domain Registry

Purpose: make cross-daemon addressing executable before Communication Center depends on it.

Files:

- `crates/agentteam-runtime/src/lib.rs` or a dedicated runtime submodule
- `crates/agentteam-contracts/src/pipeline/`
- `docs/modules/18-daemon-domain-registry.md`

Work:

- Validate local domain id and aliases from normalized config.
- Register remote daemon endpoint metadata.
- Parse `agent@domain`, `role:<role>@domain`, `team:<team_id>@domain`, `all@domain`.
- Resolve local vs remote route plan.
- Reject remote lookup fallback to local.
- Redact auth tokens in snapshots.

CLI:

```text
agentteam domain resolve --target Alice@review-daemon --config <path> --json
```

Verification:

- domain unit tests
- no Communication Center domain parsing scan
- no zterm/tmux adapter domain parsing scan

### Phase 4: Persistence Event Log

Purpose: make durable append/replay available before Error/Debug claim persistence.

Files:

- `crates/agentteam-persist/src/lib.rs`
- `docs/modules/11-persistence-event-log.md`

Work:

- Define MVP event enum.
- Append typed JSONL records atomically.
- Return append receipts.
- Replay event log from zero.
- Detect corrupt event records.
- Keep runtime state under configured runtime home, not config file.

Verification:

- append/replay unit tests
- corruption red test
- direct state write scan

### Phase 5: Error Center

Purpose: make every framework failure produce persisted classified error with evidence id.

Files:

- `crates/agentteam-error/src/lib.rs`
- `crates/agentteam-persist/src/lib.rs`
- `docs/modules/02-error-center.md`

Work:

- Accept typed fault facts.
- Classify MVP error classes.
- Generate severity and code.
- Generate evidence id.
- Request persistence append.
- Return error projection input for Output Gateway.

Verification:

- no success-wrapped error test
- missing severity/code/evidence red tests
- persisted error event test

### Phase 6: Debug Center And Resource Lifecycle MVP

Purpose: make debug and resource evidence durable before runtime behavior expands.

Files:

- `crates/agentteam-debug/src/lib.rs`
- `crates/agentteam-resource/src/lib.rs`
- `docs/modules/10-debug-center.md`
- `docs/modules/17-resource-lifecycle-manager.md`

Work:

- Register leases for config snapshot, domain snapshot, event log writer, debug bundle.
- Persist debug bundle before returning output.
- Include resource lifecycle snapshot in every debug bundle.
- Detect obvious unreleased MVP handles in tests.

CLI:

```text
agentteam debug snapshot --config <path> --json
```

Verification:

- persisted debug bundle test
- resource lease/release tests
- print-only debug red test

### Phase 7: Input/Output Gateway And CLI

Purpose: expose implemented runtime slice through stable CLI without leaking module internals.

Files:

- `crates/agentteam-gateway/src/lib.rs`
- `crates/agentteam-cli/src/main.rs`
- `crates/agentteamd/src/main.rs`
- `docs/modules/04-input-gateway.md`
- `docs/modules/05-output-gateway.md`
- `docs/modules/13-cli-agent-skill.md`

Work:

- Parse CLI raw commands through Input Gateway.
- Render text/JSON/error through Output Gateway.
- Add `config check`, `domain resolve`, `debug snapshot`, `daemon check`.
- Preserve payload semantics.
- Never expose tokens, tmux pane ids, zterm internals, or private module state.

Verification:

- CLI smoke tests
- JSON shape tests
- redaction tests

### Phase 8: Task/Comm/TANote MVP

Purpose: add team collaboration logic after durable/error/debug foundations exist.

Files:

- `crates/agentteam-comm/src/lib.rs`
- `crates/agentteam-runtime/src/lib.rs`
- `crates/agentteam-tanote/src/lib.rs`
- `docs/modules/03-communication-center.md`
- `docs/modules/09-task-engine.md`
- `docs/modules/16-tanote-collaboration-board.md`

Work:

- Task create/list/status/done/error in memory plus event log.
- Communication route envelopes through domain route plans.
- Broadcast and all target.
- TANote ordered projection and note thread query.
- No tmux delivery yet; use durable envelope projection.

Verification:

- assigned-first and blocked-first claim tests
- task failure vs framework failure tests
- TANote projection tests
- Communication Center does not own task priority test

### Phase 9: Startup And zterm/tmux Adapter Skeleton

Purpose: prepare terminal integration without launching real TUI processes yet.

Files:

- `crates/agentteam-startup/src/lib.rs`
- `crates/agentteam-tmux/src/lib.rs`
- `crates/agentteam-tui-adapter/src/lib.rs`
- `docs/modules/12-zterm-tmux-adapter.md`
- `docs/modules/14-tui-agent-adapter-center.md`
- `docs/modules/15-startup-session-manager.md`

Work:

- Build typed launch/input/output envelopes.
- Validate TA session names using domain id + project slug + agent name.
- Produce dry-run adapter commands.
- Return explicit unavailable transport error for real execution until enabled.

Verification:

- no direct tmux call scan
- no adapter-owned task/status truth scan
- dry-run launch envelope tests

## File Strategy

Keep Rust files under 500 lines by splitting owner modules early:

```text
crates/agentteam-contracts/src/
  config/
  domain/
  error/
  persist/
  debug/
  resource/

crates/agentteam-config/src/
  lib.rs
  load.rs
  validate.rs
  normalize.rs
  snapshot.rs

crates/agentteam-runtime/src/
  lib.rs
  domain/
  orchestrator/
  task/

crates/agentteam-cli/src/
  main.rs
  commands/
```

Use `lib.rs` and `mod.rs` as thin composition files.

## Risk And Mitigation

| Risk | Mitigation |
|---|---|
| Too many modules implemented at once | Use vertical phases and gates after every phase |
| Duplicate owner logic | Gate with function map and static scans |
| Error path added late | Implement Error Center before CLI/runtime grows |
| Debug not durable | Implement Persistence before Debug Center |
| Domain logic leaks into Comm/Adapter | Implement Daemon Domain Registry before Comm |
| File size grows past 500 lines | Split by owner/domain/node immediately |
| Tests become only smoke tests | Add red tests per forbidden behavior before implementation |

## Validation Matrix

| Phase | Required validation |
|---|---|
| 0 | `cargo xtask red-tests`, `cargo xtask verify-function-map` |
| 1 | `cargo test --workspace`, adjacent conversion tests |
| 2 | config unit tests, sample config smoke |
| 3 | domain route tests, no-domain-parse red scans |
| 4 | append/replay/corruption tests |
| 5 | classified/persisted error tests |
| 6 | debug persistence/resource lease tests |
| 7 | CLI JSON/text/error smoke tests |
| 8 | task/comm/TANote unit and red tests |
| 9 | dry-run adapter/startup tests |

Every phase also runs:

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

## Done Definition

The first complete development execution is done when:

- Config/domain/persistence/error/debug/resource CLI slice works locally.
- Events and debug bundles are persisted under runtime home for the sample config.
- Error and debug responses include ids and receipts.
- Cross-daemon target parsing returns explicit route plans.
- No real tmux/zterm command is executed yet.
- All required gates pass.
- `note.md`, `MEMORY.md`, and `CACHE.md` are updated with verified facts.

## Recommended First Command

Start with Phase 0 and Phase 1 only:

```text
Implement Phase 0 and Phase 1 from docs/goals/mvp-runtime-vertical-slice-plan.md.
```

Do not start Phase 2 until Phase 0 and Phase 1 pass the full gate.
