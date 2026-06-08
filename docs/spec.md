# AgentTeam Specification Draft

## Product Goal

AgentTeam is a Rust daemon + CLI framework for launching and coordinating multiple TUI agents under tmux.

The system lets a team contain multiple named agents, send work to roles, inspect status, exchange messages, render one selected agent, and persist/debug the full interaction flow.

## Non-Negotiable Requirements

- Rust implementation.
- tmux-based TUI agent process model.
- zterm-compatible daemon/mirror integration for terminal communication/render.
- TA naming rule for managed agents.
- Independent module boundaries:
  - Config Center
  - Error Center
  - Communication Center
  - Input Gateway
  - Output Gateway
  - UI Gateway
  - Debug Center
  - Agent Registry/Naming Pool
  - Team Orchestrator
  - Task Engine
  - Persistence Event Log
  - zterm/tmux Adapter
  - TUI Agent Adapter Center
  - Startup and Session Manager
  - TANote Collaboration Board
  - Resource Lifecycle Manager
- `~/.agentteam/config.toml` with comments.
- Config stores project-related user configuration only.
- Shared functions/contracts in shared blocks only.
- Pure orchestration in Team Orchestrator.
- Module API isolation.
- Red tests for every critical feature.
- Detailed ASCII flow docs.
- Build must include regression tests.
- Required files must be tracked before build is accepted.
- tmux stdout alone is not final status truth; provider TUI adapters normalize richer status signals.
- WebUI/UI only consumes input/output projections and stays decoupled from agent framework internals.
- tmux/session identifiers are internal and invisible to agents; agents use CLI/skill operations by name, role, task, and message.
- Kevin reads AgentTeam skill and uses CLI to initialize framework, query tasks, publish tasks, communicate with child agents, and wait for projected results.
- Every agent writes work notes through `agentteam note post`, producing a project `TANote.md` forum-style collaboration projection.
- Tmux communication payloads must use an agent-visible AgentTeam envelope with `from`, `to`, `action`, and content.
- Every module-owned resource must have a lifecycle lease, owner, scope, release policy, orphan/leak detection, debug snapshot, and efficiency budget.
- MVP debug must include enough evidence to trace errors, tasks, messages, agents, terminal observations, TANote threads, and resources back to persisted events.
- All debug capture is persisted in v1; no print-only debug bundle path exists.
- Daemon/session close must cleanup scoped resources and tracked temporary files through exact handles.
- v1 does not enforce aggressive memory hard caps, but queues, cursors, buffers, temp files, and projections must not grow unbounded.

## Managed Agent Naming

```text
TA_<domain_id>_<project_slug>_<agent_name>
```

Examples:

```text
TA_agentteam_Kevin
TA_agentteam_Alice
TA_agentteam_Bob
```

Only matching sessions may be controlled as a group. These are internal framework identifiers; agents use `Kevin`, `Alice`, or `Bob`, not TA session names.

## v1 User Workflows

1. Configure project/team/agents in `~/.agentteam/config.toml`.
2. Start AgentTeam daemon.
3. Run `agentteam startup init` from current TUI to initialize Kevin.
4. Kevin uses CLI/skill to spawn workers in scoped tmux sessions.
5. Send task to a role or named agent.
6. Agent checks own task list through CLI.
7. Agent reports task done/error through CLI.
8. Agents send messages to other roles through CLI.
9. Agents post/read `TANote.md` threads to discuss work, cite evidence, and hand off context.
10. User renders one selected agent.
11. User captures debug snapshot.
12. Runtime projects agent status from tmux/zterm facts, TUI adapter signals, task facts, and error facts.
13. User inspects resource lifecycle/debug snapshot to find orphaned sessions, stale adapters, leaked handles, or budget overruns.
14. User closes daemon/session and Resource Lifecycle Manager releases scoped resources and temporary files with persisted cleanup receipts.

## v1 Excluded Until Discussed

- Cloud sync.
- Remote relay.
- Full DAG workflow.
- Autonomous task planning beyond queued tasks.
- Non-tmux process backend.
- Copying zterm daemon source.
- Provider SDK as universal status truth.
- Kevin as durable persistence truth.
- Direct tmux launch outside zterm/tmux Adapter.
- Treating `TANote.md` as task/message/event truth.
- Resource cleanup without exact handle, owner receipt, and event receipt.
- MVP debug bundle without resource lifecycle information.
- Debug output without persisted bundle/evidence receipt.
- Unbounded resource growth without cleanup/drain visibility.

## Discussion Method

Use `docs/modules/00-module-discussion-index.md`.

Discuss modules one by one. After each module is accepted:

- update the module doc
- update function map
- update verification map
- update red-test plan
- only then consider code

## MVP Start Decisions

See `docs/architecture/mvp-start-gate.md`.
