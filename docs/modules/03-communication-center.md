# 03 Communication Center

## Purpose

Communication Center is the daemon-owned bidirectional communication center for managers, workers, and other team agents.

It routes commands, task-board queries, task claims, message delivery, ready reports, and task update envelopes. It does not own task state; it coordinates with Task Engine and Persistence.

## Owns

- Role-to-role message routing.
- Broadcast/all-member message routing.
- Task dispatch envelope routing.
- Message durability request to Persistence.
- Delivery status.
- Communication debug snapshot.
- Manager-to-worker command routing.
- Worker-to-daemon ready/status reports.
- Task-board query routing.
- Task claim request routing.
- Priority-aware task pull request routing.
- Agent owner authorization checks through Agent Registry.
- Domain route plan consumption for cross-daemon delivery.

## Does Not Own

- Agent process launch.
- tmux stdin/stdout.
- Task queue state transitions.
- Final output rendering.
- Error classification.
- Task priority ownership.
- Task board materialized state.
- Manager/worker identity ownership.
- Agent ready state ownership.
- Persistence event append implementation.
- TANote append format, note ordering, or thread projection.
- Daemon domain parsing or endpoint resolution.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `comm.route_message` | Communication Center | Route agent-to-agent message | `CommReq01RouteIntent` | `CommResp04DeliveryAccepted` | unknown/ambiguous target |
| `comm.route_cross_domain` | Communication Center + Daemon Domain Registry | Route domain-qualified agent/team/role target through resolved domain plan | `CommReq01RouteIntent` | `CommResp04DeliveryAccepted` | comm parses domain directly |
| `comm.route_broadcast` | Communication Center | Route message to all members in a team | `CommReq01RouteIntent` | `CommResp04DeliveryAccepted` | partial broadcast without error |
| `comm.send_message` | Communication Center + Persistence | Route and persist one message delivery | `CommReq01RouteIntent` | `CommMessageSendResult` | delivery persisted without replayable event |
| `comm.send_broadcast` | Communication Center + Persistence | Route and persist one broadcast delivery | `CommReq11BroadcastIntent` | `CommBroadcastSendResult` | broadcast delivery persisted without replayable event |
| `comm.route_manager_command` | Communication Center | Route manager command to daemon module owner | `CommReq01RouteIntent` | `CommReq03DeliveryEnvelope` | manager bypasses daemon |
| `comm.route_ready_report` | Communication Center | Accept agent ready report and route to registry/runtime owner | `CommReq05ReadyReport` | `CommResp08ReadyAccepted` | ready report mutates registry directly |
| `comm.send_ready_report` | Communication Center + Persistence | Route and persist one ready report delivery | `CommReq05ReadyReport` | `CommReadyReportSendResult` | ready delivery persisted without replayable event |
| `comm.route_task_board_query` | Communication Center | Route task-board read requests to Task Engine projection | `CommReq01TaskBoardQuery` | `CommResp04DeliveryAccepted` | comm owns task board state |
| `comm.route_task_claim` | Communication Center | Route worker claim request to Task Engine | `CommReq01TaskClaim` | `CommResp04DeliveryAccepted` | claim without owner/priority validation |
| `comm.enforce_sender_scope` | Communication Center + Agent Registry | Verify sender identity and capability | `CommReq01RouteIntent` | `CommReq02ResolvedTarget` | unauthorized manager/worker operation |
| `comm.persist_delivery_event` | Communication Center + Persistence | Persist delivery event JSONL record | `CommReq03DeliveryEnvelope` | `PersistResp03AppendReceipt` | delivery not persisted |
| `comm.snapshot` | Communication Center | Provide route/delivery snapshot to Debug Center | internal comm state | `CommDebugSnapshot` | private state leak |
| `comm.help` | Communication Center | Describe message/task-board/claim/ready commands | help topic | rendered help model | hidden wire protocol in help |

## Module Help Contract

Required help topics:

```text
agentteam help comm
agentteam help comm manager
agentteam help comm worker
agentteam help comm ready
agentteam help comm delivery
agentteam help comm message-send
agentteam help comm ready-report
agentteam help comm task-board
agentteam help comm claim
agentteam help comm message
agentteam help comm broadcast
agentteam help comm domains
agentteam help comm red-tests
```

Help content must explain:

