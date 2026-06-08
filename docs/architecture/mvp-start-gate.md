# MVP Start Gate

## Status

MVP implementation may start from this gate.

The first implementation scope is Rust workspace scaffolding, architecture gates, red-test harnesses, and minimal contract types. Business runtime behavior comes after the gates compile and run.

## v1 Decisions

| Decision | v1 Choice |
|---|---|
| user config scope | one active project per `~/.agentteam/config.toml` |
| agent launcher shape | `command` plus explicit `args`; no shell-string truth |
| startup identity | `agentteam startup init` binds current TUI as Kevin |
| daemon API | JSON over loopback HTTP in v1 |
| persistence format | typed JSONL event log, with materialized snapshots later |
| task completion truth | explicit CLI `task done` / `task error` only |
| stdout markers | evidence/diagnostic only, not task completion truth |
| UI v1 | CLI/API projection first; full WebUI later |
| zterm v1 | external configured zterm endpoint; no embedded launcher in first MVP |
| render v1 | CLI render/attach projection through zterm/tmux adapter boundary |
| provider adapters | explicit provider id in config; generic adapter required |
| user marker adapters | later than first MVP |
| task dependency DAG | later than first MVP |
| TANote thread creation | explicit `agentteam note post`; automatic task thread later |
| note attachments | evidence ids only in v1 |
| note visibility | team-visible notes only in v1 |
| resource cleanup | scoped daemon/session close cleans owned resources/temp files automatically |
| debug capture | always persisted before output |
| memory policy | growth-control, not aggressive hard caps |

## First Rust Scope

- Create Cargo workspace and crate manifests.
- Create minimal crate entry files.
- Create `xtask` gates:
  - `red-tests`
  - `verify-required-files`
  - `verify-skill-frontmatter`
  - `verify-resource-lifecycle`
  - `verify-function-map`
  - `verify-code-size`
- Create minimal contract constants for feature ids and required files.
- Make `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` pass.

## Not In First Rust Scope

- Real daemon server.
- Real tmux/zterm execution.
- Full config parser.
- Full task/message runtime.
- WebUI.
- Provider SDK integration.

## Completion Signal

First MVP scaffold is complete only when:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask red-tests
cargo xtask verify-required-files
cargo xtask verify-skill-frontmatter
cargo xtask verify-resource-lifecycle
cargo xtask verify-function-map
cargo xtask verify-code-size
```

all pass locally.
