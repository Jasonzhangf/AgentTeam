# 17 Resource Lifecycle Manager

## Purpose

Resource Lifecycle Manager owns resource lease registry, lifecycle state, orphan detection, leak detection, cleanup decision records, and efficiency budget reporting.

Each business resource still has exactly one owner module. Resource Lifecycle Manager does not take over business semantics; it records, audits, and projects resource health.

## Owns

- Resource id allocation.
- Resource lease registration.
- Resource state machine.
- Resource owner/scope registry.
- Reference count and heartbeat tracking.
- TTL and release policy tracking.
- Orphan detection.
- Leak detection.
- Efficiency budget counters.
- Growth-control counters.
- Resource debug snapshot.
- Cleanup request records.
- Resource lifecycle help text.

## Does Not Own

- Business state transitions.
- tmux execution.
- task/message/note semantics.
- error classification.
- persistence append implementation.
- config parsing.
- final CLI/UI rendering.
- broad cleanup or broad process termination.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `resource.acquire` | Resource Lifecycle Manager | Register a resource lease for an owner module/entity | `ResourceReq01AcquireIntent` | `ResourceLease04Active` | long-lived resource without lease |
| `resource.heartbeat` | Resource Lifecycle Manager | Record liveness and metric updates | active lease + metrics | `ResourceMetric03Observed` | stale resource hidden |
| `resource.release` | Resource Lifecycle Manager | Validate and record release request/result | lease id + owner receipt | `ResourceResp05Released` | release by non-owner |
| `resource.detect_orphan` | Resource Lifecycle Manager | Detect resource whose owner is gone or scope expired | resource registry + owner facts | `ResourceLeak04Orphaned` | orphan not reported |
| `resource.detect_leak` | Resource Lifecycle Manager | Detect unreleased or unbounded-growth resource | metrics + lease state | `ResourceLeak04Leaked` | leak not persisted |
| `resource.request_cleanup` | Resource Lifecycle Manager + owner module | Record scoped cleanup request using exact handle | orphan/leak + owner policy | cleanup request event | broad cleanup |
| `resource.project_budget` | Resource Lifecycle Manager | Project growth-control and budget use | metrics | budget projection | silent unbounded growth |
| `resource.snapshot` | Resource Lifecycle Manager | Provide resource snapshot to Debug Center | registry state | `ResourceDebugSnapshot` | debug missing lifecycle |
| `resource.help` | Resource Lifecycle Manager | Describe resource lifecycle commands and rules | help topic | help model | help suggests broad kill |

## Module Help Contract

Required help topics:

```text
agentteam help resource
agentteam help resource leases
agentteam help resource budgets
agentteam help resource orphans
agentteam help resource cleanup
agentteam help resource red-tests
```

Help content must explain:

- every long-lived resource needs a lease
- every lease has one owner module and one owner entity id
- resource cleanup must use exact resource handles
- orphan/leak detection persists events and exposes debug evidence
- v1 uses growth-control instead of aggressive memory caps
- unbounded growth is visible, bounded, and cleaned up
- Debug Center reads resource snapshots through Resource Lifecycle Manager

Help content must not:

- suggest broad process kill
- suggest broad file deletion
- say cleanup can be silent
- let non-owner modules mutate business resources
- say resource projection is business state truth

## Public API Boundary

```text
ResourceReq01AcquireIntent -> ResourceReq02ValidatedScope -> ResourceMetric03Initial -> ResourceLease04Active
ResourceReq01Heartbeat -> ResourceReq02ValidatedScope -> ResourceMetric03Observed -> ResourceLease04Active
ResourceReq01ReleaseIntent -> ResourceReq02ValidatedScope -> ResourceResp05Released
ResourceLeak01ScanIntent -> ResourceLeak02Candidate -> ResourceLeak03Classified -> ResourceLeak04Projection
```

Only Resource Lifecycle Manager allocates resource ids, lease ids, and leak/orphan projections.

Only owner modules can acquire or release their resources.

Only Persistence appends resource lifecycle events.

Only Error Center classifies resource faults.

## Resource State Machine

```text
planned
  |
  v
acquired
  |
  +--> active
  |      |
  |      +--> releasing
  |      |      |
  |      |      v
  |      |   released
  |      |
  |      +--> orphan_suspected
  |      |      |
  |      |      v
  |      |   orphan_confirmed
  |      |
  |      +--> leak_suspected
  |             |
  |             v
  |          leak_confirmed
  |
  +--> failed
```

No state transition may be skipped.

## Required Resource Fields

```text
resource_id
resource_class
owner_module
owner_entity_id
scope
lease_id
state
created_at
last_seen_at
last_event_id
ref_count
memory_bytes_estimate
handle_count
ttl_policy
release_policy
cleanup_policy
temp_path
evidence_id
```

## MVP Resource Classes

```text
daemon_process
tmux_session
zterm_connection
tui_process
agent_member
task_record
message_envelope
tanote_projection
event_log_writer
debug_bundle
config_snapshot
provider_adapter
ui_subscription
temporary_file
```

## Owner Rules

| resource_class | Business owner | Lifecycle owner |
|---|---|---|
| `daemon_process` | Startup Session Manager | Resource Lifecycle Manager |
| `tmux_session` | zterm/tmux Adapter | Resource Lifecycle Manager |
| `zterm_connection` | zterm/tmux Adapter | Resource Lifecycle Manager |
| `tui_process` | zterm/tmux Adapter | Resource Lifecycle Manager |
| `agent_member` | Agent Registry | Resource Lifecycle Manager |
| `task_record` | Task Engine | Resource Lifecycle Manager |
| `message_envelope` | Communication Center | Resource Lifecycle Manager |
| `tanote_projection` | TANote Board | Resource Lifecycle Manager |
| `event_log_writer` | Persistence | Resource Lifecycle Manager |
| `debug_bundle` | Debug Center | Resource Lifecycle Manager |
| `config_snapshot` | Config Center | Resource Lifecycle Manager |
| `provider_adapter` | TUI Agent Adapter Center | Resource Lifecycle Manager |
| `ui_subscription` | UI Gateway | Resource Lifecycle Manager |
| `temporary_file` | Creating owner module | Resource Lifecycle Manager |

