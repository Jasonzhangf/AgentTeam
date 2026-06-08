# 15 Startup And Session Manager

## Purpose

Startup and Session Manager owns local project/session bootstrap, daemon startup coordination, Kevin initialization, and managed TUI agent launch orchestration.

Kevin can act as the human-facing bootstrap/operator agent, but Kevin is not persistence truth. Daemon + Persistence remain the durable truth for project/session/task/message/event state.

## Owns

- Local project bootstrap flow.
- Project-scoped session directory initialization.
- Kevin-first initialization flow.
- Worker spawn plan generation.
- Managed tmux launch command envelopes.
- Startup event requests to Persistence.
- Startup/session debug snapshot.
- Agent input/output operation envelope definitions.
- Internal session/tmux detail encapsulation.

## Does Not Own

- Raw config parsing.
- Error classification.
- Direct tmux execution.
- Task state.
- Message routing.
- Provider status extraction.
- WebUI state.
- Event log append implementation.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `startup.init_project` | Startup Session Manager | Initialize project runtime/session directories from Config Center output | `ConfigResp05RuntimeConfig` | startup plan | path outside home |
| `startup.ensure_daemon` | Startup Session Manager | Ensure daemon lifecycle command is issued through scoped service/PID path | startup request | daemon command envelope | broad kill/start bypass |
| `startup.init_kevin` | Startup Session Manager | Initialize fixed manager agent Kevin in current TUI/session | team config | Kevin session descriptor | manager not Kevin |
| `startup.spawn_workers` | Startup Session Manager | Build worker launch envelopes for zterm/tmux Adapter | team config + name pool | launch envelope list | direct tmux call |
| `startup.close_session` | Startup Session Manager + Resource Lifecycle Manager | Close project session and release scoped resources | project/team scope | shutdown plan/result | orphan resources left hidden |
| `startup.cleanup_temp` | Startup Session Manager + Resource Lifecycle Manager | Remove runtime temporary files through exact tracked handles | scoped temp resources | cleanup receipts | broad file deletion |
| `startup.write_session_descriptor` | Startup Session Manager + Persistence | Request session descriptor persistence | launch/session facts | persistence event | direct file write |
| `startup.build_agent_input_op` | Startup Session Manager | Build typed operation for agent stdin/task/message injection | operation intent | input op envelope | raw string bypass |
| `startup.build_agent_output_op` | Startup Session Manager | Build typed operation for stdout/render/read observation request | observation intent | output op envelope | direct stdout ownership |
| `startup.snapshot` | Startup Session Manager | Provide startup/session snapshot to Debug Center | startup state | startup snapshot | private leak |
| `startup.help` | Startup Session Manager | Describe bootstrap and session flows | help topic | help model | hidden tmux command requirement |

## Module Help Contract

Required help topics:

```text
agentteam help startup
agentteam help startup kevin
agentteam help startup workers
agentteam help startup sessions
agentteam help startup shutdown
agentteam help startup ops
agentteam help startup red-tests
```

Help content must explain:

- bootstrap starts from Kevin
- Kevin knows it is Kevin through skill/context injection
- workers know their assigned names through skill/context injection
- Kevin uses CLI/framework tools for task board, task publish, query, and worker management
- Startup Manager builds typed launch/input/output operation envelopes
- zterm/tmux Adapter executes terminal transport operations
- Persistence stores events/session descriptors
- daemon/session close runs scoped resource cleanup and temporary file cleanup
- agents use names/roles/tasks/messages only; tmux/session details are hidden

Help content must not:

- say Kevin is persistence truth
- tell Kevin to write state files directly
- tell Kevin to call tmux directly
- expose hidden daemon wire protocol as required agent behavior
- expose tmux session names, pane ids, zterm endpoints, or session descriptor paths to agents
- suggest broad process kill commands

## Public API Boundary

```text
StartupReq01BootstrapIntent -> StartupReq02ValidatedPlan -> StartupReq03LaunchEnvelope -> StartupResp04SessionProjection
StartupReq01ShutdownIntent -> StartupReq02ValidatedPlan -> StartupReq03ShutdownEnvelope -> StartupResp04ShutdownProjection
StartupOp01AgentInputIntent -> StartupOp02InputEnvelope -> StartupOp03AdapterCommand
StartupOp01AgentOutputIntent -> StartupOp02OutputEnvelope -> StartupOp03ObservationRequest
```

