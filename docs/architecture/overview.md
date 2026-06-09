# AgentTeam Architecture Overview

## Goal

AgentTeam is a Rust runtime framework for multiple TUI agents that run in tmux and cooperate as a team.

It must provide:

- cross-platform agent management through a common daemon/CLI contract
- tmux-backed TUI agent stdin/stdout communication
- zterm-compatible terminal mirror/render integration
- provider-adapted TUI agent status signals
- single-agent control mode selection for attach_tui and headless operation
- local session bootstrap and project startup management
- startup parameter injection for agent name, role, team, and project scope
- role-based task orchestration
- explicit inter-agent messaging
- cross-daemon agent addressing through daemon domains
- shared agent discussion notes through `TANote.md`
- durable event/state truth
- isolated module APIs
- independent error, config, debug, communication, UI, input, and output centers

## System Truth Split

AgentTeam owns:

- team registry
- daemon domain registry
- agent naming pool
- single-agent control plane
- role/member lifecycle
- task queue
- message bus semantics
- config normalization
- error governance
- debug snapshots
- persistence/event log
- CLI/API/UI gateway contracts
- WebUI input/output projection boundary
- provider-specific TUI status signal normalization
- project/session bootstrap coordination
- TANote forum-style collaboration note format and append ordering
- resource lifecycle registry, leak/orphan detection, and growth-control visibility

Daemon Domain Registry owns:

- local daemon domain identity
- remote daemon domain registry
- domain-qualified agent addressing
- cross-daemon route resolution

zterm/tmux owns:

- physical terminal session
- TUI process stdin/stdout transport
- tmux pane truth
- terminal mirror/render buffer

TUI Agent Adapter Center owns:

- provider-specific status signal extraction
- normalized TUI agent status signals
- Codex/generic/future TUI adapter boundary

Agent Control Center owns:

- single-agent control mode selection
- attach_tui tmux binding
- headless SDK-style binding
- pause/stop/wait/retry control
- per-agent input/output routing

No AgentTeam module may copy zterm daemon logic. AgentTeam may depend on a zterm-compatible adapter contract.

## Rust Workspace Direction

Future crate layout:

- `agentteam-contracts`: shared types, pipeline nodes, API contracts, red-test helpers
- `agentteam-config`: Config Center
- `agentteam-error`: Error Center
- `agentteam-comm`: Communication Center
- `agentteam-debug`: Debug Center
- `agentteam-persist`: event log and snapshots
- `agentteam-runtime`: pure orchestration daemon core
- `agentteam-startup`: startup and local session manager
- `agentteam-tanote`: TANote collaboration board
- `agentteam-resource`: Resource Lifecycle Manager
- `agentteam-control`: Agent Control Center
- `agentteam-tmux`: tmux/zterm adapter
- `agentteam-tui-adapter`: TUI agent provider adapter center
- `agentteam-gateway`: input/output/UI gateway
- `agentteam-cli`: CLI binary
- `agentteamd`: daemon binary

## Architecture Rules

- Orchestrator only coordinates; it does not parse config, format UI, classify errors, or speak transport wire directly.
- Gateways translate external input/output only; they do not own business state.
- UI/WebUI consumes Input Gateway and Output Gateway contracts only; it is decoupled from agent framework internals.
- Communication Center routes messages and task dispatches; it does not launch processes.
- Daemon Domain Registry resolves `agent@domain` targets and daemon route plans; Communication Center must not parse domain addresses itself.
- Agent Registry allocates names inside a daemon domain/team scope; local agent names are not globally unique.
- TANote Collaboration Board owns `TANote.md` append format, thread projection, and note-id ordering; Communication Center only routes note/message envelopes.
- Resource Lifecycle Manager owns lease/handle registry, resource snapshots, orphan detection, cleanup decisions, and growth-control reporting. Each resource still has one business owner module.
- Error Center classifies every failure; other modules only create typed error facts.
- Debug Center reads snapshots through module-provided snapshot APIs; it does not access private state.
- Config Center is the only TOML parser and config normalizer.
- Persistence owns event append/replay; no module writes state files directly.
- TUI Agent Adapter Center extracts provider signals only; it does not project final lifecycle status.
- Agent Control Center binds one agent at a time and owns attach_tui/headless control mode selection; it does not own task truth or name allocation.
- Startup Manager coordinates bootstrap/session plans only; it does not persist files directly or execute tmux directly. Standard tmux is the transparent operator carrier for the configured manager shell, but tmux/session details remain hidden from agents.

## Main Chains

Request chain:

`CliRaw -> ParsedCommand -> ValidatedIntent -> DaemonCommand -> ModuleCommand -> Event -> Projection -> CliRendered`

Error chain:

`FaultFact -> ErrorClassified -> ErrorEvent -> ErrorProjection -> CliRenderedError`

Debug chain:

`SnapshotRequest -> ModuleSnapshot -> DebugBundle -> DebugProjection`

Terminal/status chain:

`TaskDispatch -> InputGateway -> AgentControlCenter -> TmuxAdapter/SDKAdapter -> zterm/tmux or SDK -> TuiAgentAdapter -> AgentRegistry/Runtime -> OutputGateway`

Single-agent control chain:

`ControlIntent -> InputGateway -> AgentControlCenter -> SessionBinding -> Input/Output/Wait/Pause/Stop -> Adapter -> Observation -> OutputGateway`

TANote/forum chain:

`NotePost -> TANoteBoard -> PersistenceEvent -> TANoteProjection -> TmuxEnvelope/InputGateway -> AgentVisibleMessage`

Resource lifecycle chain:

`ResourceAcquire -> LeaseRegistered -> OwnerUse -> Heartbeat/Metric -> ReleaseRequested -> ReleaseConfirmed -> Leak/Orphan/GrowthProjection`

Daemon domain route chain:

`RawTarget -> ValidatedDomainTarget -> ResolvedAgentAddress -> DomainRoutePlan -> LocalOrRemoteDelivery`

## Discussion Status

This document is a starting contract. Each module doc must be reviewed and refined before Rust code is written.
