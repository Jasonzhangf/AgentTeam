# 02 Error Center

## Purpose

Error Center is the only module that classifies, stores, persists, evidences, and projects framework errors.

All errors must be persisted to the event log. There is no non-durable error path.

## Owns

- Error taxonomy.
- Error chain contracts.
- Conversion from module fault facts to classified runtime errors.
- Error event emission.
- Error debug snapshot.
- Final error projection contract for Output Gateway.
- Error severity.
- Error code generation.
- Evidence id generation.
- Distinction between normal task failure and agent/framework fault.

## Does Not Own

- Retrying operations.
- Fixing invalid payloads.
- Formatting normal success responses.
- Module-specific recovery logic.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `error.accept_fault_fact` | Error Center | Accept raw fault facts from modules | `TeamErr01FaultFact` | `TeamErr01FaultFactReceipt` | swallowed fault |
| `error.classify` | Error Center | Classify every fault into stable error class/code | `TeamErr01FaultFact` | `TeamErr02Classified` | unclassified error |
| `error.assign_severity` | Error Center | Assign `fatal/error/warn/info` severity | `TeamErr02Classified` | `TeamErr02Classified` | missing severity |
| `error.generate_code` | Error Center | Generate `<module>.<class>.<specific>.<time>.<seq>` | `TeamErr02Classified` | `TeamErr02Classified` | malformed code |
| `error.create_evidence` | Error Center + Debug Center | Create independent `evidence_id` and evidence request | `TeamErr02Classified` | `TeamErr02EvidenceLinked` | missing evidence id |
| `error.persist_event` | Error Center + Persistence | Request durable error event append for every error | `TeamErr02Classified` | `TeamErr03RuntimeEvent` | any error not persisted |
| `error.project` | Error Center + Output Gateway | Build error projection for output | `TeamErr03RuntimeEvent` | `TeamErr04Projection` | success-wrapped error |
| `error.snapshot` | Error Center | Provide error counters/latest errors to Debug Center | internal error state | `ErrorDebugSnapshot` | token/evidence leak |
| `error.help` | Error Center | Describe error classes/codes and evidence rules | help topic | rendered help model | module final text ownership violation |

## Module Help Contract

Error Center must expose help text for CLI/UI/skill surfaces.

Required help topics:

```text
agentteam help error
agentteam help error classes
agentteam help error codes
agentteam help error evidence
agentteam help error red-tests
```

Help content must explain:

- every failure enters Error Center
- every error is persisted to event log
- modules submit fault facts, not final user-facing text
- errors are never converted to success
- no fallback/downgrade/silent retry is allowed
- error code format: `<module>.<class>.<specific>.<time>.<seq>`
- severity values: `fatal`, `error`, `warn`, `info`
- how to read error class, code, feature id, module owner, and evidence id
- task execution failure differs from agent/framework failure

Help content must not:

- instruct users to ignore an error
- suggest retry loops as fallback
- hide which module emitted the fault
- expose secrets in evidence examples

## Public API Boundary

```text
TeamErr01FaultFact -> TeamErr02Classified -> TeamErr02EvidenceLinked -> TeamErr03RuntimeEvent -> TeamErr04Projection
```

Modules can submit `FaultFact`. Only Error Center can classify.

## Required Behavior

- Every failure enters the error chain.
- Every error is persisted to the event log.
- Errors include module owner, feature id, chain node, evidence id, timestamp, sequence, severity, and code.
- Error code format:

```text
<module>.<class>.<specific>.<time>.<seq>
```

Example:

```text
config.validation.missing_project_slug.20260608T063012Z.000001
```

- Every error must have an independent `evidence_id`.
- Debug Center must be able to fetch the evidence snapshot by `evidence_id`.
- Severity values:

```text
fatal
error
warn
info
```

- Transport failures stay transport failures; they cannot become task success.
- Validation failures stay validation failures; they cannot be retried as fallback.
- Normal agent task execution failure is reported by the assigned agent through `agentteam task error`; this is task state, not automatically a framework error.
- Agent process/session/framework failure is handled by the framework and enters Error Center.

## Task Failure vs Agent/Framework Failure

Task failure:

- Agent is alive and reports `agentteam task error`.
- Task Engine records task status `error`.
- Error Center records an error only if the task error command itself fails or if policy later says task errors are also mirrored into error events.

Agent/framework failure:

- Agent process crashes.
- tmux/zterm transport fails.
- agent session cannot be found.
- agent output gateway cannot observe assigned agent.
- daemon/runtime invariant breaks.
- These are framework errors and must enter Error Center with persisted event and evidence id.

## Error Classes

- `ConfigValidation`
- `CliParse`
- `RuntimeInvariant`
- `TaskState`
- `AgentNaming`
- `CommunicationRoute`
- `Transport`
- `Persistence`
- `DebugSnapshot`
- `UiProjection`
- `ArchitectureGate`
- `AgentRuntime`

## Severity Rules

| severity | Meaning | Examples |
|---|---|---|
| `fatal` | daemon/framework cannot continue safely | event log unavailable, core invariant broken |
| `error` | operation failed and user/task is affected | missing agent session, transport send failed |
| `warn` | operation completed but non-critical issue must be visible | slow snapshot, deprecated config field |
| `info` | informational error-chain event worth preserving | explicit user diagnostic marker |

## Debug Snapshot

Snapshot includes:

- latest classified errors
- latest evidence ids
- counts by class
- counts by severity
- counts by module
- unprojected errors
- redacted evidence

## Resource Lifecycle

Error Center owns lifecycle requests for:

- error evidence record
- classified error projection handle

Rules:

- Register evidence resource when `evidence_id` is generated.
- Release short-lived projection handles after Output Gateway receives the projection.
- Evidence retention must follow configured policy and never be silently deleted.
- An evidence id referenced by an error event but missing from Debug Center/Persistence is an orphan/corruption candidate.
- Error count, evidence bytes, unprojected error count, and classification latency are efficiency metrics.

## Red Tests

- Success-wrapped error fails.
- Unclassified error event fails.
- Error without event-log append fails.
- Error without evidence_id fails.
- Error without severity fails.
- Malformed error code fails.
- Module final error text rendering fails architecture gate.
- Error without feature id fails.
- Error swallowed without event fails.
- Agent framework failure treated as normal task failure fails.
- Evidence resource without lifecycle lease fails.
- Error event referencing missing evidence fails.

## Open Decisions

- Exact evidence storage location and retention policy.
