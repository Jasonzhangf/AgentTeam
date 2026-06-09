# 13 CLI Agent Skill

## Purpose

The CLI and local skill are the agent-facing operating surface.

For a full role-by-role usage guide, read [docs/usage/agentteam-usage.md](../usage/agentteam-usage.md) together with this module.

## Owns

- Human/agent command examples.
- CLI command mapping.
- Skill usage instructions.
- No hidden protocol requirement for agents.
- No tmux/session detail exposure to agents.
- configured manager framework-operation guidance.
- Domain-qualified addressing examples for cross-daemon communication.

## Does Not Own

- Runtime state.
- Error classification.
- Terminal transport.
- Config parsing.
- tmux/session identity handling.
- Daemon domain resolution.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `cli.map_command` | CLI/Skill + Input Gateway | Map agent/human CLI command to raw input | argv | `TeamReq01CliRaw` | CLI bypasses Input Gateway |
| `cli.render_result` | CLI/Skill + Output Gateway | Display output produced by Output Gateway | rendered response | process output | direct module rendering |
| `skill.describe_runtime_ops` | CLI/Skill | Teach agents team/task/message/debug commands | skill read | agent guidance | hidden wire dependency |
| `skill.describe_domain_ops` | CLI/Skill + Daemon Domain Registry | Teach agents `agent@domain` cross-daemon addressing | skill/help text | agent guidance | hidden endpoint dependency |
| `skill.describe_manager_ops` | CLI/Skill | Teach configured manager bootstrap, task publish, wait, debug | skill read | manager guidance | missing manager ops |
| `skill.describe_worker_ops` | CLI/Skill | Teach workers ready/query/claim/done/error/note flow | skill read | worker guidance | worker direct tmux |
| `skill.hide_internals` | CLI/Skill | Keep tmux/session/zterm internals out of agent docs | skill/help text | agent-safe docs | internals exposed |
| `cli.snapshot` | CLI/Skill | Provide CLI command/help snapshot to Debug Center if needed | CLI state | CLI snapshot | durable handle retained |
| `cli.help` | CLI/Skill | Describe agent-facing CLI commands | help topic | help model | broad kill instruction |

## Module Help Contract

Required help topics:

```text
agentteam help cli
agentteam help agent-skill
agentteam help manager
agentteam help worker
agentteam help task
agentteam help msg
agentteam help message
agentteam help msg broadcast
agentteam help ready report
agentteam help domain
agentteam help note
agentteam help debug
agentteam help agent-control
agentteam help cli red-tests
```

Help content must explain:

- agents use CLI/skills only
- the configured manager initializes, publishes tasks, messages workers, waits by projections, and asks Debug Center for evidence
- `msg send` is the CLI surface for agent-to-agent delivery
- `msg broadcast` is the CLI surface for team-wide delivery
- `ready report` is the CLI surface for worker ready delivery
- workers report ready, query/claim tasks, update/done/error tasks, and write notes
- agents communicate by names, roles, tasks, messages, notes, and projections
- cross-daemon communication uses domain-qualified targets such as `Alice@review-daemon`
- `agentteam-dev` is for repository module development, not runtime team communication

Help content must not:

- depend on hidden daemon wire protocol
- must not expose tmux session names, pane ids, descriptor paths, or zterm endpoints
- expose daemon endpoint/auth details in agent-facing examples
- teach manual `TANote.md` edits
- teach direct cleanup, broad process kill, or direct state-file writes

Role execution model:

- startup parameters assign each agent name, role, team, and project scope
- skills define how the configured manager, workers, and operators should act after launch
- the manager uses CLI commands to publish tasks, send messages, broadcast, wait for status, and request debug evidence
- the manager initializes workers after launch using skills/CLI and worker startup params
- workers use CLI commands to report ready, claim work, post notes, and report done/error
- operators use `agentteam-dev` for repository changes and gates, not runtime team control

When the manager sends a request and the session stays alive, silence is treated as `busy`/pending, not `error`. The manager waits on task, message, ready, and debug projections instead of forcing a semantic reply out of tmux output.

The manager must also have a skill-defined CLI feedback path that returns execution results back to the framework. That feedback path is part of the completion loop, not an out-of-band tmux convention.

The current real bootstrap path is `agentteam start`. It starts the configured manager from the current `cwd` by default and expands into the standard tmux bootstrap carrier for that manager. The sample config names this manager `Kevin`.

## Public CLI Surface Draft

Implemented local MVP commands:

