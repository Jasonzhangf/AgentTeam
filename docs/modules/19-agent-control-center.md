# 19 Agent Control Center

## Purpose

Agent Control Center owns the explicit single-agent control plane for the AgentTeam framework.

It selects whether a given agent runs in `attach_tui` mode or `headless` mode, binds that mode to the correct transport adapter, and exposes one uniform control surface for input, output, pause, stop, resume, wait, and retry-dispatch actions.

This module is the place where the framework decides how to control one agent at a time. It does not own task truth, message truth, or naming truth. It consumes those facts and turns them into session/control actions.

## Owns

- Single-agent control mode selection.
- `attach_tui` vs `headless` binding.
- Session acquisition and session resume control.
- Codex SDK bridge process launch/status/run/interrupt/stop control.
- Input envelope routing for one agent.
- Output observation routing for one agent.
- Pause / interrupt control.
- Stop / close control.
- Wait-for-result control.
- Retry-dispatch control for recoverable agent execution failures.
- Control-plane debug snapshot.
- Control-plane help text.

## Does Not Own

- Agent name allocation.
- Team/task/message semantics.
- Config TOML parsing.
- Error classification.
- Event-log append implementation.
- tmux/zterm transport implementation.
- TUI provider signal normalization.
- Runtime task queue ownership.
- Cross-daemon domain parsing.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `agent.mode_select` | Agent Control Center | Select `attach_tui` or `headless` for one agent | agent facts + launch intent | control mode binding | implicit fallback mode |
| `agent.acquire_session` | Agent Control Center + tmux/SDK adapter | Acquire or resume one agent session | session request | session binding | missing session binding |
| `agent.bind_transport` | Agent Control Center + tmux/tui adapter | Bind the selected mode to the correct transport adapter | mode binding | transport binding | mode bound to wrong adapter |
| `agent.sdk_start_session` | Agent Control Center + Codex SDK bridge | Start or resume a persistent headless SDK bridge session | control session + project scope | headless bridge response | missing SDK env |
| `agent.sdk_read_status` | Agent Control Center + Codex SDK bridge | Read a headless SDK session status from the live bridge process | control session | headless status response | thread read failed |
| `agent.sdk_run_turn` | Agent Control Center + Codex SDK bridge | Run one headless turn through the live SDK bridge process | control session + prompt | final response receipt | runtime turn failure |
| `agent.sdk_interrupt_turn` | Agent Control Center + Codex SDK bridge | Interrupt one active headless turn through the live SDK bridge process | control session | interrupt receipt | missing active turn id |
| `agent.sdk_stop_session` | Agent Control Center + Codex SDK bridge | Stop the persistent headless bridge process by scoped session | control session | stop receipt | orphaned bridge process |
| `agent.send_input` | Agent Control Center | Send one typed input operation to one agent | input op envelope | control receipt | raw string bypass |
| `agent.observe_output` | Agent Control Center | Observe output/evidence for one agent | output op envelope | control observation | output hidden from control plane |
| `agent.read_status` | Agent Control Center + Agent Registry + TUI Adapter | Read single-agent control/status facts | session facts + adapter facts | control status projection | state inferred only from stdout |
| `agent.pause` | Agent Control Center | Pause or interrupt one active agent turn/session | pause intent | pause receipt | pause not propagated |
| `agent.stop` | Agent Control Center | Stop or close one active agent session | stop intent | stop receipt | stop not propagated |
| `agent.wait` | Agent Control Center | Wait for a single agent result or stable pending state | wait intent | wait projection | forced timeout-as-error |
| `agent.retry_dispatch` | Agent Control Center + Task Engine | Re-dispatch a recoverable execution failure | retry intent | retry receipt | retry without error fact |
| `agent.report_error` | Agent Control Center + Error Center | Report control-plane faults to Error Center | control fault fact | error receipt | swallowed control fault |
| `agent.snapshot` | Agent Control Center | Provide control-plane snapshot to Debug Center | control state | snapshot | private session leak |
| `agent.help` | Agent Control Center | Describe attach/headless control and agent IO rules | help topic | help model | hidden mode fallback |

## Module Help Contract

Required help topics:

