# 01 Config Center

## Purpose

Config Center is the only owner of `~/.agentteam/config.toml` parsing, validation, normalization, and config snapshots.

`~/.agentteam/config.toml` is the user configuration source. It stores project-related user choices only, not runtime state, task state, message history, debug bundles, pid files, derived snapshots, or daemon-owned internal materialized state.

## Owns

- TOML parse.
- Commented config example.
- Config schema validation.
- Normalized runtime config.
- Config snapshot for Debug Center.
- Config-related error facts.
- Project-related user config.
- Team category config.
- Agent count config.
- Team member role config.
- Team member launch parameters.
- Daemon domain user config shape.

## Does Not Own

- Starting daemon.
- Launching agents.
- Parsing CLI commands.
- Applying task policy.
- Reading module private state.
- Saving runtime state.
- Saving task/message/event/debug data.
- Persisting derived daemon snapshots.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `config.load_user_file` | Config Center | Load `~/.agentteam/config.toml` or explicit test override | `ConfigReq01TomlPath` | `ConfigReq02TomlRaw` | missing file, wrong path |
| `config.parse_toml` | Config Center | Parse TOML only inside Config Center | `ConfigReq02TomlRaw` | `ConfigReq03ParsedToml` | parse outside Config Center |
| `config.validate_user_shape` | Config Center | Validate user-only config shape | `ConfigReq03ParsedToml` | `ConfigReq04ValidatedUserConfig` | runtime state keys, missing project |
| `config.validate_team_members` | Config Center | Validate team category, agent count, names, roles, launch params | `ConfigReq03ParsedToml` | `ConfigReq04ValidatedUserConfig` | duplicate name, count mismatch |
| `config.validate_domains` | Config Center + Daemon Domain Registry | Validate local/remote daemon domain config shape | `ConfigReq03ParsedToml` | `ConfigReq04ValidatedUserConfig` | duplicate domain, token leak |
| `config.normalize_runtime` | Config Center | Build runtime-ready config without saving derived truth to user config | `ConfigReq04ValidatedUserConfig` | `ConfigResp05RuntimeConfig` | hidden default/fallback |
| `config.snapshot` | Config Center | Provide redacted config snapshot for Debug Center | `ConfigResp05RuntimeConfig` | `ConfigResp06Snapshot` | token leak |

## Module Help Contract

Config Center must expose help text for CLI/UI/skill surfaces.

Required help topics:

```text
agentteam help config
agentteam help config path
agentteam help config schema
agentteam help config teams
agentteam help config agents
agentteam help config red-tests
```

Help content must explain:

- default config path: `~/.agentteam/config.toml`
- config stores project-related user config only
- runtime/task/message/debug state is forbidden in config
- `teams.agent_count` must match `teams.members` count
- managed tmux session names derive from `TA_<domain_id>_<project_slug>_<agent_name>`
- daemon domains define cross-daemon addressing boundaries
- launch command fields: `command`, `args`, `cwd`, `env`

Help content must not:

- suggest fallback paths
- suggest writing runtime state into config
- expose zterm auth token
- instruct users to use broad process kill commands

## Public API Boundary

Future Rust API shape:

```text
ConfigReq01TomlPath -> ConfigReq02TomlRaw -> ConfigReq03ParsedToml -> ConfigReq04ValidatedUserConfig -> ConfigResp05RuntimeConfig
```

Only Config Center can build `ConfigResp05RuntimeConfig`.

Default path:

```text
~/.agentteam/config.toml
```

## Required Behavior

- Load user config from `~/.agentteam/config.toml` by default.
- Allow explicit config path only for tests or clearly requested CLI override.
- Preserve comments in user-facing config templates.
- Store only project-related user configuration.
- Reject missing required fields.
- Reject duplicate agent names.
- Reject invalid project slug.
- Reject tmux prefix that is not `TA` unless explicitly allowed by future policy.
- Normalize paths without guessing hidden defaults.
- Preserve comments in example file, not runtime output.
- Configure project slug and project root.
- Configure team category.
- Configure agent count.
- Configure each team member name, team role category, work role, cwd, launch command, launch args, and env.
- Reject config when declared agent count differs from actual member list count.
- Reject runtime-state-looking keys under user config.
- Configure one local daemon domain id and optional remote daemon domains.
- Reject duplicate daemon domain ids or aliases.

## User Config Schema Draft

```text
[project]
slug
root

[runtime]
home
host
port

[tmux]
managed_prefix
binary

[zterm]
host
port
auth_token

[daemon_domain]
id
aliases

[[daemon_domains.remote]]
id
aliases
host
port
auth_token

[[teams]]
id
category
agent_count

[[teams.members]]
name
team_role
role
cwd
command
args
env
```

## Not Allowed In Config

- task list
- message list
- event log
- debug snapshot
- daemon pid
- agent runtime status
- derived tmux pane state
- zterm buffer/mirror state

## Error Behavior

Config errors become `TeamErr02Validation` through Error Center.

No config error may be repaired silently.

## Debug Snapshot

Snapshot includes:

- config file path
- normalized project slug
- runtime home
- team categories
- agent count
- zterm endpoint with token redacted
- daemon domain ids and endpoint tokens redacted
- validation status

## Resource Lifecycle

Config Center owns lifecycle requests for:

- `config_snapshot`
- config parse/validation working buffer

Rules:

- Register `config_snapshot` when a normalized runtime config snapshot is made available to Debug Center.
- Release parse/validation buffers after normalized config or classified error is produced.
- Config snapshots must be redacted before projection.
- Config Center must not retain historical config snapshots unboundedly.
- Snapshot count, snapshot bytes, parse latency, and validation error count are efficiency metrics.

## Red Tests

- Duplicate agent names fail.
- Missing project slug fails.
- Invalid session prefix fails.
- Declared agent count mismatch fails.
- Runtime-state-looking keys fail.
- Non-Config module TOML parse attempt is rejected by architecture gate.
- Token is redacted in debug snapshot.
- Duplicate daemon domain id fails.
- Domain auth token is redacted in debug snapshot.
- Config snapshot without lifecycle lease fails.
- Unbounded config snapshot retention fails.

## Open Decisions

- Whether config supports multiple projects in one `~/.agentteam/config.toml`, or one active project entry at a time.
- Whether agent command may be a shell string or must be command + argv array.
- Whether tmux binary discovery is allowed or must be explicit.
- Exact supported team categories.
