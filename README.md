# AgentTeam

AgentTeam is a Rust CLI for testing multi-agent coordination through persisted messages, tasks, tmux transport, and optional Codex headless control.

This README is the user entrypoint. You do not need to understand the internal framework to run the current tests.

## What Works Now

- Send a ready signal for an agent.
- Send a message from one agent to another.
- Create, claim, finish, and inspect a task.
- Generate an ASCII and Mermaid flow report from the persisted log.
- Verify multiple tmux sessions can receive input and produce captured output.
- Optionally control one Codex headless agent through the local Codex SDK bridge.

## Requirements

Required:

- Rust toolchain with `cargo`
- A writable test directory, recommended: `~/code/playground`

Optional:

- `tmux`, for tmux input/output loopback testing
- Codex CLI plus local Codex SDK source, for headless agent testing

On this machine, the optional Codex paths used by the current tests are:

```text
/Users/fanzhang/code/codex/sdk/python/src
/opt/homebrew/bin/codex
```

## Build

From the repository root:

```text
cargo build -p agentteam-cli
```

The CLI binary is:

```text
./target/debug/agentteam
```

Set a short variable for the examples:

```text
export AGENTTEAM_BIN="$(pwd)/target/debug/agentteam"
mkdir -p ~/code/playground
export AGENTTEAM_RUN_ID="$(date +%Y%m%d%H%M%S)"
export REPO="$(pwd)"
```

## Test 1: Local Team Workflow

This test does not start a model. It verifies the core framework behavior: ready, message, task, claim, done, status, and report.

```text
export RUNTIME="$HOME/code/playground/agentteam-readme-local-$AGENTTEAM_RUN_ID"

"$AGENTTEAM_BIN" ready report \
  --runtime-home "$RUNTIME" \
  --sender Alice \
  --team default \
  --agent-name Alice \
  --body ready \
  --json

"$AGENTTEAM_BIN" msg send \
  --runtime-home "$RUNTIME" \
  --from Kevin \
  --to Alice \
  --action assign \
  --body "Claim the builder task" \
  --json

"$AGENTTEAM_BIN" task send \
  --runtime-home "$RUNTIME" \
  --team default \
  --created-by Kevin \
  --target-kind role \
  --target builder \
  --title "README smoke" \
  --body "Finish this task through AgentTeam" \
  --json

"$AGENTTEAM_BIN" task claim \
  --runtime-home "$RUNTIME" \
  --worker-name Alice \
  --worker-role builder \
  --json

"$AGENTTEAM_BIN" task done \
  --runtime-home "$RUNTIME" \
  --task AT-000001 \
  --actor Alice \
  --detail done \
  --json

"$AGENTTEAM_BIN" task status \
  --runtime-home "$RUNTIME" \
  --task AT-000001 \
  --json

"$AGENTTEAM_BIN" report flow \
  --runtime-home "$RUNTIME" \
  --json
```

Expected result:

- every command returns JSON with `"status":"ok"`
- `task status` shows task `AT-000001` as `done`
- `report flow` returns `ascii_flow` and `mermaid_flow`
- the event log exists at:

```text
$RUNTIME/events/agentteam.jsonl
```

To inspect only the generated report:

```text
"$AGENTTEAM_BIN" report flow --runtime-home "$RUNTIME" --json
```

## Test 2: tmux Input/Output Loopback

This verifies that AgentTeam can create multiple managed tmux sessions, send input, capture output, and clean up the exact sessions it created.

```text
export TMUX_RUNTIME="$HOME/code/playground/agentteam-readme-tmux-$AGENTTEAM_RUN_ID"

"$AGENTTEAM_BIN" tmux loopback \
  --runtime-home "$TMUX_RUNTIME" \
  --session-count 2 \
  --json
```

Expected result:

- `observed_count` is `2`
- `cleaned_handle_count` is `2`
- `cleanup_status` is `cleaned_exact_handles`

This test cleans only its own generated tmux sessions.

## Test 3: Optional Codex Headless Agent

Run this only if the local Codex SDK source and Codex CLI are available.

```text
export AGENTTEAM_CODEX_SDK_SRC=/Users/fanzhang/code/codex/sdk/python/src
export AGENTTEAM_CODEX_BIN=/opt/homebrew/bin/codex
export HEADLESS_SESSION="TA_headless_Alice_readme_$AGENTTEAM_RUN_ID"

cd ~/code/playground

"$AGENTTEAM_BIN" control headless \
  --agent Alice \
  --team default \
  --session "$HEADLESS_SESSION" \
  --json

"$AGENTTEAM_BIN" control headless-run \
  --agent Alice \
  --team default \
  --session "$HEADLESS_SESSION" \
  --input "reply with exactly: ready" \
  --json

"$AGENTTEAM_BIN" control headless-status \
  --agent Alice \
  --team default \
  --session "$HEADLESS_SESSION" \
  --json

"$AGENTTEAM_BIN" control headless-stop \
  --agent Alice \
  --team default \
  --session "$HEADLESS_SESSION" \
  --json
```

