# 14 TUI Agent Adapter Center

## Purpose

TUI Agent Adapter Center owns provider-specific status signal extraction for TUI agents.

tmux/zterm gives terminal transport and stdout/buffer evidence. That is too coarse for robust status detection. TUI Agent Adapter Center adds provider adapters for common TUI agents while keeping one generic AgentTeam status model.

## Owns

- TUI agent provider registry.
- Provider-specific status signal extraction.
- Generic shell/TUI adapter.
- Codex-specific diagnostic adapter.
- Future Claude/Gemini/other TUI adapters.
- Agent status signal normalization.
- Provider adapter debug snapshot.
- Provider adapter help.
- It does not select attach_tui/headless control mode.

## Does Not Own

- tmux/zterm transport.
- Terminal render buffer.
- Final agent lifecycle status projection.
- Task state.
- Error classification.
- Config parsing.
- Communication routing.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `tui_adapter.select_provider` | TUI Agent Adapter Center | Select adapter by configured provider and command metadata | agent config + launch metadata | provider adapter id | unknown provider behavior |
| `tui_adapter.extract_status_signal` | TUI Agent Adapter Center | Extract provider status signal from terminal/provider facts | stdout/buffer + provider facts | normalized status signal | stdout-only final status |
| `tui_adapter.codex_signal` | TUI Agent Adapter Center | Extract Codex-specific diagnostic signal when available | Codex session facts | normalized status signal | Codex-only generic status |
| `tui_adapter.generic_signal` | TUI Agent Adapter Center | Extract generic TUI/shell signal without provider SDK | tmux/zterm facts | normalized status signal | shell agent unsupported |
| `tui_adapter.normalize_signal` | TUI Agent Adapter Center | Normalize provider signal to generic signal enum | provider signal | `TuiAgentSignal` | provider leak into runtime |
| `tui_adapter.snapshot` | TUI Agent Adapter Center | Provide adapter snapshot to Debug Center | adapter state | adapter snapshot | secret leak |
| `tui_adapter.help` | TUI Agent Adapter Center | Describe provider/status adapter behavior | help topic | help model | hidden SDK requirement |

## Module Help Contract

Required help topics:

```text
agentteam help tui-adapter
agentteam help tui-adapter providers
agentteam help tui-adapter status
agentteam help tui-adapter codex
agentteam help tui-adapter generic
agentteam help tui-adapter red-tests
```

Help content must explain:

- tmux stdout alone is evidence, not complete status truth
- provider adapters add richer status signals
- generic TUI agents must still work without provider SDK
- Codex SDK is optional provider-specific diagnostics
- final lifecycle status is projected by Agent Registry/Runtime, not provider adapter
- provider signals must be normalized before crossing module boundary

Help content must not:

- require Codex SDK for all agents
- classify stdout text as final status
- mutate Task Engine state
- route messages or claims
- own the single-agent control plane

## Public API Boundary

```text
TuiSignalReq01AdapterInput -> TuiSignalReq02ProviderSignal -> TuiSignalResp03NormalizedSignal
```

Only TUI Agent Adapter Center can convert provider-specific observations into normalized TUI agent signals.

Only Agent Registry/Runtime can combine normalized TUI signals with Task Engine and Error Center facts into final lifecycle status.

## Normalized Signals

```text
launched
ready_hint
idle_hint
busy_hint
waiting_input_hint
error_hint
exited
unknown
```

Rules:

- signals are hints/facts, not final status
- `busy_hint` covers active work and outstanding request/response pending states
- `waiting_input_hint` is a busy subcase where the agent is alive but waiting for a caller reply
- `error_hint` must include evidence id or evidence source reference
- `unknown` is explicit and must not be treated as success
- provider-specific fields stay in debug/evidence, not business payload

## Provider Adapter Types

Generic adapter:

- uses tmux/zterm launch/session/buffer facts
- checks process/session existence
- can detect obvious process exit or transport failure
- can extract configured prompt/error markers if present

Codex adapter:

- may use Codex-specific SDK/session diagnostics if available
- may parse Codex-specific TUI markers
- cannot become global status truth
- cannot be required for non-Codex agents

Future adapters:

- Claude TUI
- Gemini TUI
- custom shell agent
- user-defined marker adapter

## Status Projection Relationship

```text
tmux/zterm Adapter
  -> terminal/session facts
  -> TUI Agent Adapter Center
  -> normalized TUI signals
  -> Agent Registry/Runtime
  -> final lifecycle status

Task Engine
  -> active task facts
  -> Agent Registry/Runtime

Error Center
  -> framework fault facts
  -> Agent Registry/Runtime
```

Final status remains:

```text
offline
starting
idle
busy
error
```

## Error Behavior

Provider adapter failures emit TuiAdapter fault facts to Error Center.

Provider adapter unavailable must not break generic TUI support unless the config explicitly requires that provider adapter.

No fallback to another provider adapter is allowed silently. If provider selection fails, expose error.

## Debug Snapshot

Snapshot includes:

- selected provider per agent
- last normalized signal per agent
- last provider evidence id/source
- adapter failures
- redacted provider metadata

## Resource Lifecycle

TUI Agent Adapter Center owns lifecycle requests for:

- `provider_adapter`
- provider diagnostic observation handle

Rules:

- Register one provider adapter instance per agent/provider selection.
- Release provider adapter when agent stops, provider selection changes, or project scope closes.
- Provider diagnostic evidence must have bounded retention through Debug/Persistence policy.
- A provider adapter without active agent member/session lease is an orphan candidate.
- Adapter instance count, diagnostic evidence bytes, and signal extraction latency are efficiency metrics.

## Red Tests

- stdout-only final status fails.
- Codex SDK required for generic agent fails.
- provider-specific payload leaks into runtime business payload fails.
- provider adapter mutates task state fails.
- provider selection failure swallowed fails.
- `unknown` treated as success fails.
- Provider adapter without lifecycle lease fails.
- Provider diagnostic evidence unbounded growth fails.

## Open Decisions

- Exact provider ids in config.
- Whether provider adapter can be auto-detected from command name, or must be explicit.
- Whether user-defined marker adapters are v1 or later.