```text
agentteam config check --config <path> --json
agentteam daemon check --config <path> --json
agentteam domain resolve --target <target> --config <path> --json
agentteam debug snapshot --config <path> --runtime-home <runtime_home> --json
agentteam start [--cwd <path>] [--config <path>] [--team <team_id>] [--json]
agentteam control attach --agent <name> --team <team_id> --json
agentteam control headless --agent <name> --team <team_id> --json
# headless requires AGENTTEAM_CODEX_SDK_SRC and AGENTTEAM_CODEX_BIN in the environment.
# It starts one persistent Codex SDK bridge for the scoped headless session.
agentteam control headless-run --agent <name> --team <team_id> --input <prompt> --json
agentteam control headless-status --agent <name> --team <team_id> --json
agentteam control headless-interrupt --agent <name> --team <team_id> --json
agentteam control headless-stop --agent <name> --team <team_id> --json
agentteam control send --agent <name> --team <team_id> --input <text> --json
agentteam control observe --agent <name> --team <team_id> --json
agentteam control pause --agent <name> --team <team_id> --json
agentteam control stop --agent <name> --team <team_id> --json
agentteam control wait --agent <name> --team <team_id> --json
agentteam control retry --agent <name> --team <team_id> --task <task_id> --json
agentteam control status --agent <name> --team <team_id> --json

agentteam task send --runtime-home <runtime_home> --team <team_id> --created-by <agent> --target-kind <agent|role> --target <name_or_role> --title <title> --body <text> --json
agentteam task list --runtime-home <runtime_home> --json
agentteam task claim --runtime-home <runtime_home> --worker-name <agent> --worker-role <role> --json
agentteam task status --runtime-home <runtime_home> --task <task_id> --json
agentteam task done --runtime-home <runtime_home> --task <task_id> --actor <agent> --detail <text> --json
agentteam task error --runtime-home <runtime_home> --task <task_id> --actor <agent> --detail <text> --json

agentteam ready report --runtime-home <runtime_home> --sender <name> --team <team_id> --agent-name <name> --body <text> --json
agentteam msg send --runtime-home <runtime_home> --from <name> --to <target> --action <action> --body <text> --json
agentteam msg broadcast --runtime-home <runtime_home> --sender <name> --team <team_id> --action <action> --body <text> --members <comma_separated_members> --json
```

Planned daemon/team commands:

```text
agentteam daemon start
agentteam daemon status
agentteam daemon stop --pid <pid>

agentteam start
agentteam start status

agentteam team list
agentteam team create --id <team_id>

agentteam agent list --team <team_id>
agentteam agent add --team <team_id> --name <name> --role <role> --cmd <cmd> --cwd <path>
agentteam agent status --team <team_id> --name <name>

agentteam msg send --team <team_id> --from <name> --to <role_or_name> --text <text>
agentteam msg send --team <team_id> --from Kevin@local --to Alice@review-daemon --text <text>
agentteam msg list --team <team_id> --agent <name>

agentteam note post --team <team_id> --from <name> --to <target> --action <action> --text <text>
agentteam note thread --team <team_id> --thread <thread_id>
agentteam note tail --team <team_id> --limit <n>

agentteam render --team <team_id> --agent <name>
agentteam debug snapshot
agentteam debug resources --team <team_id>
```

## Required Behavior

- Agents use CLI only.
- CLI can output machine-readable JSON.
- Skill docs must mention team discovery, message send, task check, task completion, error report, debug snapshot.
- Skill docs must mention manager bootstrap and worker identity.
- Skill docs must mention that startup parameters assign manager name, role, team, and project scope.
- Skill docs must mention that the manager initializes workers after launch.
- Skill docs must mention `msg send` as the message delivery surface and `ready report` as the worker ready surface.
- Skill docs must mention `agentteam start` as the configured manager entrypoint and default `cwd` scope.
- Skill must not depend on hidden daemon wire protocol.
- Skill must teach the manager how to initialize framework, query task board, publish tasks, message child agents, and wait for results.
- Skill must teach agents that single-agent attach_tui/headless control lives in Agent Control Center.
- Skill must teach roles through skills, not through hidden tmux/session details.
- Skill must teach agents to write work notes through `agentteam note post`, read `TANote.md`/note projections, and reference `thread_id`/`note_id` during discussion.
- Skill must not expose tmux session names, pane ids, session descriptor paths, zterm endpoints, or event log paths to agents.
- Agents operate through names, roles, tasks, messages, and projections only.
- Cross-daemon agent references use `agent@domain`; bare names are local-domain only.
- Agents must not manually edit `TANote.md`; direct edits are invalid because daemon order, ids, and event receipts would be missing.
- Skill must teach the manager to inspect resource/debug projections when waiting or diagnosing stuck workers.

## Error Behavior

CLI errors go through Input Gateway and Error Center.

## Debug Snapshot

CLI can request Debug Center snapshots. CLI does not collect snapshots itself.

## Resource Lifecycle

CLI/Skill owns no durable runtime resources.

Rules:

- CLI commands may create short-lived input/output handles through Input/Output Gateway.
- CLI must not hold daemon state, task state, debug bundles, or resource handles after command exit.
- CLI must not perform cleanup directly; it requests scoped cleanup through daemon/resource owner commands.
- Skill instructions must not teach manual process/session/file cleanup.

## Red Tests

- Skill references hidden protocol fails doc gate.
- Skill references tmux/session internals fails doc gate.
- Manager skill missing init/task/message/wait instructions fails doc gate.
- Skill missing TANote read/post/thread instructions fails doc gate.
- Skill telling agents to manually edit `TANote.md` fails doc gate.
- CLI command bypasses Input Gateway fails architecture gate.
- Broad process kill command in docs fails.
- CLI holding durable resource handle after command exit fails.
- Skill teaching direct cleanup fails.
- Skill hiding cross-daemon address syntax fails doc gate.
- Skill exposing daemon endpoint/auth details fails doc gate.

## Open Decisions

- Exact CLI naming: `agentteam agent` vs `agentteam member`.
- Whether `--json` is global.
- Whether `task done` can be called only by assigned agent.
- Exact note target grammar: `agent:<name>`, `role:<role>`, `team:<team_id>`, or `all`.
