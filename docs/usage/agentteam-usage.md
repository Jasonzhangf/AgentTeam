# AgentTeam Usage Guide

## Purpose

This guide explains how to read the AgentTeam skills, which docs each role should open first, and how to run the current local startup and E2E smoke path.

It covers the current repository truth, not a future ideal flow.

The configured manager's initial role assignment comes from startup parameters. Role behavior comes from skills. The sample config names this manager `Kevin`; code and docs must treat that as a configured agent name, not as a hard-coded role concept.

## What To Read First

Read these in order:

1. [AGENTS.md](../../AGENTS.md)
2. [AgentTeam skill](../../.agents/skills/agentteam/SKILL.md)
3. [AgentTeam development skill](../../.agents/skills/agentteam-dev/SKILL.md) if you will change code or docs
4. [Architecture overview](../architecture/overview.md)
5. [ASCII flows](../architecture/ascii-flows.md)
6. Role-specific module docs listed below

## Skills

### `agentteam`

Use this skill when you are operating the framework as the configured manager, a worker, or an operator using the AgentTeam CLI.

It is the runtime/operator surface. It explains:

- how to use CLI commands
- how the configured manager boots the team
- how startup parameters assign manager name, role, team, and project scope
- how the manager uses skills/CLI to initialize workers after launch
- how skills define what each role is allowed to do
- how workers report ready, claim work, and report done/error
- how notes and debug resources are used
- how to avoid tmux/session internals

### `agentteam-dev`

Use this skill when you are changing AgentTeam code, docs, gates, or module contracts.

It is the development-cycle surface. It explains:

- how to lock the function map
- how to add red tests before implementation
- how to keep module boundaries isolated
- how to respect the 500-line leaf-file limit

## Role Reading Guide

### Configured Manager

The configured manager is the single super manager in v1. The sample config names this agent `Kevin`.

The manager should read:

- [AgentTeam skill](../../.agents/skills/agentteam/SKILL.md)
- [13 CLI Agent Skill](../modules/13-cli-agent-skill.md)
- [15 Startup And Session Manager](../modules/15-startup-session-manager.md)
- [03 Communication Center](../modules/03-communication-center.md)
- [07 Agent Registry And Naming Pool](../modules/07-agent-registry-naming-pool.md)
- [09 Task Engine](../modules/09-task-engine.md)
- [10 Debug Center](../modules/10-debug-center.md)
- [16 TANote Collaboration Board](../modules/16-tanote-collaboration-board.md)
- [17 Resource Lifecycle Manager](../modules/17-resource-lifecycle-manager.md)

The manager uses these docs to learn:

- its identity comes from startup params
- how to publish tasks
- how to wait on task and message projections
- how to send messages and broadcasts
- how to read debug/resource evidence
- how not to touch tmux/session internals directly
- how startup params assign configured identity and manager role before skills take over

### Worker

Worker agents should read:

- [AgentTeam skill](../../.agents/skills/agentteam/SKILL.md)
- [13 CLI Agent Skill](../modules/13-cli-agent-skill.md)
- [07 Agent Registry And Naming Pool](../modules/07-agent-registry-naming-pool.md)
- [09 Task Engine](../modules/09-task-engine.md)
- [03 Communication Center](../modules/03-communication-center.md)
- [16 TANote Collaboration Board](../modules/16-tanote-collaboration-board.md)

If the worker needs to inspect runtime evidence or resource state, also read:

- [10 Debug Center](../modules/10-debug-center.md)
- [17 Resource Lifecycle Manager](../modules/17-resource-lifecycle-manager.md)

Worker agents use these docs to learn:

- their assigned name and role
- how to send `ready report`
- how to claim work
- how to mark tasks `done` or `error`
- how to post and read TANote discussion threads
- how to communicate with the manager and peers through CLI projections
- how the manager assigns the worker name and role through skills/CLI after its own startup

### Operator / Maintainer

An operator or maintainer who is not acting as the manager or a worker should read:

