# MVP Debug Build

## Purpose

The MVP is acceptable only if a local operator can answer what happened, who owns each resource, where evidence lives, and whether a module leaked or orphaned work.

This is a debug-first MVP. It is still no-code planning until module requirements are accepted.

## MVP Must Include

- daemon event log with replay
- explicit error code/severity/evidence id
- debug bundle command
- persisted debug bundle/evidence artifacts
- module snapshots through public snapshot APIs
- resource lifecycle registry
- agent registry/status projection
- task board projection
- message delivery projection
- TANote projection summary
- zterm/tmux transport observation summary
- startup/session projection
- required-file and architecture red-test gates

## Minimum Debug Commands

```text
agentteam daemon status --json
agentteam debug snapshot --team <team_id> --json
agentteam debug agent --team <team_id> --name <agent_name> --json
agentteam debug task --id <task_id> --json
agentteam debug resource --id <resource_id> --json
agentteam debug resources --team <team_id> --json
agentteam debug evidence --id <evidence_id> --json
agentteam debug replay --project <project_slug> --json
```

## Debug Bundle Required Fields

```text
bundle_id
created_at
project_slug
team_id
event_log_head
event_log_tail
module_snapshot_versions
errors[]
evidence[]
agents[]
tasks[]
messages[]
tanote_threads[]
resources[]
transport_observations[]
redactions[]
verification_gates[]
persistence_receipt
```

Each resource entry must include:

```text
resource_id
resource_class
owner_module
owner_entity_id
scope
lease_id
state
created_at
last_seen_at
last_event_id
ref_count
memory_bytes_estimate
handle_count
ttl_policy
release_policy
cleanup_policy
orphan_reason
leak_reason
evidence_id
```

## Resource Classes For MVP

```text
daemon_process
tmux_session
zterm_connection
tui_process
agent_member
task_record
message_envelope
tanote_projection
event_log_writer
debug_bundle
config_snapshot
provider_adapter
ui_subscription
temporary_file
```

## Debug Persistence

All debug capture is durable in v1.

Rules:

- `agentteam debug snapshot` persists a bundle before returning.
- `agentteam debug agent/task/resource/resources` persists the assembled debug bundle or evidence view before returning.
- `agentteam debug evidence` returns a persisted evidence record.
- Debug output includes `bundle_id`, `evidence_id` when applicable, and Persistence append receipt.
- No print-only or no-save debug capture mode exists in v1.

## Growth Control

The first MVP does not use aggressive hard memory limits. It must still prevent unbounded growth.

```text
max_agents_per_team = config-defined
max_pending_messages_per_agent = config-defined
max_task_projection_rows = config-defined
max_tanote_tail_blocks = config-defined
max_debug_bundle_bytes = config-defined
max_terminal_buffer_snapshot_bytes = config-defined
max_open_zterm_connections_per_agent = 1
max_provider_adapter_instances_per_agent = 1
```

Growth-control rules:

- queues, cursors, subscriptions, temp files, render buffers, and debug assembly handles must have release paths
- in-memory projections use bounded views and persisted event/evidence truth
- monotonic growth without drain/release is a Resource Lifecycle fault
- temporary files are cleaned during scoped daemon/session shutdown
- budget/growth overflow is not silently cropped; it produces explicit projection metadata and an event with evidence id
- user-visible business payload must not be semantically cropped

## MVP Acceptance Gates

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask red-tests
cargo xtask verify-required-files
cargo xtask verify-resource-lifecycle
```

During the no-code phase, the verification evidence is:

```text
find docs -type f | sort
rg -n "resource.lifecycle|MVP Debug|debug resource|orphan|leak" docs .agents AGENTS.md README.md
git status --short
```

## Not Enough For MVP

- debug snapshot without resource lifecycle data
- debug output without persisted bundle/evidence receipt
- status projection without event id/evidence id backtrace
- tmux session existence without owner lease
- error without persisted event
- orphan cleanup without exact handle
- broad process kill or broad file deletion
- UI-only observability without CLI JSON
- TANote discussion without task/event/message ids where relevant

## Open Decisions

- Whether resource heartbeat interval is global or per resource class.
- Exact warning thresholds for growth-control metrics after real MVP usage data exists.