- daemon is the communication entry point
- manager queries and manages through daemon
- worker queries task board and claims work through daemon
- workers report `ready` after startup
- task priority is owned by Task Engine, not Communication Center
- every delivery/claim/ready envelope is persisted as event request
- delivery persistence requests go through `comm.persist_delivery_event`
- delivery persistence returns a replayable JSONL event receipt
- `msg send` goes through `comm.send_message`
- `msg broadcast` goes through `comm.send_broadcast`
- `ready report` goes through `comm.send_ready_report`
- message target supports exact agent, role, team, and all members
- broadcast delivery targets all resolved team members and returns a replayable receipt
- cross-daemon targets use domain-qualified addresses resolved by Daemon Domain Registry
- v1 supports one super manager only
- agents use CLI/skill commands, not hidden daemon wire protocol
- tmux delivery is allowed only through agent-visible envelopes with sender, target, action, and content

Help content must not:

- tell manager or worker to edit task state directly
- expose daemon internal wire format as required agent behavior
- suggest fallback delivery paths
- suggest direct tmux writes for task management
- parse daemon domain addresses inside Communication Center

## Public API Boundary

```text
CommReq01RouteIntent -> CommReq02ResolvedTarget -> CommReq03DeliveryEnvelope -> CommResp04DeliveryAccepted
CommReq05ReadyReport -> CommReq06ResolvedAgent -> CommReq07ReadyEnvelope -> CommResp08ReadyAccepted
CommReq05ReadyReport -> CommReq06ResolvedAgent -> CommReq07ReadyEnvelope -> PersistResp03AppendReceipt -> CommReadyReportSendResult
CommReq11BroadcastIntent -> CommReq12ResolvedTeamMembers -> CommReq13BroadcastEnvelope -> CommResp14BroadcastAccepted
CommReq11BroadcastIntent -> CommReq12ResolvedTeamMembers -> CommReq13BroadcastEnvelope -> PersistResp03AppendReceipt -> CommBroadcastSendResult
CommReq21TaskBoardQuery -> CommReq22AuthorizedQuery -> CommReq23TaskBoardQueryEnvelope -> CommResp24TaskBoardQueryAccepted
CommReq31TaskClaim -> CommReq32AuthorizedClaim -> CommReq33TaskClaimEnvelope -> CommResp34TaskClaimAccepted
CommReq01NoteDelivery -> CommReq02ResolvedTarget -> CommReq03AgentVisibleEnvelope -> CommResp04DeliveryAccepted
CommReq01RouteIntent -> DomainRoute04Plan -> CommReq03DeliveryEnvelope -> CommResp04DeliveryAccepted
CommReq03DeliveryEnvelope -> PersistResp03AppendReceipt
CommReq01RouteIntent -> CommMessageSendResult
```

Only Communication Center resolves a target role/member for message delivery.
Only Daemon Domain Registry resolves target daemon domain.
Only Task Engine resolves task priority, claim result, and task state.
Only TANote Collaboration Board validates note blocks and projects threads.

## Required Behavior

- Send message to one role.
- Send message to one exact agent.
- Send message to one team.
- Broadcast message to all members.
- Send message/task/note to a domain-qualified target such as `Alice@review-daemon`.
- Send task dispatch envelope to one role.
- Receive manager commands through daemon.
- Receive worker `ready` reports through daemon.
- Route task-board read requests to Task Engine.
- Route worker task-claim requests to Task Engine.
- Preserve priority request metadata for Task Engine.
- Enforce sender capability:
  - v1 has exactly one super manager
  - super manager can query/manage team task board through daemon
  - worker can query board, claim task, report status, complete/error assigned task
  - unauthorized agent cannot claim another owner-only task
- Persist communication envelopes through Persistence.
- Persist delivery events through `comm.persist_delivery_event`.
- Reject unknown role.
- Reject ambiguous role.
- Reject ambiguous bare agent name when remote domain context is required.
- Reject remote daemon lookup failure explicitly.
- Preserve original message text semantics.
- Never crop or rewrite user-visible payload semantics.
- When a routed message is mirrored into `TANote.md`, call TANote Board for append/projection instead of writing note content directly.
- When delivering through tmux, wrap the visible payload as an AgentTeam envelope containing `from`, `to`, `action`, `task_id` or `thread_id` when present, and body text.

## Manager/Worker Communication Model