- [AgentTeam skill](../../.agents/skills/agentteam/SKILL.md)
- [AgentTeam development skill](../../.agents/skills/agentteam-dev/SKILL.md)
- [Architecture overview](../architecture/overview.md)
- [Function map](../architecture/function-map.md)
- [Verification map](../architecture/verification-map.md)
- [Red test plan](../red-tests/red-test-plan.md)

This role uses the docs to verify the system, not to drive a team task flow.

## Manager Current Operating Path

`agentteam start` is the user-facing entrypoint. It starts the configured manager from the current `cwd` by default, and that `cwd` becomes the manager's default scope. The sample config names this agent `Kevin`.

```text
agentteam start
```

After the manager is running, the manager uses skills/CLI to initialize worker agents with worker startup params and then runs the current CLI smoke path:

```text
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- config check --config <repo>/docs/config/config.toml.example --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- daemon check --config <repo>/docs/config/config.toml.example --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- ready report --runtime-home ~/code/playground/agentteam-e2e --sender Kevin --team default --agent-name Kevin --body ready --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- msg send --runtime-home ~/code/playground/agentteam-e2e --from Kevin --to Alice --action message --body hello --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- msg broadcast --runtime-home ~/code/playground/agentteam-e2e --sender Kevin --team default --action broadcast --body hello --members Alice,Bob --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- task send --runtime-home ~/code/playground/agentteam-e2e --team default --created-by Kevin --target-kind role --target builder --title smoke --body task-body --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- task claim --runtime-home ~/code/playground/agentteam-e2e --worker-name Alice --worker-role builder --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- task done --runtime-home ~/code/playground/agentteam-e2e --task AT-000001 --actor Alice --detail done --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- task error --runtime-home ~/code/playground/agentteam-e2e --task AT-000002 --actor Alice --detail failed --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- tmux loopback --runtime-home ~/code/playground/agentteam-tmux-e2e --session-count 2 --json
```

Replace `<repo>` with `/Users/fanzhang/Documents/github/agentteam` on this machine.

## Headless Codex Control Smoke

The headless control lane uses a persistent Codex SDK bridge process. It is for automatic control; visible TUI control still uses the tmux lane.

```text
export AGENTTEAM_CODEX_SDK_SRC=/Users/fanzhang/code/codex/sdk/python/src
export AGENTTEAM_CODEX_BIN=/opt/homebrew/bin/codex

cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- control headless --agent Kevin --team default --session TA_headless_Kevin --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- control headless-run --agent Kevin --team default --session TA_headless_Kevin --input "reply with exactly: ready" --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- control headless-status --agent Kevin --team default --session TA_headless_Kevin --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- control headless-stop --agent Kevin --team default --session TA_headless_Kevin --json
```

MVP control meanings:

- `control headless` creates or binds one headless agent session.
- `control headless-run` sends one prompt through the persistent SDK bridge.
- `control headless-stop` stops only that scoped bridge process and projects `offline`; it does not clean tmux evidence sessions.
- `control headless-status` after a stop must recover by spawning a new bridge process and resuming the persisted `thread_id`.

Recovery evidence is the same `thread_id` in `~/.agentteam/sessions/<project_slug>/headless/<session>/state.json` with a new live bridge PID.

## Minimal Role Workflow Smoke

The first workflow verification uses the sample manager `Kevin` and worker `Alice`. It does not depend on deterministic model prose. The framework truth is:

1. Alice sends `ready report`.
2. The manager sends Alice a message.
3. The manager creates a role-targeted task.
4. Alice claims the task.
5. Alice runs a small headless turn and uses CLI to mark the task `done`.
6. `task status` projects `done`.

Commands:

