# 09 Task Engine

## Purpose

Task Engine owns task state, queues, and role-level scheduling.

## Owns

- Task creation.
- Task state machine.
- Per-role serial queues.
- Cross-role concurrent readiness.
- Task event generation.
- Task snapshot.
- Task board projection.
- Task priority ordering.
- Task claim decision.
- Blocking task selection.
- Assigned-first claim ordering.
- Blocked-first claim ordering inside claim class.
- Normal task failure state from `agentteam task error`.

## Does Not Own

- Inter-agent route resolution.
- Terminal input construction.
- Agent stdout parsing.
- Error classification.
- Config parsing.
- Communication envelope delivery.
- TANote note append, forum thread projection, or note parsing.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `task.create` | Task Engine | Create task record and queued entry | `TaskReq01Create` | `TaskReq02Queued` | missing event |
| `task.schedule` | Task Engine | Project dispatch-ready work by role queue | queued tasks | `TaskReq03DispatchReady` | same-role concurrency |
| `task.claim` | Task Engine | Decide assigned/role-matching worker claim | claim request | claim result | unauthorized claim |
| `task.order_claim` | Task Engine | Apply assigned-first and blocked-first ordering | claim candidates | ordered claim | wrong priority |
| `task.transition` | Task Engine | Apply task state machine transition | task event | `TaskResp04StateChanged` | invalid transition |
| `task.project_board` | Task Engine | Build task board projection | task state | board projection | comm owns board |
| `task.snapshot` | Task Engine | Provide queue/task snapshot to Debug Center | task state | task snapshot | note body leak |
| `task.help` | Task Engine | Describe task lifecycle and claim rules | help topic | help model | notes as task truth |

## Module Help Contract

Required help topics:

```text
agentteam help task
agentteam help task send
agentteam help task claim
agentteam help task status
agentteam help task done
agentteam help task error
agentteam help task board
agentteam help task red-tests
```

Help content must explain:

- Task Engine owns task state and queues
- workers can claim only assigned or role-matching tasks
- assigned tasks outrank role-matching tasks
- blocked tasks outrank non-blocked tasks inside the same claim class
- `task error` is normal task failure state
- framework/agent runtime faults go to Error Center
- TANote entries may reference tasks but cannot mutate task state

Help content must not:

- tell Communication Center to decide priority/claim result
- infer task success from stdout alone
- treat `TANote.md` as task truth
- convert framework faults into normal task failures

## Public API Boundary

```text
TaskReq01Create -> TaskReq02Queued -> TaskReq03DispatchReady -> TaskResp04StateChanged
```

Task Engine is the only owner of task state transitions.

## Required Behavior

- Create task targeting one role/member.
- Queue tasks serially per target role.
- Allow independent roles to run concurrently.
- Maintain task board projection.
- Support priority ordering.
- Let workers claim highest-priority allowed task.
- Claim eligibility:
  - task assigned to requesting worker
  - task role matches requesting worker role
- Claim ordering:
  - assigned tasks first
  - role-matching tasks second
  - blocked tasks first inside the same claim class
- Mark sent/running/done/error.
- Persist every state transition as an event request.
- Never infer success without explicit completion signal or accepted future rule.
- Treat agent-reported `task error` as normal task state, not framework error.
- Allow task events to reference `note_id`/`thread_id` as evidence or discussion context.
- Never complete, error, assign, or claim a task only because a `TANote.md` entry says so.

## Error Behavior

Invalid state transition emits TaskState fault fact.

## Debug Snapshot

Snapshot includes:

- queues by role
- active tasks
- task board projection
- priority queues
- blocked tasks
- assigned claim candidates
- role-matching claim candidates
- latest state transitions
- note/thread references attached to tasks, not note bodies

## Resource Lifecycle

Task Engine owns lifecycle requests for:

- `task_record`
- queued task entry
- active task claim
- task board projection cursor

Rules:

- Register `task_record` when a task is created.
- Register active claim resource when a worker claim is accepted.
- Release active claim when task reaches terminal state or claim is explicitly revoked through Task Engine.
- Task record release policy is configurable; terminal tasks may remain as historical event truth but must not retain active claim/resource leases.
- A running task with no owning agent heartbeat becomes a blocked/orphan candidate, not silent success.
- Queue length by role, active claim count, oldest blocked task age, and projection row count are efficiency metrics.

## Red Tests

- Two tasks to same role cannot run concurrently.
- Tasks to different roles can be dispatch-ready concurrently.
- Worker cannot claim task outside assigned/role-matching scope.
- Assigned task outranks role-matching task.
- Blocked task outranks non-blocked task inside same claim class.
- Invalid state transition fails.
- Task success without completion evidence fails.
- Comm Center mutating queue internals fails architecture gate.
- Communication Center deciding priority/claim result fails architecture gate.
- TANote entry mutating task state fails architecture gate.
- Active claim without lifecycle lease fails.
- Terminal task retaining active claim leak fails.

## Open Decisions

- Exact completion signal: CLI explicit `task done`, agent output marker, or both.
- Whether task dependencies/DAG exist in v1.
- Whether task creation automatically opens a TANote thread in v1.
