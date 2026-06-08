# 04 Input Gateway

## Purpose

Input Gateway is the external input boundary for CLI/API/UI/WebUI commands.

## Owns

- Raw command/API request capture.
- UI/WebUI raw input capture.
- Parsing external input into typed request nodes.
- Validation handoff to contracts.
- Command-to-runtime intent mapping.

## Does Not Own

- Runtime orchestration.
- Config parsing.
- Error classification.
- Terminal transport.
- Persistence.
- UI projection rendering.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `input.parse_cli` | Input Gateway | Parse CLI raw input into parsed command node | `TeamReq01CliRaw` | `TeamReq02ParsedCommand` | malformed CLI |
| `input.parse_api` | Input Gateway | Parse daemon API request into parsed command node | `TeamReq01ApiRaw` | `TeamReq02ParsedCommand` | malformed API |
| `input.parse_ui` | Input Gateway | Parse UI/WebUI action into parsed command node | `TeamReq01UiRaw` | `TeamReq02ParsedCommand` | UI bypass |
| `input.validate_intent` | Input Gateway + contracts | Validate parsed command semantics | `TeamReq02ParsedCommand` | `TeamReq03ValidatedIntent` | raw-to-daemon shortcut |
| `input.map_daemon_command` | Input Gateway | Build daemon command from validated intent | `TeamReq03ValidatedIntent` | `TeamReq04DaemonCommand` | non-adjacent conversion |
| `input.snapshot` | Input Gateway | Provide parse/validation snapshot to Debug Center | gateway counters | input snapshot | private text leak |
| `input.help` | Input Gateway | Describe CLI/API/UI input grammar | help topic | help model | hidden wire protocol |

## Module Help Contract

Required help topics:

```text
agentteam help input
agentteam help input cli
agentteam help input api
agentteam help input ui
agentteam help input validation
agentteam help input red-tests
```

Help content must explain:

- all external commands enter through Input Gateway
- CLI/API/UI raw payloads are parsed into typed request nodes
- payload semantics must be preserved
- malformed input becomes a classified error
- downstream modules receive validated intents only

Help content must not:

- document direct runtime mutation from UI or CLI
- expose hidden daemon wire protocol as required agent behavior
- suggest raw tmux input as task/message API
- suggest fallback parsing paths

## Public API Boundary

```text
TeamReq01CliRaw -> TeamReq02ParsedCommand -> TeamReq03ValidatedIntent
TeamReq01UiRaw -> TeamReq02ParsedCommand -> TeamReq03ValidatedIntent
```

Input Gateway outputs validated intent only.

## Required Behavior

- Parse CLI commands.
- Parse local daemon API requests.
- Parse UI/WebUI actions.
- Preserve payload semantics.
- Reject malformed raw input.
- Never call tmux/zterm directly.
- Never mutate runtime state directly.

## Error Behavior

Parse or validation faults go to Error Center.

## Debug Snapshot

Snapshot includes:

- accepted command count
- rejected command count
- last command shape without sensitive text if redaction policy requires it

## Resource Lifecycle

Input Gateway owns lifecycle requests for:

- raw input parse buffer
- validated intent handle

Rules:

- Register validated intent handle for commands that cross into runtime orchestration.
- Release parse buffers after parsed command or classified parse error exists.
- Large input payload metrics must be tracked without cropping user-visible semantics.
- A validated intent without downstream command result is an orphan candidate.
- Accepted/rejected counts, parse latency, and raw payload byte size are efficiency metrics.

## Red Tests

- Runtime parsing raw CLI fails architecture gate.
- UI/WebUI bypassing Input Gateway fails architecture gate.
- Non-adjacent conversion from raw input to daemon command fails.
- Malformed input produces classified error.
- Payload crop/rewrite fails.
- Validated intent without lifecycle lease fails.
- Parse buffer retained after command completion fails.

## Open Decisions

- Exact CLI command grammar.
- Whether daemon API accepts JSON only or also line protocol.
