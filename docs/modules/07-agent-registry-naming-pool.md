# 07 Agent Registry And Naming Pool

## Purpose

Agent Registry owns daemon-domain-local agent identity, naming, capability facts, lifecycle status projection, and TA tmux session naming.

It does not use provider SDKs as generic status truth. AgentTeam must support non-Codex TUI agents with the same lifecycle model.

## Owns

- Agent name pool.
- Domain-local agent name uniqueness.
- Configured root manager name.
- Worker overflow name rule.
- Role/member registry.
- Manager/worker capability facts.
- Single super manager v1 invariant.
- TA session name builder/parser.
- Project-scoped session directory contract.
- Managed-session filtering.
- Agent lifecycle status projection.
- Agent owner facts for assigned/claimed tasks.
- It does not choose attach_tui/headless control mode.

## Does Not Own

- Task queue.
- Message delivery.
- Process transport.
- stdout/buffer transport.
- Config parsing.
- Error classification.
- Provider SDK status detection.
- Task board authorization decisions beyond identity/capability facts.
- Daemon domain id allocation or cross-daemon route resolution.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `agent.allocate_manager_name` | Agent Registry | Allocate the configured root manager name inside one daemon domain | domain + team config | `AgentNameResp03Allocated` | missing or duplicate manager |
| `agent.allocate_worker_name` | Agent Registry | Allocate worker name from fixed pool or overflow rule inside one daemon domain | domain + project slug + worker index | `AgentNameResp03Allocated` | pool order, overflow |
| `agent.validate_single_manager` | Agent Registry + Config Center | Enforce one super manager in v1 | team config | capability facts | multiple managers |
| `agent.build_ta_session_name` | Agent Registry | Build `TA_<domain_id>_<project_slug>_<agent_name>` | domain id + project slug + name | tmux session name | malformed TA name |
| `agent.project_domain_address` | Agent Registry + Daemon Domain Registry | Project `agent@domain` address for external use | local agent + domain fact | domain-qualified address | local name used globally |
| `agent.build_session_dir` | Agent Registry + Persistence | Build project session directory contract | project slug | session dir path | path outside home |
| `agent.classify_capability` | Agent Registry | Classify manager/worker capabilities | team member config | capability facts | unauthorized action |
| `agent.project_status` | Agent Registry + Runtime | Project `offline/starting/idle/busy/error` | runtime + adapter + task + error facts | status projection | SDK-only status |
| `agent.snapshot` | Agent Registry | Provide registry snapshot to Debug Center | registry state | registry snapshot | secret/private leak |
| `agent.help` | Agent Registry | Describe naming/status/session rules | help topic | help model | hidden SDK dependency |

## Module Help Contract

Required help topics:

```text
agentteam help agent
agentteam help agent names
agentteam help agent manager
agentteam help agent worker-pool
agentteam help agent status
agentteam help agent sessions
agentteam help agent domains
agentteam help agent red-tests
```

Help content must explain:

- root manager name comes from the single configured `team_role = "manager"` member; the default example name is `Kevin`
- v1 supports exactly one super manager
- first 20 workers use the fixed English name pool
- workers beyond 20 use `<project_slug>_worker_<seq>`
- agent names are unique inside one daemon domain/team scope, not globally
- domain-qualified address format is `agent@domain`
- TA session name is `TA_<domain_id>_<project_slug>_<agent_name>`
- session metadata lives under `~/.agentteam/sessions/<project_slug>/`
- TUI launch without framework/transport error plus TA session existence means ready
- steady statuses are `offline`, `starting`, `idle`, `busy`, `error`
- Codex SDK is not generic agent-status truth
- tmux/session identifiers are framework internals and hidden from agents

Help content must not:

- suggest hard-coding the manager name in code instead of reading the configured manager member
- suggest SDK-only status detection
- suggest storing session state in `config.toml`
- suggest operating non-TA sessions
- expose tmux session names as agent-facing identity
- treat local agent names as globally unique

## Public API Boundary

```text
AgentNameReq01Raw -> AgentNameReq02Validated -> AgentNameResp03Allocated
AgentAddrReq01Local -> AgentAddrReq02DomainScoped -> AgentAddrResp03Projected
AgentStatusReq01RuntimeFacts -> AgentStatusReq02TaskFacts -> AgentStatusResp03Projected
```

Only Agent Registry can allocate or parse managed local agent names inside a resolved daemon domain.

Only Daemon Domain Registry parses and validates daemon domain ids.

Only Agent Registry/Runtime can project agent lifecycle status from module facts.

## Naming Rules

Manager:

```text
Kevin
```

Worker pool, ordered:

```text
Alice
Bob
Carol
David
Emma
Frank
Grace
Henry
Irene
Jack
Laura
Mike
Nora
Oscar
Paul
Quinn
Rose
Sam
Tina
Victor
```

Worker allocation:

- worker index 1-20 uses the fixed pool.
- worker index 21+ uses:

