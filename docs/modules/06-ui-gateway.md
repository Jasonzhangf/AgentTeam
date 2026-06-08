# 06 UI Gateway

## Purpose

UI Gateway provides a separated UI/WebUI-facing API for consuming input and output projections.

UI/WebUI is decoupled from the agent framework. It does not own runtime state, task state, agent state, transport state, or module internals.

## Owns

- UI request projection.
- UI state subscription contract.
- Single-agent render attachment metadata.
- UI-safe state snapshots.
- WebUI projection consumption.
- UI command intent submission through Input Gateway.
- UI display model consumption from Output Gateway.

## Does Not Own

- Terminal mirror truth.
- Runtime mutation.
- Task queue logic.
- Error classification.
- Agent framework state ownership.
- Direct daemon module calls.
- Direct Task Engine calls.
- Direct Communication Center calls.
- Direct tmux/zterm calls.
- Persistence writes.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `ui.consume_projection` | UI Gateway | Consume output projections for WebUI/UI display | `TeamResp06Projection` | `UiResp03Projection` | UI reads internals |
| `ui.submit_input` | UI Gateway + Input Gateway | Submit UI user action as raw input intent | `UiReq01Raw` | `TeamReq01UiRaw` | UI bypasses Input Gateway |
| `ui.subscribe_projection` | UI Gateway | Subscribe to projected state streams only | projection stream | UI subscription | direct runtime subscription |
| `ui.render_agent_surface` | UI Gateway + zterm/tmux Adapter | Request render metadata for selected TA agent | UI target | render projection | non-TA render |
| `ui.snapshot` | UI Gateway | Provide UI subscription/projection snapshot | UI gateway state | UI snapshot | private state leak |
| `ui.help` | UI Gateway | Describe UI/WebUI boundary | help topic | help model | UI framework coupling |

## Module Help Contract

Required help topics:

```text
agentteam help ui
agentteam help ui web
agentteam help ui input
agentteam help ui output
agentteam help ui projection
agentteam help ui red-tests
```

Help content must explain:

- UI/WebUI consumes Input Gateway and Output Gateway contracts
- UI/WebUI must not own agent framework state
- UI actions enter as raw UI input and pass through Input Gateway
- UI display uses Output Gateway projections only
- UI render requests target managed TA agents only

Help content must not:

- document direct runtime internals as UI API
- tell UI to mutate task/agent/comm state
- tell UI to call tmux/zterm directly
- tell UI to write persistence state

## Public API Boundary

```text
UiReq01Raw -> UiReq02Validated -> UiResp03Projection
TeamResp06Projection -> UiResp03Projection
```

UI Gateway can submit intent through Input Gateway and consume projections from Output Gateway.

UI Gateway cannot mutate runtime internals directly.

## Required Behavior

- Render selected agent by team and role/name.
- Show team members.
- Show task list.
- Show message list.
- Show daemon/debug status.
- Expose only projected state.
- WebUI consumes only projection models.
- UI user actions must enter Input Gateway.
- UI output must come from Output Gateway projection.
- No framework state is stored in UI except ephemeral view state.

## Error Behavior

UI faults enter Error Center; UI must not invent fallback state.

## Debug Snapshot

Snapshot includes:

- active UI subscriptions
- selected agent render target
- projection version
- active projection streams
- last UI input intent shape

## Resource Lifecycle

UI Gateway owns lifecycle requests for:

- `ui_subscription`
- selected render projection subscription
- UI projection stream cursor

Rules:

- Register every UI/WebUI subscription before streaming projections.
- Release subscription when client disconnects, render target changes, or session closes.
- A subscription with stale heartbeat becomes an orphan candidate.
- UI Gateway must not keep full framework state in memory; it stores ephemeral view cursors and projection ids only.
- Active subscription count, stale subscription count, and projection queue depth are efficiency metrics.

## Red Tests

- UI mutating runtime state directly fails.
- UI reading private module state fails.
- UI bypassing Input Gateway fails.
- UI bypassing Output Gateway projections fails.
- UI writing Persistence state fails.
- UI direct Task Engine/Communication Center call fails.
- Render target outside TA scope fails.
- UI subscription without lifecycle lease fails.
- Stale UI subscription hidden from debug fails.

## Open Decisions

- Whether render uses zterm UI package later or only CLI attach in v1.
