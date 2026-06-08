# 08 Team Orchestrator

## Purpose

Team Orchestrator coordinates modules. It is pure orchestration, not a business logic dumping ground.

## Owns

- Team command coordination.
- Module call order.
- Runtime command dispatch.
- High-level state transition requests.

## Does Not Own

- Config parsing.
- Error classification.
- Task queue internals.
- Message routing internals.
- Terminal transport.
- Persistence file IO.
- Output formatting.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `team.route_command` | Team Orchestrator | Route validated daemon command to owner modules | `TeamReq04DaemonCommand` | `TeamRuntime05ModulePlan` | gateway owns call order |
| `team.create_team` | Team Orchestrator + Agent Registry | Coordinate team/member creation requests | daemon command | module call result | duplicate owner |
| `team.dispatch_task` | Team Orchestrator + Task Engine + Comm Center | Coordinate task creation and delivery | validated intent | daemon result | direct queue mutation |
| `team.dispatch_message` | Team Orchestrator + Comm Center | Coordinate message route request | validated intent | daemon result | direct tmux send |
| `team.request_debug` | Team Orchestrator + Debug Center | Coordinate debug snapshot request | debug intent | daemon result | private state read |
| `team.snapshot` | Team Orchestrator | Provide orchestration trace to Debug Center | command trace | orchestrator snapshot | leaked lock |
| `team.help` | Team Orchestrator | Describe orchestration boundaries | help topic | help model | business logic dumping |

## Module Help Contract

Required help topics:

```text
agentteam help team
agentteam help team orchestration
agentteam help team create
agentteam help team members
agentteam help team dispatch
agentteam help team red-tests
```

Help content must explain:

- Team Orchestrator coordinates owner modules only
- validated daemon commands are the entry point
- task logic belongs to Task Engine
- message routing belongs to Communication Center
- terminal transport belongs to zterm/tmux Adapter
- final output belongs to Output Gateway

Help content must not:

- document raw CLI parsing inside orchestrator
- suggest orchestrator classifies errors
- suggest direct tmux/zterm calls
- suggest direct persistence file writes

## Public API Boundary

```text
TeamReq04DaemonCommand -> TeamRuntime05ModulePlan -> TeamResp05DaemonResult
```

The orchestrator only accepts validated daemon commands.

## Required Behavior

- Create team.
- Add/remove/list members by calling Agent Registry.
- Send task by calling Task Engine and Communication Center.
- Send message by calling Communication Center.
- Request render by calling UI Gateway/Adapter boundary.
- Request debug snapshot by calling Debug Center.

## Error Behavior

Invariant faults go to Error Center.

Orchestrator must not repair downstream module failures.

## Debug Snapshot

Snapshot includes:

- current command in flight
- module call trace
- active orchestration locks
- latest command result ids

## Resource Lifecycle

Team Orchestrator owns lifecycle requests for:

- in-flight orchestration command handles
- orchestration locks

Rules:

- Register an orchestration command handle before invoking downstream modules.
- Release the handle only after success or classified failure result is persisted.
- Orchestration locks must have owner command id and timeout policy.
- A lock whose command is no longer active is an orphan candidate.
- Active command count, lock age, and downstream call latency are efficiency metrics.

## Red Tests

- Orchestrator parsing raw CLI fails.
- Orchestrator direct tmux call fails.
- Orchestrator classifying error fails.
- Orchestrator formatting CLI output fails.
- Orchestration lock without lifecycle lease fails.
- Completed command with leaked orchestration handle fails.

## Open Decisions

- Whether orchestrator is a crate or module inside runtime crate.
- Whether commands are processed single-threaded or through actor mailbox.
