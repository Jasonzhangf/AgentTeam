# 16 TANote Collaboration Board

## Purpose

TANote Collaboration Board owns the project `TANote.md` collaboration format, ordered note append protocol, forum-style thread projection, and agent-visible message envelope format.

Every agent writes work notes during execution through AgentTeam CLI. The resulting `TANote.md` lets agents read each other's context like a forum while keeping task/message/event truth inside daemon-owned modules.

## Owns

- `TANote.md` block format.
- Note id and sequence allocation.
- Thread id and reply relation validation.
- Agent note append validation.
- Agent-visible envelope rendering for note/message delivery.
- Note-to-task/message/reference metadata validation.
- TANote debug snapshot.
- TANote help text.

## Does Not Own

- Task state transitions.
- Communication route resolution.
- tmux stdin/stdout transport.
- Error classification.
- Persistence event append implementation.
- Agent identity/capability truth.
- UI/WebUI final rendering.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `tanote.post` | TANote Board | Validate note intent and allocate note/thread ids | `TANoteReq01PostIntent` | `TANoteReq03ValidatedNote` | missing from/to/action |
| `tanote.append_event` | TANote Board + Persistence | Request durable note event before projection | `TANoteReq03ValidatedNote` | persistence append receipt | projection without event |
| `tanote.project_file` | TANote Board | Materialize ordered `TANote.md` projection | note event stream | `TANoteResp05FileProjection` | duplicate sequence/id |
| `tanote.project_thread` | TANote Board | Project one thread for CLI/UI/agent reading | thread id | `TANoteResp05ThreadProjection` | orphan reply accepted |
| `tanote.build_envelope` | TANote Board | Build agent-visible from/to/action envelope for tmux delivery | validated note/message | `TANoteResp04AgentEnvelope` | raw tmux payload |
| `tanote.verify_file` | TANote Board | Verify `TANote.md` block syntax and sequence continuity | file projection | verification result | manual edit accepted |
| `tanote.snapshot` | TANote Board | Provide note/thread snapshot to Debug Center | snapshot request | `TANoteDebugSnapshot` | private state leak |
| `tanote.help` | TANote Board | Describe note post/read/thread/envelope rules | help topic | help model | help says notes are task truth |

## Module Help Contract

Required help topics:

```text
agentteam help note
agentteam help note post
agentteam help note thread
agentteam help note format
agentteam help note envelope
agentteam help note red-tests
```

Help content must explain:

- agents post notes through CLI, not direct file edits
- `TANote.md` is a forum-style collaboration projection
- every note has `from`, `to`, `action`, `thread_id`, `note_id`, and ordered sequence
- notes can reference tasks/messages/events/evidence
- notes cannot mutate task state by themselves
- tmux delivery uses an agent-visible envelope, not tmux ids or hidden wire protocol
- agents can read `TANote.md` and query projected threads

Help content must not:

- say `TANote.md` is task/message/event truth
- tell agents to manually append to the file
- expose tmux session names, pane ids, zterm endpoints, or session descriptor paths
- allow raw tmux text without an AgentTeam envelope
- suggest fallback note writes when daemon append fails

## Public API Boundary

```text
TANoteReq01PostIntent -> TANoteReq02ParsedBlock -> TANoteReq03ValidatedNote -> TANoteResp04AgentEnvelope -> TANoteResp05FileProjection
TANoteReq01ThreadQuery -> TANoteReq02ThreadSelector -> TANoteResp05ThreadProjection
TANoteErr01Malformed -> TANoteErr02Rejected -> TeamErr01FaultFact
```

Only TANote Board validates note format and projects `TANote.md`.

Only Persistence appends durable note events.

Only Communication Center resolves delivery targets.

Only Task Engine changes task state.

## File Location Contract

Project note projection:

```text
<project_root>/TANote.md
```

Runtime metadata and events remain under AgentTeam runtime home, not inside `TANote.md`.

The file is a materialized projection and may be read by agents. Agent writes must go through:

```text
agentteam note post ...
```

Direct manual edits are invalid because they lack daemon sequence, event receipt, and validation.

## TANote Block Format

Each append is one block. Blocks are append-only.

````text
<!-- TANOTE:BEGIN v1 -->
### TN-20260608T120102Z-000001

```toml
version = 1
note_id = "TN-20260608T120102Z-000001"
thread_id = "TH-20260608T120000Z-000001"
seq = 1
time = "2026-06-08T12:01:02Z"
from = "Kevin"
to = ["agent:Alice"]
action = "ask"
team_id = "default"
task_id = "AT-1"
message_id = "MSG-1"
reply_to = ""
visibility = "team"
delivery = ["tanote", "tmux"]
event_id = "EVT-20260608T120102Z-000001"
evidence_id = ""
```

