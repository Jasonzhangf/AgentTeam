# ASCII Flows

## 1. Top-Level Runtime

```text
                         +----------------------+
                         |      config.toml      |
                         +----------+-----------+
                                    |
                                    v
                         +----------------------+
                         |    Config Center     |
                         | parse/normalize only |
                         +----------+-----------+
                                    |
                                    v
+-------------+       +------------+-------------+       +------------------+
| CLI / Skill | ----> |        Input Gateway      | ----> |  Runtime Core    |
| WebUI / UI  |       | parse external commands   |       | pure orchestration|
+------+------+       +------------+-------------+       +---+----------+---+
       ^                           |                         |          |
       |                           v                         |          v
       |                +----------+-----------+             |  +-------+------+
       |                |      Error Center    | <-----------+  | Task Engine  |
       |                | one error owner      |                +-------+------+
       |                +----------+-----------+                        |
       |                           |                                    v
       |                           v                           +--------+-------+
       |                +----------+-----------+               | Communication |
       |                |      Output Gateway  | <-------------+ Center        |
       |                | render responses     |               +---+--------+--+
       |                +----------+-----------+                   |        |
       |                           |                               |        v
       +---------------------------+                               | +------+------+
                                                                   | | Persistence |
                                                                   | | Event Log   |
                                                                   | +------+------+
                                                                   |        |
                                                                   v        v
                                                           +-------+--------+
                                                           | Debug Center   |
                                                           | snapshots      |
                                                           +----------------+
```

## 1.2 Daemon Domain Boundary

```text
zterm daemon A                         zterm daemon B
domain: local                          domain: review-daemon
      |                                      |
      v                                      v
+-------------------+                +-------------------+
| AgentTeam daemon  |                | AgentTeam daemon  |
| local domain      |                | remote domain     |
+---------+---------+                +---------+---------+
          |                                    ^
          v                                    |
+---------+---------+      DomainRoute04Plan  |
| Daemon Domain     | ------------------------+
| Registry          |
+---------+---------+
          |
          v
agent address truth:
agent@domain
```

Rules:

- Daemon domain is the cross-daemon naming boundary.
- Local agent name is unique only inside one daemon domain/team scope.
- Cross-daemon target must be domain-qualified, for example `Alice@review-daemon`.
- Communication Center asks Daemon Domain Registry for route plans; it does not parse domains itself.
- zterm/tmux Adapter consumes resolved endpoint facts; it does not own business target resolution.

## 1.1 UI/WebUI Boundary

```text
UI / WebUI
  |
  | user action
  v
+-----------------------+
| Input Gateway         |
| raw UI input -> intent|
+-----------+-----------+
            |
            v
+-----------------------+
| Runtime / Modules     |
| framework truth       |
+-----------+-----------+
            |
            v
+-----------------------+
| Output Gateway        |
| projection only       |
+-----------+-----------+
            |
            v
UI / WebUI
  |
  | ephemeral view state only
  v
no agent framework state ownership
```

Forbidden:

```text
UI/WebUI -> Task Engine direct
UI/WebUI -> Communication Center direct
UI/WebUI -> Persistence direct
UI/WebUI -> tmux/zterm direct
UI/WebUI -> private module state
```

## 2. Agent Task Dispatch

```text
User/Agent CLI
    |
    v
+-----------------------+
| TeamReq01CliRaw       |
+-----------+-----------+
            |
            v
+-----------------------+
| TeamReq02ParsedCommand|
+-----------+-----------+
            |
            v
+-----------------------+
| TeamReq03ValidatedIntent
+-----------+-----------+
            |
            v
+-----------------------+
| TeamReq04DaemonCommand|
+-----------+-----------+
            |
            v
+-----------------------+       +--------------------+
| Team Orchestrator     | ----> | Agent Registry     |
| pure routing          |       | domain-local names  |
+-----------+-----------+       +--------------------+
            |
            v
+-----------------------+
| Task Engine           |
| per-role queue        |
+-----------+-----------+
            |
            v
+-----------------------+
| Communication Center  |
| target route request  |
+-----------+-----------+
            |
            v
+-----------------------+
| Daemon Domain Registry|
| resolve agent@domain  |
+-----------+-----------+
            |
            v
+-----------------------+
| Input Gateway         |
| build terminal input  |
+-----------+-----------+
            |
            v
+-----------------------+
| zterm/tmux Adapter    |
| send stdin to TA pane |
+-----------+-----------+
            |
            v
+-----------------------+
| TUI Agent Process     |
+-----------------------+
```

## 2.1 Manager Bootstrap And Worker Spawn