Only Startup Manager builds bootstrap/session plans.

Only zterm/tmux Adapter executes tmux launch/input/output transport.

Only Persistence appends durable startup/session events.

Only Resource Lifecycle Manager records release/orphan/leak/cleanup state.

## Local Directory Contract

User config:

```text
~/.agentteam/config.toml
```

Project session metadata:

```text
~/.agentteam/sessions/<project_slug>/
```

Suggested child paths:

```text
~/.agentteam/sessions/<project_slug>/session.json
~/.agentteam/sessions/<project_slug>/agents/
~/.agentteam/sessions/<project_slug>/launch/
~/.agentteam/sessions/<project_slug>/evidence/
```

Durable events remain owned by Persistence. Session descriptors are materialized projections/recoverability aids, not the only truth.

Session and tmux identifiers are framework internals. Agents never need these paths or ids.

Temporary runtime files:

```text
~/.agentteam/tmp/<project_slug>/<run_id>/
```

Temporary files may include staging buffers, render captures, debug assembly scratch files, adapter observations, and projection work files. They must be registered as scoped resources and removed during scoped daemon/session close after persisted receipts exist.

## Bootstrap Flow

```text
Human current TUI
  |
  | agentteam startup init
  v
Startup Manager
  |
  +--> Config Center: load user config
  +--> Agent Registry: allocate Kevin + workers
  +--> Persistence: append ProjectBootstrapRequested
  +--> zterm/tmux Adapter: ensure Kevin TA session/current session binding
  +--> CLI/Skill: inject Kevin identity/tool usage guidance
  +--> Persistence: append KevinReady/SessionDescriptor events
  +--> zterm/tmux Adapter: spawn worker TA sessions with initial commands
  +--> CLI/Skill: inject worker identity/tool usage guidance
  +--> TUI Agent Adapter Center: observe provider status signals
  +--> Agent Registry/Runtime: project statuses
```

## Kevin Role

Kevin is the single super manager in v1.

Kevin can:

- read the AgentTeam skill to learn framework operations
- initialize the framework through `agentteam startup init`
- query task board through CLI
- publish tasks through CLI
- assign task owners through CLI
- broadcast messages through CLI
- communicate with child agents through CLI messages/tasks
- wait for child agent results through task status/projections
- inspect agents through CLI
- request debug snapshots through CLI
- request worker spawn/restart through daemon command envelopes

Kevin cannot:

- mutate event log directly
- write session descriptor files directly
- call tmux directly
- depend on tmux/session ids or paths
- bypass Input Gateway/Communication Center/Task Engine
- become persistence truth

## Kevin Skill Contract

Kevin must receive/read skill guidance that teaches:

- own identity: `Kevin`
- role: single super manager
- framework initialization command
- task-board query command
- task publish/assign command
- message/broadcast command
- worker status query command
- wait-for-result loop through task status/projections
- debug snapshot command
- rule that tmux/session details are hidden internals

Kevin waits for results by querying task/message/status projections, not by reading child tmux/session internals.

## Worker Startup Flow

Workers are launched in managed TA tmux sessions.

Worker initial command must inject:

- agent name
- work role
- team role `worker`
- project slug
- team id
- CLI usage/help reference
- task claim/check/update/done/error instructions

Worker ready rule:

- launch returns no framework/transport error
- TA session exists
- optional provider adapter signal may add `ready_hint`
- Runtime/Agent Registry projects `idle` when no active task exists

## Session Shutdown Flow

`agentteam daemon stop --pid <pid>` or future `agentteam session close --project <project_slug>` closes a project session through scoped shutdown.

```text
Shutdown request
  |
  v
Startup Session Manager
  |
  +--> Persistence: append ShutdownRequested
  +--> Input Gateway/Runtime: stop accepting new work for this scope
  +--> Debug Center: persist final debug bundle
  +--> Resource Lifecycle Manager: scan active scoped leases
  +--> zterm/tmux Adapter: close managed terminal resources by exact handles
  +--> TUI Agent Adapter Center: release provider adapters
  +--> UI Gateway: release subscriptions
  +--> TANote/Output/Debug: release projection cursors and temp files
  +--> Persistence: flush and append ShutdownCompleted or ShutdownFailed
```

