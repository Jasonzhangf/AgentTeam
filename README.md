# AgentTeam

Rust-first multi-agent orchestration framework for TUI agents running under tmux.

AgentTeam will manage teams of named agents, task queues, inter-agent messaging, debug snapshots, configuration, and terminal communication through a zterm-compatible tmux bridge.

Current status: MVP scaffold phase. Rust workspace/gate skeletons may be implemented; business runtime behavior still follows the architecture docs first.

## First Version Target

- Rust daemon.
- Rust CLI.
- `config.toml` configuration.
- TA-prefixed tmux agent sessions.
- Role-based task queues.
- Cross-agent messaging.
- Shared `TANote.md` forum-style collaboration notes.
- Resource lifecycle tracking for leaks, orphans, and efficiency budgets.
- Single-agent render gateway through zterm/tmux terminal mirror.
- Independent Error Center, Communication Center, UI Gateway, Input Gateway, Output Gateway, Debug Center, Config Center, and Persistence/Event Log.

## Key Docs

- Architecture overview: `docs/architecture/overview.md`
- ASCII flows: `docs/architecture/ascii-flows.md`
- File structure: `docs/architecture/file-structure.md`
- Function map: `docs/architecture/function-map.md`
- Verification map: `docs/architecture/verification-map.md`
- Module discussion index: `docs/modules/00-module-discussion-index.md`
- Config example: `docs/config/config.toml.example`

## Naming Rule

Managed tmux sessions use:

```text
TA_<project_slug>_<agent_name>
```

Only sessions matching this prefix and project slug may be group-operated by AgentTeam.
