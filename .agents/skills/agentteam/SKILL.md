---
name: agentteam
description: Use when operating inside the AgentTeam project or when an agent needs to coordinate with other team agents through the AgentTeam CLI.
---

# AgentTeam Skill

Use this skill when operating inside the AgentTeam project or when an agent needs to coordinate with other team agents through the AgentTeam CLI.

For repository module development, architecture gates, Rust implementation, function maps, verification maps, and red tests, use the separate `agentteam-dev` skill.

## Current Status

MVP scaffold phase. Rust workspace/gate skeletons may be implemented; business runtime behavior still requires target module docs and red tests first.

## Required Reading

1. `AGENTS.md`
2. `docs/architecture/overview.md`
3. `docs/architecture/ascii-flows.md`
4. `docs/modules/00-module-discussion-index.md`
5. `docs/architecture/mvp-start-gate.md`
6. Target module doc
7. `docs/architecture/function-map.md`
8. `docs/architecture/verification-map.md`

## Operating Rules

- Use Rust for implementation.
- Use `agentteam-dev` for module development lifecycle work.
- Keep module APIs isolated.
- Put shared logic in contracts/blocks.
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

## Future CLI Use

Agents should use CLI commands only, for example:

```text
agentteam agent list --team default --json
agentteam task list --team default --agent Kevin --json
agentteam task send --team default --to Alice --text "Implement approved module"
agentteam msg send --team default --from Kevin --to Bob --text "Please review task AT-1"
agentteam msg send --team default --from Kevin@local --to Alice@review-daemon --text "Cross-daemon review request"
agentteam note post --team default --from Kevin --to agent:Alice --action ask --text "Inspect AT-1 and reply in the thread"
agentteam note thread --team default --thread TH-20260608T120000Z-000001
agentteam debug resources --team default --json
agentteam task done --id AT-1 --summary "Completed with tests"
agentteam task error --id AT-1 --message "Blocked by missing config"
```

Do not depend on hidden daemon wire protocol.

Use domain-qualified addresses such as `Alice@review-daemon` for cross-daemon communication. Bare names are local-domain only.
