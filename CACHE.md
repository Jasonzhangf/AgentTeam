# CACHE

2026-06-08:
- New project `/Users/fanzhang/Documents/github/agentteam` started as empty directory.
- User requires Rust implementation, no code yet, first create detailed docs and file structure.
- User requires global AGENTS-compatible naming, shared blocks, pure orchestration, isolated module APIs.
- Required modules: Error Center, Communication Center, UI Gateway, Input Gateway, Output Gateway, Debug Center, Config Center, agent naming pool, task/team orchestration, zterm/tmux adapter.
- User requires detailed ASCII flow diagrams and per-feature red tests.
- Added TANote Collaboration Board requirement: each agent writes work notes via `agentteam note post`; project `TANote.md` is a forum-style projection with from/to/action/thread/note ids, while task truth stays in Task Engine.
- Fixed local skill YAML frontmatter after Codex skipped `.agents/skills/agentteam/SKILL.md`.
- Added Resource Lifecycle Manager and MVP Debug Build docs for resource leases, orphan/leak detection, exact-handle cleanup, and efficiency budget reporting.
- Latest decisions: debug is persisted in v1; daemon/session close cleans scoped resources and temp files; memory policy is growth-control, not aggressive hard caps.
- Latest governance decisions: function map is a hard implementation gate; 500-line hard limit applies to hand-written Rust leaf files including shared blocks/contracts.
- Local skill split: use `agentteam-dev` for per-module development cycle; keep `agentteam` for runtime/CLI team collaboration guidance.
- New requirement: zterm supports multiple daemons; AgentTeam supports cross-daemon communication through daemon domains. `agent@domain` parsing/route resolution belongs to Daemon Domain Registry; Agent Registry names are domain-local.