```text
<project_slug>_worker_<seq>
```

Example:

```text
agentteam_worker_21
```

Managed tmux session name:

```text
TA_<domain_id>_<project_slug>_<agent_name>
```

Examples:

```text
TA_local_agentteam_Kevin
TA_local_agentteam_Alice
TA_review-daemon_agentteam_Alice
```

These names are internal framework identifiers. Agents use domain-qualified addresses such as `Kevin@local` or `Alice@review-daemon` when crossing daemon boundaries, not tmux session names.

## Required Behavior

- Each agent has a unique `name` inside one daemon domain/team scope.
- Each agent has a work role.
- Each agent has a team role category: `manager` or `worker`.
- v1 allows exactly one super manager.
- Exactly one root manager must be configured in v1. The default sample config uses `Kevin`.
- Multiple managers are future expansion, not v1 behavior.
- Worker capability allows ready report, task-board query, task claim, task update, task done/error for assigned/claimed tasks.
- Agent owner mapping defines which agent owns a task when assigned or claimed.
- Project slug and agent name must be normalized before TA session construction.
- Domain id must be normalized and resolved by Daemon Domain Registry before TA session construction.
- External projections use `agent@domain` when there is any cross-daemon context.
- Group operations must only affect sessions parsed as managed for current project.
- Non-TA sessions are invisible to group operations.

## Session Directory

Project-scoped session metadata directory:

```text
~/.agentteam/sessions/<project_slug>/
```

This directory is runtime-owned. It may store session descriptors, launch evidence, and adapter observation metadata.

It must not store user config. User config remains `~/.agentteam/config.toml`.

## Ready And Status Rules

Ready rule:

- TUI launch returns no framework/transport error.
- Managed TA tmux session exists.
- Then agent is ready.
- Ready projects to `idle` when no task is active.
- If the root manager just received a request and the session is still alive, silence alone does not become `error`; keep the projection `busy` until an idle or fault signal arrives.

Steady status values:

| status | Meaning | Primary owner facts |
|---|---|---|
| `offline` | agent not launched or intentionally stopped | Runtime/Adapter |
| `starting` | launch requested, waiting for TA session confirmation | Runtime/Adapter |
| `idle` | TUI launched without error, TA session alive, no active task or pending request | Adapter + Task Engine |
| `busy` | agent has active assigned/claimed task or an outstanding request/response is still in flight | Task Engine + Adapter |
| `error` | launch/session/transport/framework fault | Adapter + Error Center |

`ready` is a transition fact, not a long-lived steady status.

Generic status truth:

- tmux/zterm session existence
- launch result
- zterm/tmux transport events
- normalized TUI Agent Adapter signals
- Task Engine active task facts
- Error Center framework fault facts

Codex SDK rule:

- Codex SDK can be a future provider-specific diagnostic source.
- Codex SDK must flow through TUI Agent Adapter Center if used.
- Codex SDK must not be generic AgentTeam status truth.
- Non-Codex TUI agents must work with the same status model.

## Error Behavior

Duplicate, invalid, or ambiguous names emit AgentNaming fault facts.

Launch/session/transport faults are owned by Runtime or zterm/tmux Adapter and enter Error Center from that owner.

## Debug Snapshot

Snapshot includes:

- known agents
- manager name
- worker name pool allocation
- role mapping
- capability facts
- allocated names
- projected statuses
- session directory
- parsed managed sessions
- domain-qualified address projections
- rejected non-managed sessions count

## Resource Lifecycle

Agent Registry owns lifecycle requests for:

- `agent_member`
- capability projection
- status projection entry

Rules:

- Register an `agent_member` resource when a member is accepted into a team.
- Release it when the member is removed or the project/team scope is closed.
- Status projection entries must reference current task/resource facts and must not keep unbounded historical buffers.
- An agent member with no matching session/resource heartbeat after startup grace is an orphan/status mismatch candidate.
- Agent count, status projection count, and stale member count are efficiency metrics.

## Red Tests

- Duplicate name fails.
- Invalid name fails.
- Multiple super managers fail in v1.
- Missing or duplicate configured manager fails.
- Worker 1-20 allocation order mismatch fails.
- Worker 21+ not named `<project_slug>_worker_<seq>` fails.
- Status derived only from Codex SDK fails.
- Ready without TA session existence fails.
- Session metadata outside `~/.agentteam/sessions/<project_slug>/` fails.
- Non-TA group operation target fails.
- Wrong project prefix is ignored/rejected.
- tmux adapter inventing names fails architecture gate.
- Agent-facing projection exposing TA session name as identity fails.
- Agent Registry treating local names as globally unique fails architecture gate.
- Agent member without lifecycle lease fails.
- Removed member with active status projection leak fails.

## Open Decisions

- Allowed character set for names.
- Whether work role can differ from agent name.
- Whether naming pool reserves system names like `daemon`, `system`, `debug`.