```text
export AGENTTEAM_CODEX_SDK_SRC=/Users/fanzhang/code/codex/sdk/python/src
export AGENTTEAM_CODEX_BIN=/opt/homebrew/bin/codex

cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- ready report --runtime-home ~/code/playground/agentteam-workflow --sender Alice --team default --agent-name Alice --body ready --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- msg send --runtime-home ~/code/playground/agentteam-workflow --from Kevin --to Alice --action assign --body "Claim the builder smoke task" --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- task send --runtime-home ~/code/playground/agentteam-workflow --team default --created-by Kevin --target-kind role --target builder --title "workflow smoke" --body "Return one concise result" --json
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- task claim --runtime-home ~/code/playground/agentteam-workflow --worker-name Alice --worker-role builder --json
(cd ~/code/playground && cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- control headless-run --agent Alice --team default --session TA_headless_Alice_workflow --input "You are Alice, role builder. Run this command to finish your claimed task: <repo>/target/debug/agentteam task done --runtime-home ~/code/playground/agentteam-workflow --task AT-000001 --actor Alice --detail done --json. Then reply with a one-line summary." --json)
cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- task status --runtime-home ~/code/playground/agentteam-workflow --task AT-000001 --json
(cd ~/code/playground && cargo run --manifest-path <repo>/Cargo.toml -p agentteam-cli -- control headless-stop --agent Alice --team default --session TA_headless_Alice_workflow --json)
```

The headless run is launched from `~/code/playground` because the Codex SDK bridge uses workspace-write sandboxing. The runtime home that the worker writes must be inside the SDK workspace.

Expected evidence:

- ready/message/task commands return persisted event ids and sequences
- `control headless-run` returns a control projection with SDK mode and captured details
- final `task status` returns one task with status `done`

## Manager Workflow

The manager loop is:

1. Open the `agentteam` skill.
2. Read the startup and communication docs.
3. Confirm the team and worker names from config.
4. Use CLI commands to publish tasks or broadcasts.
5. Wait on task, message, and debug projections.
6. Use `ready report` from workers as the readiness signal.
7. Use `task status` and `task list` to inspect progress.
8. Use `debug snapshot` and resource views to inspect evidence, leaks, and orphaned work.

The manager must not:

- call tmux directly
- write session/state files directly
- treat himself as durable persistence truth
- treat tmux stdout as final task truth

## Worker Workflow

Worker agents should follow this loop:

1. Read the `agentteam` skill.
2. Read the worker-facing sections in the CLI and startup docs.
3. Report `ready` after launch.
4. Claim a task only when it matches the worker scope.
5. Work the task.
6. Mark the task `done` or `error` through CLI.
7. Post progress or evidence into TANote threads when discussion is needed.
8. Use `task status` to confirm the board projection.

Workers must not:

- manage other workers
- edit TANote manually
- read tmux session ids as part of normal workflow
- assume stdout text alone is the source of truth

## Role-to-Doc Map

| Role | Primary docs | Typical commands |
|---|---|---|
| Manager, sample name `Kevin` | `agentteam`, `13-cli-agent-skill`, `15-startup-session-manager`, `03-communication-center`, `09-task-engine` | `ready report`, `msg send`, `msg broadcast`, `task send`, `task claim`, `task status`, `debug snapshot` |
| Worker | `agentteam`, `13-cli-agent-skill`, `07-agent-registry-naming-pool`, `09-task-engine`, `16-tanote-collaboration-board` | `ready report`, `task claim`, `task done`, `task error`, `note post`, `task status` |
| Operator | `agentteam`, `agentteam-dev`, `verification-map`, `red-test-plan` | `cargo xtask verify`, `cargo test --workspace`, `cargo xtask red-tests` |

## Current E2E Smoke Sequence

This is the current verified local smoke sequence:

1. `ready report`
2. `msg send`
3. `msg broadcast`
4. `task send`
5. `task list`
6. `task claim`
7. `task done`
8. `task error`
9. `task status`
10. `tmux loopback`

Expected evidence:

- message and broadcast receipts include `event_id`, `sequence`, and `log_path`
- task commands include `task_id`, `status`, and event sequence
- `tmux loopback` includes exact-handle cleanup evidence

## What Not To Do

- Do not use broad process kill commands.
- Do not edit `TANote.md` manually.
- Do not treat tmux stdout as final status truth.
- Do not read or mutate another module's private state directly.
- Do not add fallback or downgrade paths.
