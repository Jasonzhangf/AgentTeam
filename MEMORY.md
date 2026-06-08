# MEMORY

No implemented runtime truth yet.

Verified project baseline:
- AgentTeam is a Rust-first framework project.
- Initial work must create architecture/module docs and file structure before code.
- Runtime must avoid fallback, duplicate owner logic, and cross-module internal state access.
- `TANote.md` is now specified as an agent-readable forum-style collaboration projection: agents post through CLI/daemon, TANote Board owns format/order/thread projection, and task/message/event truth remains in daemon-owned modules.
- Local AgentTeam skill must include YAML frontmatter; otherwise Codex skips it and Kevin/worker CLI guidance is unavailable.
- MVP debug requires Resource Lifecycle evidence: every long-lived resource has owner/scope/lease/release policy, orphan/leak/growth detection, event persistence, debug snapshot, and exact-handle cleanup.
- Debug capture is durable in v1: debug bundles/evidence/snapshots must be persisted before CLI/UI output claims them.
- Daemon/session close must run scoped resource cleanup and tracked temporary file cleanup; v1 avoids aggressive memory hard caps but forbids unbounded growth without release/drain/cleanup visibility.
- Function map is a hard implementation gate: no feature work without map owner/contracts/paths/gates and verification mapping.
- Rust source size policy: hand-written Rust leaf files have a 500-line hard limit; shared blocks/contracts are not exempt and should be split by owner/domain/node.
- Local skill split: `agentteam` is the runtime/CLI collaboration surface; `agentteam-dev` is the repository module-development cycle skill.
- Daemon domain design: zterm may expose multiple daemons, so AgentTeam agent names are domain-local. Cross-daemon addressing uses Daemon Domain Registry and canonical `agent@domain`; Communication Center must consume route plans instead of parsing domain strings itself.