Body:

Please inspect the config module and reply in this thread.

<!-- TANOTE:END TN-20260608T120102Z-000001 -->
````

Required fields:

- `version`
- `note_id`
- `thread_id`
- `seq`
- `time`
- `from`
- `to`
- `action`
- `team_id`
- `visibility`
- `delivery`
- `event_id`

Optional fields:

- `task_id`
- `message_id`
- `reply_to`
- `evidence_id`

Allowed target forms:

```text
agent:<name>
role:<role>
team:<team_id>
all
```

Initial action set:

```text
ask
reply
proposal
decision
status
blocker
evidence
handoff
announce
task_reference
```

Rules:

- `seq` is monotonically increasing per project.
- `note_id` is unique.
- `reply_to` must point to an existing note in the same thread.
- `from` must be a registered agent name.
- `to` must resolve through Communication Center when delivery is requested.
- `action` describes discussion intent only.
- Task state changes require Task Engine commands such as `agentteam task done` or `agentteam task error`.
- Corrections are new notes with `action = "reply"` or `action = "decision"`; old blocks are not edited.

## Agent-Visible Tmux Envelope

When a note/message is delivered through tmux, the target agent sees a visible semantic envelope:

```text
[AgentTeamEnvelope v1]
from: Kevin
to: agent:Alice
action: ask
team_id: default
thread_id: TH-20260608T120000Z-000001
note_id: TN-20260608T120102Z-000001
task_id: AT-1

Please inspect the config module and reply in this thread.
[/AgentTeamEnvelope]
```

This envelope is the only agent-facing communication shape for tmux delivery.

Forbidden in the envelope:

- tmux session names
- pane ids
- session descriptor paths
- zterm endpoints
- daemon internal wire fields

## Required Behavior

- Create a project `TANote.md` projection when the first note event is accepted.
- Append one validated block per note event.
- Preserve body semantics without cropping or rewriting.
- Let multiple agents post interleaved notes through daemon-serialized sequence order.
- Support thread query by `thread_id`.
- Support latest-note query by agent/team/task.
- Support note references from task/message/debug projections.
- Build tmux delivery envelopes with explicit `from`, `to`, and `action`.
- Reject malformed, duplicate, unauthorized, or out-of-order note blocks.
- Reject any path where a note mutates task state without Task Engine.

## Error Behavior

Malformed note intents emit TANote fault facts to Error Center.

Examples:

- missing sender
- unknown target
- invalid action
- duplicate note id
- non-monotonic sequence
- orphan reply
- missing persistence receipt
- projection write failure
- raw tmux delivery without envelope

All TANote errors must persist to event log through Error Center.

## Debug Snapshot

Snapshot includes:

- TANote projection path
- latest sequence
- latest note id
- active thread ids
- note count by agent
- unresolved references
- projection verification status
- latest persistence receipt

Snapshot must not include hidden tmux/session identifiers.

## Resource Lifecycle

TANote Board owns lifecycle requests for:

- `tanote_projection`
- note append projection handle
- thread projection cursor

Rules:

- Register projection handle before materializing `TANote.md`.
- Release append handle after projection materialization result is persisted.
- Thread projection cursors must be bounded and released after CLI/UI request finishes.
- A projection file with no matching latest note event is a corruption/orphan candidate.
- Note count, projection bytes, thread cursor count, and latest projection latency are efficiency metrics.

## Persistence Behavior

TANote Board requests Persistence events for:

- note append requested
- note append accepted/rejected
- file projection materialized
- thread projection requested
- envelope delivery requested
- verification failure

`TANote.md` projection is accepted only after the matching note event append receipt exists.

## Red Tests

- Missing `from`, `to`, or `action` fails.
- Unknown sender fails.
- Unknown target fails.
- Invalid target form fails.
- Invalid action fails.
- Duplicate `note_id` fails.
- Non-monotonic `seq` fails.
- Orphan `reply_to` fails.
- Manual direct edit accepted as truth fails.
- Note body semantic crop/rewrite fails.
- `TANote.md` entry mutating task state fails.
- Projection without persistence event fails.
- Communication Center writing `TANote.md` directly fails.
- Raw tmux payload without AgentTeam envelope fails.
- Envelope exposing tmux/session/zterm internals fails.
- TANote projection without lifecycle lease fails.
- Thread projection cursor leak fails.

## Open Decisions

- Whether `agentteam task send` automatically creates a TANote thread.
- Whether note body supports attachments by path or only evidence ids in v1.
- Whether agents can request private note visibility, or v1 uses team-visible notes only.