```text
Current TUI / human
    |
    | agentteam start (defaults to cwd scope)
    v
+-----------------------+
| Input Gateway         |
+-----------+-----------+
            |
            v
+-----------------------+
| Startup Session Mgr   |
| bootstrap plan owner  |
+-----+-----------+-----+
      |           |
      v           v
+-----+----+  +---+----------------+
| Config   |  | Agent Registry     |
| Center   |  | manager + names    |
+-----+----+  +---+----------------+
      |           |
      v           v
+-----+-----------+-----+
| Persistence request   |
| bootstrap events      |
+-----+-----------+-----+
      |
      v
+-----+-----------------+
| zterm/tmux Adapter    |
| launch TA sessions    |
+-----+-----------------+
      |
      v
+-----+-----------------+
| Manager ready         |
| worker launch later   |
+-----------------------+
```

Role control path:

```text
Manager startup params -> configured name/role/team/project scope
Manager skills -> manager behavior and worker initialization
Worker startup params -> worker name/role/team/project scope
Worker skills -> ready/claim/done/error/note control
Manager -> task/message/broadcast/debug control
tmux -> transparent transport carrier, hidden from agents
```

## 2.2 Single-Agent Control Plane

```text
User / Manager / Runtime
    |
    | typed control intent
    v
+---------------------------+
| Input Gateway             |
| control intent parse      |
+-------------+-------------+
              |
              v
+---------------------------+
| Agent Control Center      |
| mode select + bind        |
+------+------+-------------+
       |      |
       |      | headless
       |      v
       |  +----------------------+
       |  | SDK Bridge Process   |
       |  | live client/thread    |
       |  +----------+-----------+
       |             |
       |             v
       |  +----------------------+
       |  | Headless Agent       |
       |  | session/control loop  |
       |  +----------+-----------+
       |             |
       |             v
       |  +----------------------+
       |  | Output Gateway       |
       |  | status/projection    |
       |  +----------------------+
       |
       | attach_tui
       v
 +----------------------+
 | zterm/tmux Adapter   |
 | stdin/stdout carrier  |
 +----------+-----------+
            |
            v
 +----------------------+
 | Visible TUI Agent    |
 | tmux session/pane    |
 +----------+-----------+
            |
            v
 +----------------------+
 | Output Gateway       |
 | status/projection    |
 +----------------------+
```

Rules:

- mode selection is explicit
- attach_tui and headless are both first-class
- control-plane output is projected, not guessed from raw pane text alone
- tmux session details stay hidden from agent-facing layers
- SDK control is the automatic lane; tmux control remains the human-observable lane

Forbidden:

```text
Manager -> event log direct
Manager -> tmux direct
Startup Manager -> tmux direct
Startup Manager -> state file direct
Worker -> hidden daemon wire
```

## 3. Terminal Output Capture

```text
+-----------------------+
| TUI Agent Process     |
+-----------+-----------+
            |
            v
+-----------------------+
| tmux pane truth       |
+-----------+-----------+
            |
            v
+-----------------------+
| zterm daemon/mirror   |
+-----------+-----------+
            |
            v
+-----------------------+
| zterm/tmux Adapter    |
| buffer head/sync      |
+-----------+-----------+
            |
            v
+-----------------------+
| Output Gateway        |
| normalize observation |
+-----------+-----------+
            |
            v
+-----------------------+
| Communication Center  |
| correlate with task   |
+-----------+-----------+
            |
            v
+-----------------------+
| Task Engine           |
| update task state     |
+-----------+-----------+
            |
            v
+-----------------------+
| Persistence Event Log |
+-----------------------+
```

## 4. Error Chain

```text
Module fault
    |
    v
+-----------------------+
| TeamErr01FaultFact    |
| raw local evidence    |
+-----------+-----------+
            |
            v
+-----------------------+
| TeamErr02Classified   |
| Error Center only     |
+-----------+-----------+
            |
            v
+-----------------------+
| TeamErr03RuntimeEvent |
| durable event         |
+-----------+-----------+
            |
            v
+-----------------------+
| TeamErr04Projection   |
| CLI/UI/API response   |
+-----------------------+
```

Rules:

- No module renders final error text except Output Gateway.
- No module converts an error into success.
- No retry/fallback path unless explicitly designed as first-class state, not compensation.

## 5. Debug Snapshot

```text
CLI: agentteam debug snapshot
    |
    v
+------------------------+
| Debug Center           |
+----+----+----+----+----+
     |    |    |    |
     v    v    v    v
 Config Error Comm Task ...
 Snapshot APIs only
     |    |    |    |
     +----+----+----+
          |
          v
+------------------------+
| Debug Bundle           |
| redacted, typed, saved |
+------------------------+
```

