# 10 Debug Center

## Purpose

Debug Center collects typed snapshots from every module through public snapshot APIs.

## Owns

- Snapshot request routing.
- Debug bundle creation.
- Redaction.
- Snapshot persistence request.
- Debug output projection input.

## Does Not Own

- Module private state.
- Runtime mutation.
- Error classification.
- Config parsing.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `debug.request_snapshot` | Debug Center | Parse validated debug intent into module snapshot requests | `DebugReq01SnapshotIntent` | `DebugReq02ModuleSnapshotRequest` | private read |
| `debug.collect_module` | Debug Center | Collect snapshot through public module API | module snapshot request | module snapshot | missing API |
| `debug.collect_resource` | Debug Center + Resource Lifecycle Manager | Include resource lifecycle snapshot in MVP bundle | snapshot request | resource snapshot | missing lifecycle |
| `debug.redact` | Debug Center + contracts | Redact secrets from debug material | module snapshots | redacted snapshots | token leak |
| `debug.persist_bundle` | Debug Center + Persistence | Persist bundle before projection | `DebugResp03Bundle` | persistence receipt | print-only debug |
| `debug.fetch_evidence` | Debug Center | Fetch persisted evidence by `evidence_id` | evidence id | evidence material | missing evidence |
| `debug.snapshot` | Debug Center | Provide Debug Center self-snapshot | debug state | debug snapshot | bundle leak |
| `debug.help` | Debug Center | Describe snapshot/evidence/resource debug rules | help topic | help model | direct private state |

## Module Help Contract

Required help topics:

```text
agentteam help debug
agentteam help debug snapshot
agentteam help debug evidence
agentteam help debug resources
agentteam help debug bundles
agentteam help debug red-tests
```

Help content must explain:

- Debug Center uses public snapshot APIs only
- every debug bundle is persisted before output
- debug output returns `bundle_id` and persistence receipt
- error evidence is fetched by `evidence_id`
- MVP debug bundles include Resource Lifecycle snapshots
- print-only debug capture is invalid in v1

Help content must not:

- instruct modules to expose private state
- suggest non-persisted debug output
- expose secrets in examples
- treat debug metadata as business payload

## Public API Boundary

```text
DebugReq01SnapshotIntent -> DebugReq02ModuleSnapshotRequest -> DebugResp03Bundle
```

Modules expose snapshot APIs. Debug Center cannot inspect private internals.

## Required Behavior

- Capture all module snapshots.
- Capture one module snapshot.
- Capture one team/agent/task snapshot.
- Redact secrets.
- Persist every debug bundle under runtime home before returning a CLI/UI projection.
- Return persisted `bundle_id` and persistence receipt in debug output.
- Include feature ids and module owner names.
- Pull TANote snapshots only through the TANote Board snapshot API.
- Include Resource Lifecycle snapshots in MVP debug bundles.
- Link every suspicious resource to its owner module, lease id, last event id, and cleanup policy.
- Do not support print-only debug capture in v1.

## Error Behavior

Snapshot failure emits DebugSnapshot fault fact.

## Debug Snapshot

Debug Center snapshot includes:

- snapshot requests
- bundle ids
- redaction policy version
- failed modules
- TANote thread/note summary ids when requested
- resource lease summary, leak/orphan counts, and budget overrun ids

## Resource Lifecycle

Debug Center owns lifecycle requests for:

- `debug_bundle`
- module snapshot request handle
- evidence fetch handle

Rules:

- Register debug bundle resource before collecting module snapshots.
- Register snapshot request handles per module and release each after success or classified failure.
- Release evidence fetch handles after persisted bundle materialization.
- A debug bundle without Resource Lifecycle snapshot is invalid for MVP.
- A debug bundle without Persistence append receipt is invalid.
- Bundle byte size, snapshot count, failed module count, and collection latency are efficiency metrics.

## Red Tests

- Secret token leaks fail.
- Private state access fails architecture gate.
- Missing module snapshot API fails required-file gate.
- Snapshot failure without error event fails.
- Debug Center parsing `TANote.md` directly fails architecture gate.
- Debug bundle missing Resource Lifecycle snapshot fails MVP debug gate.
- Debug bundle without lifecycle lease fails.
- Debug bundle without persistence receipt fails.
- Print-only debug capture fails.
- Module snapshot handle leak fails.

## Open Decisions

- Snapshot file format: JSON or JSONL bundle.