```text
manager
  |
  | query/manage task board
  v
daemon Communication Center
  |
  +--> Task Engine: create/update/query/assign/claim
  +--> Agent Registry: check owner/capability
  +--> Persistence: append envelope/event
  +--> worker: deliver assignment/message

worker
  |
  | ready/query/claim/update/done/error
  v
daemon Communication Center
  |
  +--> Task Engine: priority claim + state transition request
  +--> Agent Registry: identity/capability check
  +--> Persistence: append envelope/event
```

## Task Board Interaction

- Manager publishes or changes tasks through daemon.
- Manager may assign task owner.
- Worker queries task board through daemon.
- Worker claim request asks Task Engine for the highest-priority available task it is allowed to claim.
- Worker claim eligibility is limited to:
  - tasks assigned to that worker
  - tasks matching that worker role
- Claim ordering is Task Engine truth:
  - assigned tasks outrank role-matching open tasks
  - blocked tasks outrank non-blocked tasks inside the same claim class
- Priority ordering is Task Engine truth.
- Blocking task handling is Task Engine truth.
- Communication Center only routes request/response envelopes.

## Cross-Daemon Targeting

Communication Center accepts domain-qualified targets only after Input Gateway parsing:

```text
agent:<name>@<domain>
role:<role>@<domain>
team:<team_id>@<domain>
all@<domain>
```

For plain `Alice`, Communication Center requests Daemon Domain Registry resolution with local-domain context.

For `Alice@review-daemon`, Communication Center requests a remote route plan and then builds the business envelope for that plan.

Communication Center must not parse or validate daemon domain ids directly.

## Agent Ready Flow

- Agent starts under tmux/zterm adapter.
- Agent reports ready through CLI/skill.
- Communication Center accepts ready report.
- Agent Registry/Runtime owns materialized ready status.
- Manager can start task management after ready appears in projected state.

## Error Behavior

Unknown, ambiguous, unauthorized, or malformed route emits a CommunicationRoute fault fact to Error Center.

Agent task execution failure reported by `agentteam task error` is a Task Engine state update, not Communication Center error.

Agent process/session/framework fault is routed to Error Center by the owning runtime/adapter module.

Malformed note references or note append failures are TANote fault facts. Communication Center must not convert them into successful delivery.

## Debug Snapshot

Snapshot includes:

- active routes
- pending delivery envelopes
- latest delivery results
- blocked routes
- manager command envelopes
- worker ready reports
- claim request envelopes
- domain route plan ids
- delivery persistence receipts
- persisted delivery receipts
- linked TANote note ids/thread ids, without parsing note body

## Resource Lifecycle

Communication Center owns lifecycle requests for:

- `message_envelope`
- remote domain route handle
- delivery queue entries
- pending broadcast fanout entries
- claim/query envelopes

Rules:

- Register each pending envelope with Resource Lifecycle Manager before delivery work begins.
- Release envelope resource after delivered, failed, or explicitly rejected event receipt exists.
- Broadcast fanout must release every recipient entry; partial release without explicit failure event is invalid.
- Cross-daemon route handles must release after accepted delivery or classified failure event receipt.
- A pending envelope whose target agent is gone becomes an orphan candidate with `last_event_id` and `evidence_id`.
- Queue length, oldest pending envelope age, and delivery retry count are efficiency metrics.

## Red Tests

- Unknown role fails.
- Duplicate target role fails.
- Broadcast partial delivery without explicit error fails.
- Unauthorized sender fails.
- More than one super manager in v1 fails.
- Manager direct task-state mutation through Communication Center fails.
- Communication Center owning task priority fails architecture gate.
- Worker claim without Task Engine decision fails.
- Ready report directly mutating registry state fails architecture gate.
- Ready report delivery without persistence event request fails.
- Delivery without persistence event request fails.
- Payload semantic rewrite fails.
- Adapter-owned routing fails architecture gate.
- Communication Center parsing domain address directly fails architecture gate.
- Cross-daemon lookup failure falling back to local delivery fails.
- Communication Center writing `TANote.md` directly fails architecture gate.
- Raw tmux message without AgentTeam envelope fails.
- Pending envelope without lifecycle lease fails.
- Broadcast fanout entry leak fails.

## Current Decisions

- Broadcast is required.
- Message target supports exact agent, role, team, and all members.
- v1 supports one super manager only.
- Multi-manager support is future expansion.
- Worker claim visibility is only assigned tasks and role-matching tasks.
- Task Engine owns assigned-first and blocked-first claim ordering.
- Communication Center and Task Engine remain separate modules.
