# File Structure

MVP scaffold may create Rust workspace, crate entry files, and architecture gates. Business runtime behavior still waits for module-specific implementation approval.

## Target Workspace

```text
agentteam/
  AGENTS.md
  README.md
  Cargo.toml
  Cargo.lock
  CACHE.md
  MEMORY.md
  note.md
  docs/
    architecture/
      overview.md
      ascii-flows.md
      file-structure.md
      function-map.md
      verification-map.md
      mvp-debug-build.md
      mvp-start-gate.md
      function-map-gate.md
      code-size-policy.md
    config/
      config.toml.example
    flows/
      build-regression-flow.md
    usage/
      agentteam-usage.md
    modules/
      00-module-discussion-index.md
      01-config-center.md
      02-error-center.md
      03-communication-center.md
      04-input-gateway.md
      05-output-gateway.md
      06-ui-gateway.md
      07-agent-registry-naming-pool.md
      08-team-orchestrator.md
      09-task-engine.md
      10-debug-center.md
      11-persistence-event-log.md
      12-zterm-tmux-adapter.md
      13-cli-agent-skill.md
      14-tui-agent-adapter-center.md
      15-startup-session-manager.md
      16-tanote-collaboration-board.md
      17-resource-lifecycle-manager.md
      18-daemon-domain-registry.md
      19-agent-control-center.md
      20-report-flow.md
    tanote/
      TANote.md.example
    red-tests/
      red-test-plan.md
  crates/
    agentteam-contracts/
      Cargo.toml
      src/
        lib.rs
        pipeline/
        feature_map/
        verification_map/
    agentteam-config/
      Cargo.toml
      src/
        lib.rs
    agentteam-error/
      Cargo.toml
      src/
        lib.rs
    agentteam-comm/
      Cargo.toml
      src/
        lib.rs
    agentteam-debug/
      Cargo.toml
      src/
        lib.rs
    agentteam-persist/
      Cargo.toml
      src/
        lib.rs
    agentteam-runtime/
      Cargo.toml
      src/
        lib.rs
    agentteam-startup/
      Cargo.toml
      src/
        lib.rs
    agentteam-tanote/
      Cargo.toml
      src/
        lib.rs
    agentteam-resource/
      Cargo.toml
      src/
        lib.rs
    agentteam-control/
      Cargo.toml
      src/
        lib.rs
    agentteam-tmux/
      Cargo.toml
      src/
        lib.rs
    agentteam-tui-adapter/
      Cargo.toml
      src/
        lib.rs
    agentteam-gateway/
      Cargo.toml
      src/
        lib.rs
    agentteam-cli/
      Cargo.toml
      src/
        main.rs
    agentteamd/
      Cargo.toml
      src/
        main.rs
  tests/
    red/
    integration/
    fixtures/
  xtask/
    Cargo.toml
    src/
      main.rs
  .cargo/
    config.toml
  .agents/
    skills/
      agentteam/
        SKILL.md
      agentteam-dev/
        SKILL.md
```

## User Config Location

User-editable config lives outside the repo by default:

```text
~/.agentteam/config.toml
```

The repo keeps only examples and schema docs:

```text
docs/config/config.toml.example
```

`~/.agentteam/config.toml` stores project-related user configuration only. Runtime data belongs under the configured runtime home, not inside the config file.

## Required File Gate Direction

Future build gate must fail if required tracked files are missing.

Future git gate must fail if required source/config/test docs are untracked.

Planned command:

```text
cargo xtask verify-required-files
```

Required groups:

- workspace manifests
- workspace lockfile
- Cargo aliases
- crate manifests
- crate entry files
- module docs
- function map
- verification map
- red-test plan
- config example
- TANote example/format
- MVP debug build plan
- MVP start gate
- function map gate
- code-size policy
- local skills

## No-Code Phase Rule

The no-code phase is closed by `docs/architecture/mvp-start-gate.md`.

Current allowed Rust work:

- workspace manifests
- crate entry files
- `xtask` gate implementation
- minimal shared constants/contracts needed by gates

Business runtime behavior waits until the target module doc, function map row, verification map row, and red tests are ready.