```text
agentteam help agent-control
agentteam help agent-control attach
agentteam help agent-control headless
agentteam help agent-control input
agentteam help agent-control output
agentteam help agent-control pause
agentteam help agent-control stop
agentteam help agent-control wait
agentteam help agent-control retry
agentteam help agent-control status
agentteam help agent-control red-tests
```

Help content must explain:

- one agent at a time is the scope of this module
- `attach_tui` keeps a visible tmux session for human observation and manual intervention
- `headless` uses a local persistent Codex SDK bridge process for automatic control of agent execution and dispatch
- `headless` uses the Codex SDK bridge driven by `AGENTTEAM_CODEX_SDK_SRC` and `AGENTTEAM_CODEX_BIN`
- the framework chooses the control mode explicitly; silent fallback between modes is forbidden
- the same typed input surface is used for the configured manager and workers
- input is always a typed operation, not a raw hidden string
- output is always observed through a control-plane envelope, not direct state access
- pause/stop/wait/retry actions are framework operations, not agent self-interpretation
- tmux session names, pane ids, SDK internals, and headless state file paths remain hidden from agent-facing help
- retry dispatch is only for recoverable execution failures and must be backed by an error fact

Help content must not:

- tell agents to bypass the control plane and write raw stdin themselves
- suggest mode fallback if one mode fails
- expose tmux pane/session ids
- expose SDK session internals as required agent behavior
- conflate agent control with task truth or message truth

## Public API Boundary

```text
AgentCtlReq01ModeIntent -> AgentCtlReq02ResolvedMode -> AgentCtlReq03SessionBinding -> AgentCtlReq04ControlAction -> AgentCtlResp05ControlProjection
AgentCtlReq01ModeIntent -> AgentCtlReq06AttachTuiBranch -> AgentCtlReq07TmuxBinding -> AgentCtlResp05ControlProjection
AgentCtlReq01ModeIntent -> AgentCtlReq08HeadlessBranch -> AgentCtlReq09SdkBinding -> AgentCtlResp05ControlProjection
ControlSessionInput -> headless_process::start_session -> HeadlessBridgeResponse -> AgentCtlResp05ControlProjection
ControlSessionInput -> headless_process::session_status -> HeadlessBridgeResponse -> AgentCtlResp05ControlProjection
ControlSessionInput + prompt -> headless_process::run_turn -> HeadlessBridgeResponse -> AgentCtlResp05ControlProjection
ControlSessionInput -> headless_process::interrupt_turn -> HeadlessBridgeResponse -> AgentCtlResp05ControlProjection
ControlSessionInput -> headless_process::stop_session -> HeadlessBridgeResponse -> AgentCtlResp05ControlProjection
```

Only Agent Control Center chooses the control mode for a single agent.
Only zterm/tmux Adapter executes tmux transport.
Only the local Codex SDK bridge executes headless Codex control.
Only Agent Registry projects final lifecycle status from facts.

## Required Behavior

- Bind a single agent to exactly one control mode at a time.
- Support explicit `attach_tui` mode.
- Support explicit `headless` mode.
- Use tmux as the transparent carrier for `attach_tui`.
- Use SDK-style agent control for `headless` through the local persistent Codex SDK bridge, with `AGENTTEAM_CODEX_SDK_SRC` and `AGENTTEAM_CODEX_BIN` set explicitly.
- Keep one live bridge process per headless session so the Codex SDK client, loaded thread, and turn notification queue stay in one runtime process.
- Treat `control headless` as create/bind for the MVP headless agent session.
- Treat `headless-status` or `headless-run` after scoped `headless-stop` as recovery only when the bridge resumes the same persisted `thread_id`.
- Project normal scoped stop as `offline`, not as a framework error.
- Keep the same agent identity across both modes.
- Send typed input envelopes through the selected control path.
- Observe typed output/evidence through the selected control path.
- Pause one active agent turn/session on request.
- Stop or close one active agent session on request.
- Wait for a single agent result or pending state on request.
- Retry a recoverable execution failure only after an explicit error fact exists.
- Treat a live agent with outstanding work but no final semantic reply as `busy` or pending, not as `error`.
- Keep mode selection explicit; no fallback to another mode without a new control decision.

