# Report Flow

## Purpose

Report Flow turns persisted AgentTeam event logs into human-readable workflow reports.

The first MVP report renders an ASCII flow and Mermaid sequence diagram from `events/agentteam.jsonl` only.

## Owns

- read-only replay projection from Persistence Event Log records
- event-to-flow-step classification
- ASCII flow rendering
- Mermaid flow rendering
- report metadata: log path, event count, latest sequence

## Does Not Own

- event append or sequence assignment
- task state mutation
- communication delivery
- agent/session/tmux lifecycle
- debug snapshot capture
- UI/WebUI state

## Module Function Map

| feature_id | Public function | Owner | Notes |
|---|---|---|---|
| `report.flow` | `execute_report_flow` | Runtime Report Flow | CLI/runtime entrypoint for read-only report projection |
| `report.flow` | `build_flow_report` | Runtime Report Flow | Converts replayed event records to report model |
| `report.flow` | `classify_flow_step` | Runtime Report Flow | Maps one persisted event to one flow step |
| `report.flow` | `render_ascii_flow` | Runtime Report Flow | Renders ordered ASCII report text |
| `report.flow` | `render_mermaid_flow` | Runtime Report Flow | Renders ordered Mermaid sequence diagram |

## Module Help Contract

```text
agentteam report flow --runtime-home <runtime_home> --json
```

Required flags:

- `--runtime-home`: runtime directory containing `events/agentteam.jsonl`
- `--json`: required for MVP CLI output

Output projection includes:

- `log_path`
- `event_count`
- `latest_sequence`
- `steps`
- `ascii_flow`
- `mermaid_flow`

## Public API Boundary

Report Flow consumes:

- `agentteam_persist::replay_event_log`
- `agentteam_persist::PersistedEventRecord`

Report Flow returns:

- a serializable report projection through Runtime and Output Gateway

No module may call Report Flow to mutate state.

## Required Behavior

- Missing event log is a valid empty report with `event_count = 0`.
- Corrupt JSONL, duplicate sequence, or non-contiguous sequence must fail through Persistence replay.
- Known event kinds produce explicit actor-to-actor flow edges.
- Unknown event kinds are rendered as generic persisted events with the feature and event kind visible.
- The report must preserve event order by sequence.
- The report must never crop or rewrite semantic payload text for transport. Rendering labels may summarize event type and ids, while raw event payload remains available in the persisted log.

## Error Behavior

- Persistence replay failures become a `Report` local command error.
- Report Flow must not treat corrupt logs as partial success.
- Report Flow must not create Error Center framework events for read-only report generation failure in MVP CLI mode.

## Debug Snapshot

The report projection is itself debug evidence:

- log path
- latest sequence
- event count
- flow step count
- unknown event count

Debug Center may later embed this report by calling the public read-only API.

## Resource Lifecycle

Report Flow opens the event log through Persistence replay only.

It owns no long-lived resource, socket, tmux session, bridge process, or temp file.

## Red Tests

| red_test_id | Forbidden behavior | Expected failure |
|---|---|---|
| `red.report.live_state_source` | Report reads live task/session/agent state instead of event log | architecture gate or report test fails |
| `red.report.mutates_state` | Report appends events or writes report files during generation | report/unit or persistence-owner gate fails |
| `red.report.corrupt_log_success` | Corrupt or duplicate-sequence log renders a partial diagram as success | report/unit test fails |

## Open Decisions

- Whether future report commands should write `.md` artifacts is deferred. MVP returns JSON only.
- Whether Mermaid output should become a first-class file export is deferred.
