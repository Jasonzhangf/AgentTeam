# Function Map

This map is the feature owner truth. If a feature is not listed here, implementation must not begin.

The hard gate is `cargo xtask verify-function-map`.

| feature_id | Owner module | Canonical contracts | Allowed paths | Forbidden paths | Required gates |
|---|---|---|---|---|---|
| `config.center` | Config Center | `ConfigReq*`, `ConfigResp*`, `ConfigErr*` | `agentteam-config`, `docs/modules/01-*`, `docs/config/config.toml.example` | runtime, gateway, CLI parsing TOML directly; runtime/task/debug state inside user config | config unit, config red tests |
| `error.center` | Error Center | `TeamErr*` | `agentteam-error`, contracts error chain | all modules rendering final error text | error unit, success-wrapped-error red test |
| `comm.center` | Communication Center | `CommReq*`, `CommResp*` | `agentteam-comm` | tmux adapter owning task/message semantics | routing unit, duplicate-owner red test |
| `domain.registry` | Daemon Domain Registry | `DomainReq*`, `DomainAgentAddr*`, `DomainRoute*` | `agentteam-runtime`, contracts domain chain, `docs/modules/18-*` | Communication Center parsing domain addresses; Agent Registry treating local names as globally unique; zterm adapter resolving business target domains | domain unit, cross-daemon routing red tests |
| `gateway.input` | Input Gateway | `TeamReq*` | `agentteam-gateway` | runtime/UI parsing raw CLI/API/UI payload directly | parser/validator tests |
| `gateway.output` | Output Gateway | `TeamResp*` | `agentteam-gateway` | modules/UI formatting final CLI/UI text directly or exposing private state | projection tests |
| `gateway.ui` | UI Gateway | `UiReq*`, `UiResp*` | `agentteam-gateway`, future web UI | UI/WebUI mutating runtime state directly or calling framework internals | UI contract tests |
| `agent.naming_pool` | Agent Registry | `AgentName*`, `AgentMember*`, `AgentStatus*` | `agentteam-runtime`, contracts | tmux adapter inventing names; Codex SDK as generic status truth | naming/status red tests |
| `team.orchestration` | Team Orchestrator | `TeamCommand*` | `agentteam-runtime` | gateways deciding execution order | orchestration tests |
| `task.engine` | Task Engine | `TaskEvent*`, `TaskState*` | `agentteam-runtime` | comm center mutating queue internals | queue tests |
| `debug.center` | Debug Center | `DebugSnapshot*` | `agentteam-debug` | debug center reading private fields | snapshot tests |
| `persist.event_log` | Persistence | `PersistEvent*` | `agentteam-persist` | modules writing state files directly | replay tests |
| `adapter.zterm_tmux` | zterm/tmux Adapter | `TerminalReq*`, `TerminalResp*` | `agentteam-tmux` | runtime shelling out to tmux directly; adapter owning task/status truth | adapter contract tests |
| `adapter.tui_agent` | TUI Agent Adapter Center | `TuiSignalReq*`, `TuiSignalResp*` | `agentteam-tui-adapter` | stdout-only final status; Codex SDK as universal truth; provider payload in runtime business state | tui adapter red tests |
| `startup.session` | Startup Session Manager | `StartupReq*`, `StartupOp*` | `agentteam-startup` | Kevin as persistence truth; direct tmux execution; direct state file write | startup/session red tests |
| `tanote.board` | TANote Collaboration Board | `TANoteReq*`, `TANoteEvent*`, `TANoteProjection*` | `agentteam-tanote`, `docs/tanote/TANote.md.example` | Task Engine/Comm/agents directly mutating TANote format or treating notes as task truth | TANote format/order/thread red tests |
| `resource.lifecycle` | Resource Lifecycle Manager | `ResourceReq*`, `ResourceLease*`, `ResourceMetric*`, `ResourceLeak*` | `agentteam-resource`, contracts resource chain | modules creating long-lived resources without leases; broad cleanup; orphan/leak/growth hidden from event log/debug | lifecycle/leak/orphan/growth red tests |
| `cli.agent_skill` | CLI/Skill | `CliCommand*` | `agentteam-cli`, `.agents/skills` | skill depending on hidden wire protocol; agent-facing tmux/session internals; Kevin missing framework-operation guidance | CLI smoke tests |

## Owner Rule

Each row has one owner module. Shared helpers must be moved into `agentteam-contracts` or explicit block modules, not duplicated in owner crates.

## Discussion Items

- Confirm whether `agentteam-runtime` owns both team orchestration and task engine, or whether task engine becomes its own crate.
- Confirm whether UI Gateway includes terminal render surface in v1 or only returns render attachment metadata.
