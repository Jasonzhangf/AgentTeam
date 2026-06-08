# 05 Output Gateway

## Purpose

Output Gateway renders all external responses and projections for CLI/API/UI/WebUI consumers.

## Owns

- Success response projection.
- Error response projection from Error Center.
- Debug bundle projection.
- Human-readable CLI text.
- Machine-readable JSON output.
- UI/WebUI projection models.
- Projection stream payloads.

## Does Not Own

- Error classification.
- Runtime state mutation.
- Task state transitions.
- Transport sends to TUI agents.
- UI/WebUI state ownership.
- Runtime internal state exposure.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `output.render_cli_text` | Output Gateway | Render final human-readable CLI output | `TeamResp05DaemonResult` | `TeamResp06CliRendered` | module renders final text |
| `output.render_json` | Output Gateway | Render machine-readable JSON output | `TeamResp05DaemonResult` | JSON projection | required field omitted |
| `output.render_error` | Output Gateway + Error Center | Render classified error projection | `TeamErr04Projection` | `TeamResp06CliRendered` | error code dropped |
| `output.render_ui_projection` | Output Gateway | Build UI/WebUI display projection | `TeamResp05DaemonResult` | `TeamResp06UiProjection` | private state leak |
| `output.redact_projection` | Output Gateway + contracts | Redact secrets before projection | projection draft | redacted projection | token leak |
| `output.snapshot` | Output Gateway | Provide render/projection snapshot to Debug Center | gateway state | output snapshot | debug metadata as payload |
| `output.help` | Output Gateway | Describe output modes and projection rules | help topic | help model | final text outside gateway |

## Module Help Contract

Required help topics:

```text
agentteam help output
agentteam help output text
agentteam help output json
agentteam help output ui
agentteam help output error
agentteam help output red-tests
```

Help content must explain:

- Output Gateway is the only final renderer
- text, JSON, error, debug, and UI projections keep status/error/evidence ids
- secrets are redacted before UI/WebUI projection
- debug truncation may not crop user-visible business semantics

Help content must not:

- tell modules to render final CLI/UI text
- hide missing verification or incomplete state
- expose private module state
- convert classified errors to success output

## Public API Boundary

```text
TeamResp05DaemonResult -> TeamResp06CliRendered
TeamErr04Projection -> TeamResp06CliRendered
TeamResp05DaemonResult -> TeamResp06UiProjection
TeamErr04Projection -> TeamResp06UiProjection
```

Only Output Gateway renders final CLI text.
Only Output Gateway builds UI/WebUI display projections.

## Required Behavior

- Support text output.
- Support JSON output for scripts.
- Support UI/WebUI projection output.
- Preserve status and error codes.
- Avoid hiding missing verification or incomplete state.
- Redact secrets before any UI/WebUI projection.
- Never expose module private state.

## Error Behavior

If rendering fails, emit UiProjection or OutputProjection fault fact.

## Debug Snapshot

Snapshot includes:

- last projection type
- output mode
- projection failures
- UI/WebUI projection version

## Resource Lifecycle

Output Gateway owns lifecycle requests for:

- response projection handle
- UI projection stream item
- rendered debug output handle

Rules:

- Register projection handle before rendering CLI/UI/API output.
- Release projection handle after output is delivered or classified rendering failure is persisted.
- Debug/projection payload truncation may only affect debug/projection material and must include truncation metadata.
- User-visible business payload semantics must not be cropped.
- Projection bytes, render latency, stream queue depth, and truncation marker count are efficiency metrics.

## Red Tests

- Module final text rendering fails architecture gate.
- Error code dropped during rendering fails.
- JSON output omits required fields fails.
- UI projection exposes private state fails.
- UI projection drops status/error/evidence ids fails.
- Output projection without lifecycle lease fails.
- Projection truncation without metadata fails.

## Open Decisions

- Whether text output defaults to compact or table view.
- Whether JSON output is stable in v1 or marked experimental.
