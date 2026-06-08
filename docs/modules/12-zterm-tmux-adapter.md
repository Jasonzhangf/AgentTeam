# 12 zterm/tmux Adapter

## Purpose

The adapter is the only module that talks to tmux and zterm-compatible terminal bridge APIs.

## Owns

- Managed tmux session operations.
- zterm bridge connection.
- Multiple zterm daemon endpoint connections after domain resolution.
- Terminal input send.
- Buffer head/sync requests.
- Single agent render attachment.
- Terminal transport errors as fault facts.
- TUI launch result observation.
- TA session existence observation.
- stdout/buffer observation from tmux/zterm.

## Does Not Own

- Team/task/message semantics.
- Agent naming allocation.
- Error classification.
- Config parsing.
- Output rendering.
- Generic agent status ownership.
- Provider SDK status ownership.
- Agent-facing API exposure.
- Daemon domain target resolution.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `tmux.launch_session` | zterm/tmux Adapter | Launch managed TA tmux session | `TerminalReq01AdapterCommand` | `TerminalResp03TransportEvent` | non-TA session |
| `tmux.send_input` | zterm/tmux Adapter | Send stdin to managed TA agent pane | `TerminalReq02TransportRequest` | transport event | runtime direct tmux |
| `tmux.connect_domain_endpoint` | zterm/tmux Adapter + Daemon Domain Registry | Connect to resolved zterm daemon endpoint | `DomainRoute04Plan` | transport connection | adapter resolves domain |
| `tmux.observe_output` | zterm/tmux Adapter | Read stdout/buffer evidence through tmux/zterm | transport request | `TerminalResp04Observation` | payload crop |
| `tmux.render_agent` | zterm/tmux Adapter | Attach/render one selected managed agent | render command | render observation | agent internals exposed |
| `tmux.list_managed` | zterm/tmux Adapter + Agent Registry | List project-scoped managed TA sessions | project scope | managed session list | non-TA group op |
| `tmux.classify_transport_fault` | zterm/tmux Adapter + Error Center | Emit transport/session fault fact | transport error | fault fact | swallowed transport |
| `tmux.snapshot` | zterm/tmux Adapter | Provide transport/session snapshot to Debug Center | adapter state | adapter snapshot | hidden leak |
| `tmux.help` | zterm/tmux Adapter | Describe transport boundary and hidden internals | help topic | help model | agent-facing tmux ids |

## Module Help Contract

Required help topics:

```text
agentteam help tmux
agentteam help tmux launch
agentteam help tmux input
agentteam help tmux output
agentteam help tmux render
agentteam help tmux domains
agentteam help tmux loopback
agentteam help tmux red-tests
```

Help content must explain:

- zterm/tmux Adapter is the only tmux/zterm transport owner
- multiple zterm daemons are addressed only through resolved daemon domain endpoint facts
- only managed TA sessions are in scope
- stdout/buffer output is evidence, not final task/status truth
- loopback smoke proves multiple managed TA sessions can exchange input/output before higher-level task or communication work depends on the adapter
- tmux session names, pane ids, and zterm internals are hidden from agents
- cleanup uses exact tracked handles only

Help content must not:

- teach agents to call tmux directly
- expose tmux pane/session ids as agent-facing API
- classify stdout as final task success or final agent status
- resolve `agent@domain` business targets
- suggest broad process kill or broad session cleanup

## Public API Boundary

```text
TerminalReq01AdapterCommand -> TerminalReq02TransportRequest -> TerminalResp03TransportEvent -> TerminalResp04Observation
DomainRoute04Plan -> TerminalReq02TransportRequest
```

Runtime must not call tmux directly.

Agents must not call or see this adapter API directly. Agent-facing surfaces use CLI/skill commands that refer to agent names, roles, tasks, and messages.

Daemon Domain Registry resolves daemon endpoints before adapter use.

## Required Behavior

- Create/start managed TA tmux sessions.
- Send stdin to target session.
- Connect to the zterm daemon endpoint selected by a resolved daemon domain route plan.
- Request terminal output/mirror state.
- Observe stdout/buffer through tmux/zterm bridge for evidence and diagnostics.
- Detect framework-level launch/session/transport failure.
- List scoped managed sessions.
- Render one selected agent.
- Reject non-managed sessions.
- Surface zterm/tmux transport failures explicitly.
- Keep tmux session names, pane ids, and bridge internals out of agent-facing projections unless Debug Center explicitly redacts/authorizes debug output for a human operator.
- Never parse `agent@domain` business targets.

## Status Observation Rule

The adapter provides generic runtime facts:

- launch accepted/failed
- TA tmux session exists/missing
- transport connected/disconnected/error
- stdout/buffer observation
- pane/process termination when available

The adapter does not decide final agent status alone.

Agent Registry/Runtime projects status with Task Engine and Error Center facts.

stdout/buffer observation may identify visible TUI error text as evidence. It is not the only status truth.

Provider-specific status logic belongs in TUI Agent Adapter Center, not this adapter.

## Error Behavior

Transport and session faults emit Transport fault facts to Error Center.

No fallback to direct tmux if zterm bridge mode is required by current command.

## Debug Snapshot

Snapshot includes:

- zterm endpoint
- resolved daemon domain endpoints with tokens redacted
- active terminal connections
- managed tmux sessions
- latest stdout/buffer observation summary
- last buffer head per agent
- transport error counts

## Resource Lifecycle

zterm/tmux Adapter owns lifecycle requests for:

- `tmux_session`
- `zterm_connection`
- remote zterm daemon connection
- `tui_process`
- terminal buffer observation handle
- render attachment handle

Rules:

- Register managed tmux session resource before launch command is executed.
- Register zterm connection resource before connection use.
- Register a remote zterm daemon connection only after Daemon Domain Registry provides a route plan.
- Release render attachment when render command/session ends.
- Release zterm connection when no active render/input/output operation needs it.
- A managed tmux session without Agent Registry member lease is an orphan candidate.
- A zterm connection without active agent/render operation is a leak candidate.
- A remote daemon connection without a domain route handle is a leak candidate.
- Active session count, active connection count, buffer snapshot bytes, and transport error count are efficiency metrics.
- Cleanup must use exact resource ids/session handles only; broad process kill is forbidden.

## Red Tests

- Runtime direct tmux shell call fails architecture gate.
- zterm/tmux Adapter resolving `agent@domain` directly fails architecture gate.
- Remote daemon token leak in snapshot fails.
- Non-TA session operation fails.
- Agent-facing output exposing tmux/session internals fails.
- zterm unavailable fails explicitly.
- SDK-only status detection fails.
- stdout evidence classified as final task/framework state by adapter fails.
- Output payload crop/rewrite fails.
- Adapter modifying task state fails architecture gate.
- tmux/zterm resource without lifecycle lease fails.
- zterm connection leak hidden from debug fails.
- broad adapter cleanup fails.

## Open Decisions

- Whether v1 launches zterm daemon or requires an already-running endpoint.
- Exact render UX: attach tmux, proxy zterm render, or future UI.
