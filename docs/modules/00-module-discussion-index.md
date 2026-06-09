# Module Discussion Index

Use this file as the agenda for one-module-at-a-time requirement discussion.

## Discussion Order

1. Config Center
2. Error Center
3. Communication Center
4. Agent Registry and Naming Pool
5. Persistence Event Log
6. Task Engine
7. Team Orchestrator
8. Input Gateway
9. Output Gateway
10. UI Gateway
11. Debug Center
12. zterm/tmux Adapter
13. CLI Agent Skill
14. TUI Agent Adapter Center
15. Startup and Session Manager
16. TANote Collaboration Board
17. Resource Lifecycle Manager
18. Daemon Domain Registry
19. Agent Control Center

## Per-Module Discussion Template

Each module doc must answer:

- purpose
- owns
- does not own
- module function map
- module help contract
- public API boundary
- input chain nodes
- output chain nodes
- error behavior
- debug snapshot behavior
- persistence behavior
- red tests
- open decisions

## Current Global Decisions

- Implementation language: Rust.
- Configuration: `config.toml` with comments.
- Runtime modules must expose isolated APIs.
- Shared functions go to contracts/blocks, not duplicated across modules.
- Orchestrator is pure coordination.
- Error Center is the only error classifier/projection authority.
- Communication Center is independent from terminal transport.
- UI, Input, and Output gateways are separate.
- Every feature needs red tests.
- TUI status is adapter-based. tmux stdout alone is evidence, not final status truth.
- The configured manager can bootstrap/manage through CLI, but daemon + Persistence remain durable truth.
- Agents discuss work in project `TANote.md` through the TANote Collaboration Board; notes are not task/message/event truth.
- Every module-owned resource must have a lifecycle lease, scoped owner, release path, debug snapshot, orphan/leak red test, and efficiency budget.
- Daemon domain is the cross-daemon naming boundary; `agent@domain` route resolution belongs to Daemon Domain Registry.

## Open Global Decisions

- Whether v1 includes an interactive local UI, or only a UI Gateway API plus CLI render command.
- Whether zterm integration uses only WebSocket bridge protocol in v1, or also embeds a local zterm daemon launcher.
- Exact persistence file format for snapshots after append-only events.
