---
name: agentteam
description: Use when operating inside the AgentTeam project or when an agent needs to coordinate with other team agents through the AgentTeam CLI.
---

# AgentTeam Skill

Use this skill when operating inside the AgentTeam project or when an agent needs to coordinate with other team agents through the AgentTeam CLI.

For repository module development, architecture gates, Rust implementation, function maps, verification maps, and red tests, use the separate `agentteam-dev` skill.

## Current Status

MVP local runtime phase. Config/domain/debug/daemon-check commands execute through owner modules. Task Engine now supports local persistent task commands backed by the AgentTeam event log. `agentteam start` is the user-facing configured-manager entrypoint, it defaults to current `cwd`, and it expands into the transparent tmux bootstrap carrier for that configured manager. The sample config names this manager `Kevin`.

## Required Reading

1. `AGENTS.md`
2. `docs/architecture/overview.md`
3. `docs/architecture/ascii-flows.md`
4. `docs/modules/00-module-discussion-index.md`
5. `docs/architecture/mvp-start-gate.md`
6. `docs/usage/agentteam-usage.md`
7. Target module doc
8. `docs/architecture/function-map.md`
9. `docs/architecture/verification-map.md`

## Operating Rules

- Use Rust for implementation.
- Use `agentteam-dev` for module development lifecycle work.
- Keep module APIs isolated.
- Put shared logic in contracts/blocks.
- Treat startup parameters as the source of truth for each agent name, role, team, and project scope.
- Treat configured sample names such as `Kevin` as data only; framework concepts use neutral manager/root-manager/configured-agent wording.
- Treat `agentteam start` as the user-facing configured-manager entrypoint that defaults to current `cwd`.
- Treat function map as a hard gate before implementation.
- Keep hand-written Rust source leaf files at 500 lines or less; split shared blocks/contracts instead of exempting them.
- Keep orchestrator pure.
- Route all errors through Error Center.
- Route all messages/tasks through Communication Center.
- Route all CLI/API parsing through Input Gateway.
- Route all external rendering through Output Gateway.
- Route all debug snapshots through Debug Center.
- Write team work notes through `agentteam note post`; read `TANote.md` or note projections for discussion context.
- Treat resources as leased owner-scoped entities; use debug/resource views for leaks, orphans, and budget evidence.
- Treat debug output as persisted; use returned bundle/evidence ids for follow-up.
- Close daemon/session through scoped commands so resources and temporary files are cleaned by owner modules.
- Never use broad process kill commands.
- Never manually edit `TANote.md`; daemon-generated note ids, sequence, and event receipts are required.

Role execution model:

  - startup params give the configured manager its name, role, team, and project scope
  - skills tell the configured manager how to act as manager and how to initialize workers
  - the manager uses CLI commands to publish tasks, send messages, broadcast, wait for ready/status, and request debug evidence
  - the manager uses skills and CLI to initialize worker name/role/team/project scope after launch
  - workers use CLI commands to report ready, claim work, post notes, and report done/error
  - operators use the same docs plus `agentteam-dev` for module development and gates

When the manager sends a request and the session remains alive, silence is pending work, not error. Use task/message/ready/debug projections and adapter fault evidence to decide `idle`, `busy`, or `error`.

The configured manager is the root manager for the project agent tree. It does not auto-exit after startup; the user explicitly exits the manager, and the scoped shutdown flow exits the managed workers with it. The manager skill surface must include a CLI feedback path that returns task results to the framework.

## Current Local CLI Use

Agents should use CLI commands only, for example:

```text
agentteam config check --config docs/config/config.toml.example --json
agentteam daemon check --config docs/config/config.toml.example --json
agentteam domain resolve --target Alice@review-daemon --config docs/config/config.toml.example --json
agentteam debug snapshot --config docs/config/config.toml.example --runtime-home target/agentteam-smoke --json
agentteam task send --runtime-home target/agentteam-task-smoke --team default --created-by Kevin --target-kind role --target builder --title "Implement approved module" --body "Use owner APIs and gates" --json
agentteam task list --runtime-home target/agentteam-task-smoke --json
agentteam task claim --runtime-home target/agentteam-task-smoke --worker-name Alice --worker-role builder --json
agentteam task status --runtime-home target/agentteam-task-smoke --task AT-000001 --json
agentteam task done --runtime-home target/agentteam-task-smoke --task AT-000001 --actor Alice --detail "Completed with tests" --json
agentteam task error --runtime-home target/agentteam-task-smoke --task AT-000001 --actor Alice --detail "Blocked by missing config" --json
agentteam ready report --runtime-home target/agentteam-task-smoke --sender Alice --team default --agent-name Alice --body "ready" --json
agentteam msg send --runtime-home target/agentteam-task-smoke --from Kevin --to Alice --action message --body "Please review task AT-1" --json
agentteam msg broadcast --runtime-home target/agentteam-task-smoke --sender Kevin --team default --action broadcast --body "Team sync" --members Alice,Bob --json
AGENTTEAM_CODEX_SDK_SRC=/Users/fanzhang/code/codex/sdk/python/src AGENTTEAM_CODEX_BIN=/opt/homebrew/bin/codex agentteam control headless --agent Kevin --team default --session TA_headless_Kevin --json
AGENTTEAM_CODEX_SDK_SRC=/Users/fanzhang/code/codex/sdk/python/src AGENTTEAM_CODEX_BIN=/opt/homebrew/bin/codex agentteam control headless-run --agent Kevin --team default --session TA_headless_Kevin --input "reply with exactly: ready" --json
AGENTTEAM_CODEX_SDK_SRC=/Users/fanzhang/code/codex/sdk/python/src AGENTTEAM_CODEX_BIN=/opt/homebrew/bin/codex agentteam control headless-status --agent Kevin --team default --session TA_headless_Kevin --json
AGENTTEAM_CODEX_SDK_SRC=/Users/fanzhang/code/codex/sdk/python/src AGENTTEAM_CODEX_BIN=/opt/homebrew/bin/codex agentteam control headless-stop --agent Kevin --team default --session TA_headless_Kevin --json
agentteam note post --team default --from Kevin --to agent:Alice --action ask --text "Inspect AT-1 and reply in the thread"
agentteam note thread --team default --thread TH-20260608T120000Z-000001
```

Headless control rules:

- `control headless` creates or binds one headless agent session.
- `control headless-run` sends one typed prompt through the persistent Codex SDK bridge.
- `control headless-stop` is a normal scoped stop and should project `offline`, not `error`.
- `control headless-status` after stop recovers only when it resumes the same persisted `thread_id`.
- If a headless worker must write runtime state under `~/code/playground`, run the control command from `~/code/playground` so the SDK workspace-write sandbox covers that runtime home.

Planned daemon/team communication commands remain:

```text
agentteam agent list --team default --json
agentteam msg send --team default --from Kevin --to Bob --text "Please review task AT-1"
agentteam msg send --team default --from Kevin@local --to Alice@review-daemon --text "Cross-daemon review request"
agentteam note post --team default --from Kevin --to agent:Alice --action ask --text "Inspect AT-1 and reply in the thread"
agentteam note thread --team default --thread TH-20260608T120000Z-000001
agentteam debug resources --team default --json
```

Do not depend on hidden daemon wire protocol.

Use domain-qualified addresses such as `Alice@review-daemon` for cross-daemon communication. Bare names are local-domain only.