Debug Center may request snapshots. It must not inspect module private fields.

## 6. Group Operation Safety

```text
All tmux sessions
    |
    v
+---------------------------+
| Agent Naming Pool         |
| filter TA_<project>_*     |
+-------------+-------------+
              |
              v
+---------------------------+
| Explicit target set       |
| no non-TA sessions        |
+-------------+-------------+
              |
              v
+---------------------------+
| tmux/zterm Adapter        |
| scoped operation only     |
+---------------------------+
```

## 6.1 Cross-Daemon Communication

```text
Kevin@local
    |
    | msg send --to Alice@review-daemon
    v
+-----------------------+
| Input Gateway         |
+-----------+-----------+
            |
            v
+-----------------------+
| Communication Center  |
| business envelope     |
+-----------+-----------+
            |
            v
+-----------------------+
| Daemon Domain Registry|
| parse/resolve domain  |
+-----------+-----------+
            |
            v
+-----------------------+
| DomainRoute04Plan     |
| remote daemon endpoint|
+-----------+-----------+
            |
            v
+-----------------------+
| zterm/tmux Adapter or |
| daemon client adapter |
| transport only        |
+-----------+-----------+
            |
            v
Alice@review-daemon
```

Forbidden:

```text
Communication Center -> parse domain string directly
Agent Registry -> global unique name allocation
zterm/tmux Adapter -> decide business target domain
remote lookup failure -> local fallback delivery
```

## 7. TANote Forum And Tmux Delivery

```text
-----------------------+
| Agent CLI / Skill     |
| note post / msg send  |
+-----------+-----------+
            |
            v
+-----------------------+
| Input Gateway         |
| raw input -> intent   |
+-----------+-----------+
            |
            v
+-----------------------+       +----------------------+
| Communication Center  | ----> | TANote Board         |
| route target only     |       | format + note order  |
+-----------+-----------+       +----------+-----------+
            |                              |
            |                              v
            |                   +----------+-----------+
            |                   | Persistence Event Log |
            |                   | append note event     |
            |                   +----------+-----------+
            |                              |
            |                              v
            |                   +----------+-----------+
            |                   | TANote.md Projection  |
            |                   | forum-style thread    |
            |                   +----------+-----------+
            |                              |
            v                              v
+-----------+-----------+       +----------+-----------+
| Input Gateway         | <---- | Agent-facing envelope |
| typed input op        |       | from/to/action/body   |
+-----------+-----------+       +----------+-----------+
            |
            v
+-----------------------+
| zterm/tmux Adapter    |
| physical transport    |
+-----------+-----------+
            |
            v
+-----------------------+
| Target TUI Agent      |
| sees no tmux details  |
+-----------------------+
```

Rules:

- `TANote.md` is an agent-readable collaboration projection, not task/message/event truth.
- Agents append through `agentteam note post`; the daemon assigns note ids and serial order.
- Tmux is only physical delivery. The visible payload is an AgentTeam envelope with `from`, `to`, `action`, and content.
- A note can reference task/message/event ids, but cannot complete or mutate a task by itself.

## 8. Resource Lifecycle And Orphan Guard

```text
Owner module
  |
  | acquire resource handle
  v
+-----------------------------+
| Resource Lifecycle Manager  |
| lease + owner + budget      |
+-------------+---------------+
              |
              v
+-------------+---------------+
| Persistence Event Log       |
| acquire/use/release events  |
+-------------+---------------+
              |
              v
+-------------+---------------+
| Owner module uses resource  |
| no cross-owner mutation     |
+-------------+---------------+
              |
              v
+-------------+---------------+
| Resource heartbeat/metrics  |
| ttl/refcount/bytes/handles  |
+-------------+---------------+
              |
              v
+-------------+---------------+
| Release or orphan scan      |
| explicit outcome event      |
+------+------+---------------+
       |      |
       v      v
+------+------+--+     +----------------------+
| ReleaseConfirmed |   | Error Center         |
| no leaked handle |   | leak/orphan fault    |
+-----------------+   +----------+-----------+
                                |
                                v
                      +---------+------------+
                      | Debug Center         |
                      | resource snapshot    |
                      +----------------------+
```

Rules:

- Every long-lived resource has `resource_id`, `owner_module`, `resource_class`, `scope`, `lease_id`, `created_at`, `last_seen_at`, and release policy.
- Owner modules request acquire/release; Resource Lifecycle Manager records and audits lifecycle.
- Orphan detection never becomes silent success. It emits event evidence and, when required, an Error Center fault.
- Cleanup uses explicit resource handles only. Broad process kill or broad file deletion is forbidden.