Expected result:

- `control headless-run` returns a JSON control projection
- `control headless-status` reports the session state
- `control headless-stop` reports `offline`

The stop command is scoped to this one headless session.

## Test 4: Optional Minimal Agent Workflow

This test asks a Codex headless agent to finish a task by calling AgentTeam CLI. It is the current smallest agent-driven workflow.

```text
export AGENTTEAM_CODEX_SDK_SRC=/Users/fanzhang/code/codex/sdk/python/src
export AGENTTEAM_CODEX_BIN=/opt/homebrew/bin/codex
export REPO="${REPO:-/Users/fanzhang/Documents/github/agentteam}"
export AGENTTEAM_BIN="$REPO/target/debug/agentteam"
export AGENTTEAM_RUN_ID="$(date +%Y%m%d%H%M%S)"
export RUNTIME="$HOME/code/playground/agentteam-readme-agent-$AGENTTEAM_RUN_ID"
export HEADLESS_WORKFLOW_SESSION="TA_headless_Alice_readme_workflow_$AGENTTEAM_RUN_ID"

"$AGENTTEAM_BIN" ready report \
  --runtime-home "$RUNTIME" \
  --sender Alice \
  --team default \
  --agent-name Alice \
  --body ready \
  --json

"$AGENTTEAM_BIN" msg send \
  --runtime-home "$RUNTIME" \
  --from Kevin \
  --to Alice \
  --action assign \
  --body "Claim and finish task AT-000001" \
  --json

"$AGENTTEAM_BIN" task send \
  --runtime-home "$RUNTIME" \
  --team default \
  --created-by Kevin \
  --target-kind role \
  --target builder \
  --title "agent workflow smoke" \
  --body "Use AgentTeam CLI to mark this task done" \
  --json

"$AGENTTEAM_BIN" task claim \
  --runtime-home "$RUNTIME" \
  --worker-name Alice \
  --worker-role builder \
  --json

cd ~/code/playground

"$AGENTTEAM_BIN" control headless-run \
  --agent Alice \
  --team default \
  --session "$HEADLESS_WORKFLOW_SESSION" \
  --input "$AGENTTEAM_BIN task done --runtime-home $RUNTIME --task AT-000001 --actor Alice --detail done --json. Then reply with one short summary." \
  --json

"$AGENTTEAM_BIN" task status \
  --runtime-home "$RUNTIME" \
  --task AT-000001 \
  --json

"$AGENTTEAM_BIN" report flow \
  --runtime-home "$RUNTIME" \
  --json

"$AGENTTEAM_BIN" control headless-stop \
  --agent Alice \
  --team default \
  --session "$HEADLESS_WORKFLOW_SESSION" \
  --json
```

Expected result:

- `task status` shows `AT-000001` as `done`
- `report flow` shows ready, message, task create, claim, and done steps
- the generated `ascii_flow` can be copied directly into a text report
- the generated `mermaid_flow` can be rendered by Mermaid-compatible tools

## Current User Commands

Most current tests use these commands:

```text
agentteam ready report
agentteam msg send
agentteam msg broadcast
agentteam task send
agentteam task claim
agentteam task status
agentteam task done
agentteam task error
agentteam report flow
agentteam tmux loopback
agentteam control headless
agentteam control headless-run
agentteam control headless-status
agentteam control headless-stop
```

All current MVP commands require `--json`.

## Useful Docs

- User and role guide: `docs/usage/agentteam-usage.md`
- Architecture overview: `docs/architecture/overview.md`
- ASCII architecture flows: `docs/architecture/ascii-flows.md`
- Function map: `docs/architecture/function-map.md`
- Verification map: `docs/architecture/verification-map.md`
- Report Flow module: `docs/modules/20-report-flow.md`

## Verified Baseline

The latest verified baseline includes:

- full workspace tests
- clippy with warnings denied
- architecture red tests
- function-map gate
- code-size gate
- skill-frontmatter gate
- required-file gate
- resource-lifecycle gate
- real `report flow` smoke from a persisted workflow log

Run the full verification suite:

```text
cargo xtask verify
```
