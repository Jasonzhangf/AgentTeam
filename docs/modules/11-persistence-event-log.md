# 11 Persistence Event Log

## Purpose

Persistence owns durable event append, replay, and materialized snapshots.

## Owns

- Append-only event log.
- Event replay.
- Materialized state snapshot.
- Persistence debug snapshot.
- State file paths under runtime home.
- Append receipts for TANote note events and materialized projection events.
- Append receipts for resource acquire/use/release/leak/orphan/cleanup events.
- Append receipts for debug bundle, debug evidence, and debug snapshot materialization events.

## Does Not Own

- Business state transition rules.
- Error classification.
- Config parsing.
- Terminal transport.
- TANote block validation or thread projection.
- Resource lifecycle decision logic.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `persist.append_event` | Persistence | Append validated event atomically | `PersistReq02ValidatedEvent` | `PersistResp03AppendReceipt` | append without receipt |
| `persist.validate_event` | Persistence + contracts | Validate typed event draft | `PersistReq01EventDraft` | `PersistReq02ValidatedEvent` | corrupt accepted |
| `persist.replay` | Persistence | Replay event log from zero | `PersistReq04Replay` | replay stream | replay mismatch |
| `persist.materialize` | Persistence | Rebuild materialized state snapshot | replay stream | `PersistResp05MaterializedState` | direct state write |
| `persist.detect_corruption` | Persistence | Detect corrupt event records loudly | event record | persistence fault fact | silent corruption |
| `persist.snapshot` | Persistence | Provide event/snapshot state to Debug Center | persistence state | persistence snapshot | path leak |
| `persist.help` | Persistence | Describe event log, replay, and receipts | help topic | help model | fallback storage |

## Module Help Contract

Required help topics:

```text
agentteam help persist
agentteam help persist event-log
agentteam help persist replay
agentteam help persist receipt
agentteam help persist snapshot
agentteam help persist red-tests
```

Help content must explain:

- Persistence is the only state-file writer
- every append returns a receipt
- event replay rebuilds durable state
- corruption fails loudly through Error Center
- modules request append/materialize; they do not write state files directly

Help content must not:

- suggest partial success after append failure
- suggest direct file writes by business modules
- hide corruption markers
- use config as runtime state storage

## Public API Boundary

```text
PersistReq01EventDraft -> PersistReq02ValidatedEvent -> PersistResp03AppendReceipt
PersistReq04Replay -> PersistResp05MaterializedState
```

No module writes state files directly.

## Required Behavior

- Append event atomically.
- Return append receipt.
- Replay from zero.
- Rebuild materialized state.
- Detect corrupt event records.
- Expose persistence snapshot.
- Persist TANote note append events before `TANote.md` projection is accepted.
- Persist resource lifecycle transitions before debug projections claim release, leak, orphan, or cleanup state.
- Persist Debug Center bundle/evidence materialization before debug output claims a bundle exists.

## Error Behavior

IO/corruption faults emit Persistence fault facts.

No fallback to partial success.

## Debug Snapshot

Snapshot includes:

- event log path
- snapshot path
- event count
- latest event id
- replay status
- corruption markers
- latest resource lifecycle event id
- latest debug bundle event id

## Resource Lifecycle

Persistence owns lifecycle requests for:

- `event_log_writer`
- replay cursor
- materialized snapshot writer
- debug bundle writer

Rules:

- Register event log writer when daemon opens append path.
- Release writer during scoped daemon shutdown only after final flush result event exists.
- Replay cursors must be bounded and released after replay/debug command finishes.
- Snapshot writer must emit append/materialize receipt before any module claims durable state.
- Debug bundle writer must be released after persisted bundle receipt exists.
- Open writer count, replay cursor count, debug bundle writer count, append latency, and replay duration are efficiency metrics.

## Red Tests

- Direct state file write by non-persistence module fails architecture gate.
- Corrupt event record fails loudly.
- Replay mismatch fails.
- Append without receipt fails.
- `TANote.md` projection without prior note event fails.
- Resource release/leak/orphan projection without prior lifecycle event fails.
- Debug bundle projection without prior persisted debug event fails.
- Event log writer without lifecycle lease fails.
- Replay cursor leak fails.

## Open Decisions

- Event format: JSONL with typed enum.
- Snapshot cadence.
- Whether event log is fsync-on-every-event in v1.