Rules:

- Shutdown cleanup is automatic for resources owned by the closing daemon/session scope.
- Cleanup must use exact resource ids, leases, pids, session handles, and temp-file handles.
- No broad process kill or broad file deletion is allowed.
- Persistent artifacts are not temporary cleanup targets: event log, debug bundles, evidence, TANote projection, and session descriptors remain until retention policy says otherwise.
- Temporary files are removed only after their durable event/debug/evidence materialization receipts exist.
- If cleanup fails, shutdown is `error` with persisted evidence; it is not reported as success.

## Agent Input/Output Operations

Agent input and output must be typed operations, not arbitrary hidden string hacks.

Input operation examples:

```text
AgentInputOp::InjectTaskInstruction
AgentInputOp::InjectMessage
AgentInputOp::InjectSkillReminder
AgentInputOp::RequestStatusReport
AgentInputOp::SendRawKeys
```

Output operation examples:

```text
AgentOutputOp::ReadRecentBuffer
AgentOutputOp::ReadStatusSignal
AgentOutputOp::CaptureRenderFrame
AgentOutputOp::CollectEvidence
```

Rules:

- Input op goes through Input Gateway then zterm/tmux Adapter.
- Output op goes through zterm/tmux Adapter then TUI Agent Adapter Center when status semantics are needed.
- Output Gateway projects observations for CLI/UI/WebUI.
- No operation may crop or rewrite semantic payload.

## Persistence Behavior

Startup Manager requests Persistence events for:

- project bootstrap requested
- daemon startup requested/result
- Kevin init requested/result
- worker spawn requested/result
- session descriptor materialized
- agent input op requested/result
- agent output op requested/result

All errors go through Error Center and are persisted as error events.

## Error Behavior

Startup failures emit Startup fault facts to Error Center.

Examples:

- config load failure
- session dir invalid
- Kevin init failure
- worker spawn envelope failure
- adapter launch failure
- session descriptor materialization failure

No fallback to unmanaged tmux session.

## Debug Snapshot

Snapshot includes:

- bootstrap phase
- project slug
- session directory
- Kevin session descriptor
- worker spawn plan
- latest launch receipts
- latest input/output operation receipts
- blocked startup reason

## Resource Lifecycle

Startup Session Manager owns lifecycle requests for:

- `daemon_process`
- startup command handle
- session descriptor projection
- launch plan handle
- shutdown plan handle
- temporary runtime file handles

Rules:

- Register daemon process resource when a scoped daemon start request is accepted.
- Register launch plan handle before worker spawn orchestration begins.
- Release launch plan after all worker launch results are persisted as success or classified failure.
- Register shutdown plan handle before scoped daemon/session close begins.
- Release shutdown plan only after all scoped release/cleanup events are persisted.
- Temporary file handles must be released and removed during scoped shutdown.
- Session descriptor projections must have event receipts and must not become the only truth.
- A daemon process without matching pid/service-scoped owner record is an orphan candidate.
- Startup phase duration, shutdown duration, pending launch count, pending cleanup count, temporary file count, and descriptor projection size are efficiency metrics.

## Red Tests

- Kevin treated as persistence truth fails.
- Startup Manager calling tmux directly fails.
- Session metadata outside `~/.agentteam/sessions/<project_slug>/` fails.
- Worker launch without TA session name fails.
- Worker identity injection missing name/role/team fails.
- Kevin skill missing init/query/task/message/wait guidance fails.
- Agent-facing docs exposing tmux/session identifiers fail.
- Agent input raw string bypassing typed operation fails.
- Agent output direct stdout ownership by Startup Manager fails.
- Startup event not persisted fails.
- Broad kill/start command fails.
- Startup launch plan without lifecycle lease fails.
- Daemon process orphan hidden from debug fails.
- Scoped shutdown leaving tracked temp files without cleanup result fails.
- Shutdown success without final persisted debug bundle fails.

## Open Decisions

- Whether `agentteam startup init` binds current TUI as Kevin or always creates a new Kevin tmux session.
- Exact identity/skill injection text shape.
- Exact session descriptor JSON schema.