## Required Behavior

- Register every long-lived resource before first use.
- Persist acquire, heartbeat, release, orphan, leak, cleanup request, and cleanup result events.
- Track owner module and owner entity id.
- Track resource scope: project, team, agent, task, message, debug bundle, or process.
- Track resource budget/growth metrics.
- Detect owner disappearance.
- Detect missing heartbeat after configured TTL.
- Detect unreleased resources after owner scope closes.
- Detect monotonic unbounded growth in memory estimates, handle counts, queues, cursors, and temp files.
- Request cleanup through exact handles and owner policy.
- Never use broad process kill or broad file deletion.
- Never mark cleanup successful without owner receipt and event receipt.
- On daemon/session close, automatically release resources owned by that scope and cleanup tracked temporary files.
- Do not delete persistent artifacts during temp cleanup.

## Efficiency And Growth Management

Efficiency metrics must be available in debug snapshots:

- resource count by class
- resource count by owner module
- memory bytes estimate by class
- handle count by class
- pending queue length where applicable
- oldest active lease age
- stale heartbeat count
- budget overrun ids
- unbounded growth suspect ids
- temporary file count and bytes
- cleanup latency

v1 policy:

- Do not fail normal work only because a hard memory number is crossed.
- Do enforce bounded growth through TTL, release on scope close, bounded in-memory projections, queue drain rules, and persisted debug evidence.
- Full business payload remains in durable event/evidence paths; in-memory debug/projection views may be bounded with explicit metadata.
- emit `ResourceGrowth` or `ResourceBudget` fault fact when a resource grows monotonically without release/drain or violates configured policy
- persist event with evidence id
- keep user-visible payload semantics intact
- projection truncation is allowed only for debug/projection payloads and must include truncation metadata

## Orphan And Leak Rules

Orphan examples:

- tmux session exists but no active agent lease owns it
- zterm connection exists after owning render subscription closed
- debug bundle file exists without active or completed bundle event
- task record lease remains active after task final state and release window
- temp file exists after daemon/session scoped shutdown cleanup

Leak examples:

- ref_count never returns to zero
- heartbeat remains stale past TTL
- memory estimate exceeds budget and keeps growing
- message envelope queue keeps growing without delivery/failure events
- temp file count/bytes keep growing without materialization or cleanup receipts

All orphan/leak findings must include:

```text
resource_id
lease_id
owner_module
owner_entity_id
last_event_id
evidence_id
reason
recommended_owner_action
```

## Persistence Behavior

Resource Lifecycle Manager requests Persistence events for:

- resource acquire requested/result
- resource heartbeat observed
- resource release requested/result
- orphan scan requested/result
- leak scan requested/result
- cleanup requested/result
- budget overrun observed
- scoped shutdown cleanup requested/result
- temporary file cleanup requested/result

## Resource Lifecycle

Resource Lifecycle Manager owns lifecycle requests for its own internal:

- lease registry snapshot
- orphan scan cursor
- leak scan cursor
- budget projection cursor
- scoped shutdown cleanup cursor

Rules:

- The lease registry is a daemon-core resource and must be bounded by project/team scope.
- Orphan/leak scan cursors are short-lived and released after scan result is persisted.
- Budget projection cursors are released after Debug/Output Gateway projection completes.
- Scoped shutdown cleanup cursors are released after cleanup result is persisted.
- Resource Lifecycle Manager must not exempt itself from lease/debug rules.
- Registry size, scan latency, cursor count, and budget projection bytes are efficiency metrics.

## Error Behavior

Resource faults emit ResourceLifecycle fault facts to Error Center.

Examples:

- acquire without owner
- duplicate active lease for exclusive resource
- release by non-owner
- missing heartbeat past TTL
- over-budget resource
- unbounded growth suspected
- orphan confirmed
- cleanup failed
- lifecycle projection without persistence receipt

## Debug Snapshot

Snapshot includes:

- resource counts by class
- resource counts by owner module
- active leases
- stale leases
- orphan candidates
- confirmed orphans
- leak candidates
- confirmed leaks
- budget usage
- growth-control suspects
- temporary file cleanup results
- cleanup requests/results
- latest resource lifecycle event id

Snapshot must not expose secrets. It may include internal handles only in human-authorized debug bundles, not agent-facing output.

## Red Tests

- Long-lived resource created without lease fails.
- Resource release by non-owner fails.
- Exclusive resource double-acquire fails.
- Missing heartbeat not detected fails.
- Owner scope closed with active resource fails.
- Orphan resource not persisted as event fails.
- Leak not visible in debug snapshot fails.
- Cleanup success without owner receipt fails.
- Broad process kill cleanup fails.
- Broad file deletion cleanup fails.
- Unbounded growth silently ignored fails.
- Scoped shutdown leaves tracked temp files without cleanup result fails.
- Debug bundle missing resource lifecycle summary fails.
- Resource Lifecycle Manager internal scan cursor leak fails.

## Open Decisions

- Exact default TTL per resource class.
- Exact warning thresholds for growth-control metrics.
- Cleanup outside scoped daemon/session shutdown requires owner policy or explicit user command in v1.
- Which internal handles are allowed in human-only debug bundles.