## Control-Mode Flow

```text
User / Manager / Runtime
      |
      v
+---------------------------+
| Input Gateway             |
| typed control intent      |
+-------------+-------------+
              |
              v
+---------------------------+
| Agent Control Center      |
| mode select + session bind|
+-------------+-------------+
      |                       |
      | attach_tui            | headless
      v                       v
+---------------------+   +----------------------+
| zterm/tmux Adapter   |   | SDK Bridge Process   |
| tmux stdin/stdout    |   | live client/thread    |
+----------+----------+   +-----------+----------+
           |                          |
           v                          v
+---------------------+   +----------------------+
| Visible TUI Agent   |   | Headless Agent       |
| process/session     |   | execution session    |
+----------+----------+   +-----------+----------+
           |                          |
           +-----------+--------------+
                       |
                       v
+-----------------------------+
| Output Gateway / Runtime    |
| status projection, receipts |
+-----------------------------+
```

## Error Behavior

- Control-plane failures become Error Center facts.
- Mode-selection failure is explicit.
- Session-binding failure is explicit.
- Input delivery failure is explicit.
- Output observation failure is explicit.
- Pause/stop/wait/retry failures are explicit.
- A missing SDK source path, codex binary, HOME, or headless script is a hard `HeadlessUnavailable` error.
- A bridge/runtime/response failure is a hard `HeadlessBridge` error.
- A stale bridge port, missing pid, or failed live-client request is a hard `HeadlessBridge` error and must not fall back to a per-command SDK process.
- `headless` no longer pretends success when the bridge fails.

## Debug Snapshot

Snapshot includes:

- selected mode per agent
- adapter type bound to that agent
- headless project slug
- session binding id or thread id, redacted as required
- last turn id when present
- last input operation id
- last output observation id
- last pause/stop/wait/retry action id
- last control-plane error id
- control-plane resource counts

## Resource Lifecycle

Agent Control Center owns lifecycle requests for:

- `agent_control_session`
- `attach_tui_binding`
- `headless_sdk_binding`
- `headless_bridge_state_dir`
- `headless_bridge_process`
- `headless_bridge_socket`
- `headless_thread_handle`
- `headless_turn_handle`
- `input_operation_handle`
- `output_observation_handle`
- `pause_handle`
- `wait_handle`
- `retry_dispatch_handle`

Rules:

- Register a control-session lease before the agent is considered bound.
- Register exactly one active transport binding per controlled agent.
- Release the binding when the agent stops, switches mode, or the scope closes.
- Stop the headless bridge process by scoped session command when the project/session closes.
- Release headless bridge state files when the project/session closes.
- Release observation handles after the output has been consumed.
- Release wait handles when a wait completes or is interrupted.
- Retry dispatch must not leak the old failed control binding.
- A control session without a matching owner/session lease is an orphan candidate.
- A binding without an active owner/session is a leak candidate.
- Control-plane counts, wait latency, retry count, and open binding count are efficiency metrics.

## Red Tests

- Mode fallback without explicit decision fails.
- `headless` without SDK binding fails.
- `headless` start/status/run/interrupt/stop require explicit bridge env and state-path evidence.
- `headless` run must reuse a live bridge process and must not create a per-command SDK runtime.
- `headless` recovery must reuse the persisted `thread_id` and show a new bridge process after stop.
- normal scoped stop projected as `error` fails.
- headless response parsing failures are explicit bridge errors.
- `attach_tui` without tmux binding fails.
- Input sent outside the typed control envelope fails.
- Output read directly from private state fails.
- Pause not propagated to the active control session fails.
- Stop not propagated to the active control session fails.
- Wait forced into error instead of pending/busy fails.
- Retry dispatch without an error fact fails.
- Control-plane snapshot leaks session internals fails.
- Control-plane resource without lease fails.
- Control-plane binding leak fails.

## Open Decisions

- Whether `headless` can be selected for any agent or only for Codex-capable agents.
- Whether mode switching mid-session is allowed, or requires stop and re-bind.
- Whether input/output envelopes are shared with Startup Manager or live only here.
