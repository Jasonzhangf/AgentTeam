# note

2026-06-08 exploration:
- Repo was empty and not a git repository before initialization.
- `/Volumes/extension/code/zterm` contains zterm daemon/server truth under `android/src/server`.
- zterm mac local tmux transport is Electron-local polling and not the shared daemon truth.
- AgentTeam should reuse zterm-compatible terminal protocol/adapter, not copy daemon code.
- Config Center discussion: user config lives at `~/.agentteam/config.toml`, must keep comments, stores project-related user config only, includes project, team category, agent count, member role, launch command/args/env/cwd; runtime/task/message/debug state is forbidden in config.
- Module docs now require a module-local function map and help contract. Config Center and Error Center docs include these sections.
- Error Center discussion: all errors must persist to event log; code format is `<module>.<class>.<specific>.<time>.<seq>`; every error has independent `evidence_id` fetchable by Debug Center; severity values are `fatal/error/warn/info`; normal agent-reported `task error` is task failure, while agent process/session/framework faults are framework errors handled by Error Center.
- Resolved wording conflict: normal `agentteam task error` stays Task Engine task state; framework/agent runtime faults enter Error Center.
- Communication Center discussion from user screenshot: daemon is bidirectional communication entry; manager publishes/manages/query task board through daemon; worker reports ready, queries board, claims priority task, updates/completes through daemon; Communication Center routes envelopes and persists delivery, but Task Engine owns task board, priority, blocking, and claim result; Agent Registry owns manager/worker capability and task owner facts.
- Communication Center decisions: broadcast required; message target supports exact agent/role/team/all members; v1 has exactly one super manager; worker claim only assigned or role-matching tasks; Task Engine claim ordering is assigned first and blocked first inside same claim class; Communication Center and Task Engine must stay separate.
- Agent Registry decisions: manager name fixed as Kevin; worker name pool has 20 English names; worker 21+ uses `<project_slug>_worker_<seq>`; TUI launch without framework/transport error plus TA session existence marks ready and projects idle when no task active; steady statuses are offline/starting/idle/busy/error; generic status truth uses tmux/zterm/runtime/task/error facts, not Codex SDK; session metadata path is `~/.agentteam/sessions/<project_slug>/`.
- Status model refinement: user worried stdout-only status is too coarse. Added TUI Agent Adapter Center to normalize provider-specific TUI signals. tmux/zterm stdout is transport/evidence; Codex SDK can be provider-specific diagnostic only; final status is projected by Agent Registry/Runtime from adapter signals + task + error facts.
- UI/WebUI decision: UI only consumes Input Gateway and Output Gateway projections; it owns ephemeral view state only and must not mutate/read agent framework internals, Task Engine, Communication Center, Persistence, or tmux/zterm directly.
- Startup/session discussion: user wants current TUI initializes Kevin, Kevin spawns other agents via tmux initial commands, and agents use skills/CLI. Added Startup Session Manager. Kevin is operator/bootstrap, not persistence truth. Session metadata lives under `~/.agentteam/sessions/<project_slug>/`. Agent input/output must be typed operations routed through gateways/adapters, not raw hidden strings.
- Encapsulation decision: tmux/session details are invisible to agents. Agents know names/roles/team/tasks/messages and use CLI/skills only. tmux session names, pane ids, session descriptor paths, zterm endpoints, daemon wire, and event paths are framework internals.
- Kevin skill decision: Kevin can read AgentTeam skill and must learn init/query/task publish/message/broadcast/wait/debug operations. Kevin waits by task/message/status projections, not by reading child tmux/session internals.
- TANote discussion requirement: each agent should write work notes through `agentteam note post`, producing project `TANote.md` as a forum-style collaboration projection. TANote entries require from/to/action/thread/note ids and daemon sequence. `TANote.md` is readable collaboration material, not task/message/event truth; task state still requires Task Engine commands. Tmux delivery uses an agent-visible AgentTeam envelope and hides tmux/session/zterm internals.
- User requested MVP debug sufficiency plus per-module resource lifecycle management. Added direction: MVP debug must include resource lifecycle evidence. Every long-lived resource needs owner/scope/lease/release policy, orphan/leak detection, event persistence, debug snapshot, exact-handle cleanup, and efficiency budget reporting.
- User decided debug is persisted for v1: debug bundles/evidence/snapshots must be materialized with persistence receipts before output. No print-only/no-save debug capture path.
- User decided v1 memory policy: no aggressive hard memory caps for now, but queues/cursors/buffers/temp files/projections must have bounded growth and cleanup paths.
- User decided daemon/session close must run scoped resource cleanup and temporary file cleanup through exact tracked handles.
- User clarified function map is a hard rule. Added direction: no feature implementation/modification without function map and verification map coverage.
- User asked about 500-line file limit. Decision: apply 500-line hard limit to hand-written Rust leaf files, including shared blocks/contracts; split shared code instead of exempting it. Docs/fixtures/generated files are outside this gate.
- User corrected that the per-module development cycle should be a separate `agentteam-dev` local skill, not mixed into the runtime/CLI `agentteam` skill.
- User clarified zterm supports multiple daemons and AgentTeam must support cross-daemon communication. Decision: add Daemon Domain Registry as unique owner for daemon domain ids, `agent@domain` parsing, and cross-daemon route plans; Agent Registry names are domain-local only.
- 2026-06-08 execution planning resume: loaded `agentteam-dev`, `AGENTS.md`, `CACHE.md`, `MEMORY.md`, `note.md`, architecture overview/flows/function-map/verification-map/MVP start gate, and `docs/goals/mvp-runtime-vertical-slice-plan.md`. Current executable development should begin with Phase 0 + Phase 1 only: tighten `xtask` architecture red tests and add typed contracts in `agentteam-contracts`; no business runtime, no real daemon start, and no tmux/zterm execution yet. Current uncommitted files observed: `xtask/src/main.rs`, `xtask/src/red_tests.rs`, and `docs/goals/mvp-runtime-vertical-slice-plan.md`.
- 2026-06-08 Phase 0/1 execution result: implemented static red-test scans, function-level function-map gate, and typed contracts for config/domain/persist/error/debug/resource. Added canonical `cargo xtask verify` so function-map verification runs before compile/test gates. Verified commands passed: `cargo xtask verify`; expanded gates include `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` with 13 contract tests, `cargo xtask red-tests`, `cargo xtask verify-required-files`, `cargo xtask verify-skill-frontmatter`, `cargo xtask verify-resource-lifecycle`, `cargo xtask verify-function-map`, `cargo xtask verify-code-size`. Required new files were staged so required-file tracking gate can pass.
- 2026-06-08 Phase 2 Config Center result: after committing Phase 0/1 as `2196768`, implemented Config Center load/parse/validate/normalize/snapshot in owner crate only. Added `serde` and `toml` dependencies to `agentteam-config`; `toml::from_str` appears only in `crates/agentteam-config/src/parse.rs`, while red-test scanner still scans for outside-owner TOML use. Verified `cargo xtask verify` passed with 7 Config Center tests and 13 contracts tests.
- 2026-06-08 Phase 3 Domain Registry result: after committing Phase 2 as `d081d1d`, implemented `agentteam-runtime/src/domain` owner modules. Domain Registry registers local/remote domains, enforces id/alias uniqueness, resolves `agent@domain`, `role:<role>@domain`, `team:<team>@domain`, `all@domain`, resolves bare targets to local only, and rejects missing remote domains without local fallback. Verified `cargo xtask verify` passed with 7 runtime domain tests; static audit showed `split('@')` only in Domain Registry owner.
- 2026-06-08 Phase 4 Persistence Event Log start: target owner is `agentteam-persist`; module doc requires typed JSONL append, receipt, replay, corruption failure, materialized state summary, and Persistence-only state-file writes. Added owner modules for error/model/append/replay/materialize/tests and will gate them through required-files, function-map, and full `cargo xtask verify`.
- 2026-06-08 Phase 4 Persistence Event Log result: implemented typed JSONL append/replay/materialize in `agentteam-persist`; append returns `PersistResp03AppendReceipt`, replay can start from sequence, corrupt JSONL and sequence mismatch fail explicitly, empty draft fails validation. Added new owner files to required-files and function-map. Verified `cargo xtask verify` passed with 7 Persistence tests plus existing Config/Contracts/Domain tests.
- 2026-06-08 Phase 5 Error Center start: owner is `agentteam-error`; module doc requires every framework fault to be classified with severity/code/evidence id and persisted through Persistence before projection. Normal agent-reported task errors must not be treated as framework errors. No direct file writes in Error Center.
- 2026-06-08 Phase 5 Error Center result: implemented Error Center owner modules for classification, code/evidence generation, persistence append, and projection. Public API is narrowed to `handle_framework_fault` so callers cannot project before persistence. Tests cover severity/code, evidence id, persisted framework error event payload, malformed code seed, normal task-error rejection, and persistence failure not becoming success.
- 2026-06-08 Phase 6 Debug/Resource result: implemented `agentteam-resource` lease registry with acquire/release/leak snapshot and persisted resource lifecycle events. Implemented `agentteam-debug` bundle capture that acquires a debug_bundle lease, consumes Resource public snapshot, persists debug_bundle event, releases the lease, and returns `DebugResp03Bundle` with persistence receipt and resource snapshot id. Verified `cargo xtask verify` passed with 5 Resource tests and 3 Debug tests.
- 2026-06-08 Local CLI parsing result: user agreed to run local parsing clearly before first real agent launch. Implemented `agentteam-gateway` local argv parsing for `config check`, `domain resolve`, and `debug snapshot`, plus JSON rendering that marks `local_parse_only: true`. `agentteam-cli` now delegates argv parsing/rendering to Gateway. Verified `cargo xtask verify` and three smoke commands: config/domain/debug parsing all returned `TeamReq03ValidatedIntent` JSON without daemon/tmux startup.
- 2026-06-08 Local owner execution start: parse-only CLI slice has been reverified with cargo xtask verify and three smoke commands. Next slice will move TeamReq request contracts to shared contracts where needed, add a runtime local executor that consumes validated intents, call only Config/Domain/Debug public APIs, and keep CLI/Gateway as thin parse/render boundaries without daemon/tmux/zterm startup.
- 2026-06-08 Daemon check skeleton start: next Phase 7 slice adds `agentteam daemon check --config <path> --json` as a local routeability smoke. It must parse through Gateway, execute through Runtime local orchestration, call Config Center and Domain Registry public APIs only, render through Output Gateway, and explicitly report daemon/tmux/zterm are not started or touched.

2026-06-08 Phase 8 Task Engine start: scope is local persistent task event truth only. Implement Task Engine owner under agentteam-runtime/src/task, backed by agentteam-persist replay/append. CLI slice will add task send/list/status/done/error through Gateway parse -> Runtime local executor -> Output Gateway render. No daemon, tmux, zterm, Communication Center, or TANote mutation in this slice.

2026-06-08 Phase 8 Task Engine result: implemented local persistent task event truth in agentteam-runtime::task with CLI task send/list/status/done/error. Added shared event_payload_hash in contracts and removed duplicate hash helpers from debug/error/resource. Verified cargo xtask verify and CLI smoke send/list/done/status with two persisted task events.

2026-06-08 tmux loopback slice start: user prioritized proving multiple tmux sessions can exchange stdin/stdout before further Task/Comm work. Scope: adapter-owned real tmux smoke command that creates multiple managed TA sessions, injects input, captures output evidence, and cleans sessions by exact names. Runtime and CLI must not call tmux directly; no daemon/zterm launch yet.
2026-06-08 tmux loopback slice result: implemented `tmux loopback` vertical smoke through `agentteam-tmux` + runtime/local CLI. Real smoke passed with 2 sessions: ready/input/output observed on both, `cleanup_status=cleaned_exact_handles`, and no residual `TA-agentteam-tmux-smoke-loopback*` sessions after exact cleanup. Key fix: tmux pane capture needed `capture-pane -J` to join wrapped lines; launch required `sh -lc` wrapper.
2026-06-08 Communication Center slice result:
- Added typed ready / task-board query / task claim pipeline nodes in `agentteam-contracts::comm` and routed them through `agentteam-comm`.
- `agentteam-comm` now exports `route_ready_report`, `route_task_board_query`, and `route_task_claim`, with validation-only routing and explicit acceptance results.
- Split `crates/agentteam-contracts/src/comm/mod.rs` test block into `crates/agentteam-contracts/src/comm/tests.rs` to satisfy the 500-line hard limit.
- Verified with `cargo test -p agentteam-contracts -p agentteam-comm`, `cargo xtask verify-function-map`, and full `cargo xtask verify`.
2026-06-08 Task Engine claim slice result:
- Added task claim truth to `agentteam-runtime::task` with `priority`/`blocked` carried in task records and event payloads.
- `TaskEngine::claim_task` now selects assigned tasks before role-matching tasks, with blocked-before-unblocked ordering inside the same claim class and explicit `task_claimed` persistence.
- Added CLI/Gateway support for `task claim --runtime-home ... --worker-name ... --worker-role ... --json` and a local runtime smoke that claims a task, transitions it to running, then completes it.
- Verified with `cargo test -p agentteam-runtime -p agentteam-gateway`, `cargo xtask verify-function-map`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo xtask verify-code-size`, and `cargo xtask verify`.
2026-06-08 Communication Center delivery persistence slice start:
- Next executable slice is `comm.persist_delivery_event`: add a comm-owned JSONL append helper, keep route validation separate from persistence, and verify replayable delivery payloads against `agentteam-persist`.
- Communication Center needs explicit persistence failure classification so append errors do not collapse into validation.
- Function-map and verification-map entries must be added before/with the implementation because `cargo xtask verify-function-map` is a hard gate.
2026-06-08 Communication Center message-send slice result:
- Added `comm.send_message` plus `agentteam msg send --runtime-home ... --from ... --to ... --action ... --body ... --json`.
- The route now validates target, persists a `comm_message_delivery` JSONL event through `agentteam-persist`, and returns delivery metadata with event id/sequence/log path.
- Added gateway/runtime/output coverage for `msg send`, plus function-map and verification-map entries for the new path.
- Verified with `cargo test -p agentteam-comm -p agentteam-gateway -p agentteam-runtime`, `cargo xtask red-tests`, `cargo xtask verify-code-size`, `cargo xtask verify-function-map`, and `cargo xtask verify`.
2026-06-08 Communication Center ready-report slice result:
- Added `comm.send_ready_report` plus `agentteam ready report --runtime-home ... --sender ... --team ... --agent-name ... --body ... --json`.
- The route now validates ready reports, persists a `comm_ready_report_delivery` JSONL event through `agentteam-persist`, and returns replayable delivery metadata with event id/sequence/log path.
- Added gateway/runtime/output coverage for `ready report`, plus function-map, verification-map, red-test-plan, and startup/CLI docs updates for the worker-ready surface.
- Verified with `cargo fmt --check`, `cargo test -p agentteam-comm -p agentteam-gateway -p agentteam-runtime`, `cargo xtask red-tests`, `cargo xtask verify-code-size`, `cargo xtask verify-required-files`, `cargo xtask verify-skill-frontmatter`, `cargo xtask verify-resource-lifecycle`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo xtask verify-function-map`, and `cargo xtask verify`.
2026-06-08 Broadcast/pre-E2E slice result:
- Added `msg broadcast` delivery path in `agentteam-comm`, `agentteam-gateway`, and `agentteam-runtime`, plus matching docs/gates for the command surface and receipts.
- Split gateway parsing into `input.rs`, `broadcast.rs`, and `options.rs` so every hand-written Rust leaf file stays under 500 lines.
- Verified `cargo xtask verify-function-map`, `cargo xtask verify-code-size`, `cargo xtask red-tests`, `cargo xtask verify-required-files`, `cargo xtask verify-skill-frontmatter`, `cargo xtask verify-resource-lifecycle`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `cargo xtask verify`.
- Playground smoke in `~/code/playground` succeeded with runtime home `/Volumes/extension/code/playground/agentteam-smoke`: ready report, broadcast delivery receipt, task send, task claim, task done, and task status all returned JSON receipts with persisted event ids/sequences.
2026-06-09 E2E smoke result:
- Real local E2E smoke ran under `/Users/fanzhang/Documents/github/agentteam/agentteam-e2e` and `/Users/fanzhang/Documents/github/agentteam/agentteam-tmux-e2e`.
- Sequence passed: ready report, direct message, broadcast, task send/list/claim/done/status, second task claim/error/status, and tmux loopback.
- Task success path ended in `done` for `AT-000001`; task error path ended in `error` for `AT-000002`.
- tmux loopback reported `observed_count=2`, `cleaned_handle_count=2`, and `cleanup_status=cleaned_exact_handles`.
2026-06-09 Usage guide result:
- Added `docs/usage/agentteam-usage.md` as the unified role-based operating guide.
- Linked the guide from `agentteam` skill, `13-cli-agent-skill.md`, and `docs/architecture/file-structure.md`.
- The guide documents current real startup behavior, role reading order, Kevin workflow, worker workflow, and current E2E smoke sequence.
2026-06-09 startup clarification:
- Standard tmux is now the transparent bootstrap carrier in docs; `routecodex`/`rcc` startup wording was removed from the user-facing path.
- Kevin startup parameters assign Kevin identity; Kevin later initializes workers with their own tmux + role params through skills/CLI.
- Kevin and worker docs now emphasize CLI control, `ready report`, `msg send`, `msg broadcast`, task commands, and note commands rather than hidden tmux/session details.
2026-06-09 bootstrap refinement:
- Kevin is the only first-launch target for startup params; Kevin then initializes workers with their own tmux + params.
- Worker startup remains tmux + role together, but it is Kevin-issued after Kevin's own launch.
2026-06-09 startup implementation planning:
- `agentteam start` is the executable bootstrap entrypoint.
- Default scope is current `cwd`; config path can still default to `~/.agentteam/config.toml`.
- Implementation should launch Kevin through the tmux adapter, persist a startup/resource lease event, and return a JSON bootstrap report.
- Kevin workers remain later-issued by Kevin through skills/CLI; `start` does not directly spawn workers in v1.
2026-06-09 startup/status rule:
- Kevin output status cannot be forced by semantic reply.
- `stdout` is transport evidence; `busy` covers active work and pending request/response.
- `idle` requires live session + no active task/request; `error` requires launch/session/transport/framework fault.
- function map needed new entries for `start` parse/execute/startup helpers and tmux launch helpers.
2026-06-09 evidence retention rule:
- Do not clean Kevin test sessions/evidence during the test run.
- Cleanup must wait until the user explicitly exits Kevin; otherwise evidence is lost before the test concludes.
2026-06-09 Kevin root-manager rule:
- Kevin is the root manager of the project agent tree.
- User explicitly exits Kevin; Kevin must not auto-exit after startup.
- Kevin exit triggers scoped worker exit through shutdown flow.
- Kevin skills need a CLI feedback path to return execution results to the framework.

2026-06-09 stdin experiment:
- Confirmed live Kevin tmux session exists: `TA_local_agentteam_Kevin`.
- Manual `tmux send-keys -l <probe>` + `Enter` injected a unique marker into the Kevin pane; `tmux capture-pane -pt TA_local_agentteam_Kevin -J` showed the marker in the pane output.
- Current framework CLI does not expose a real agent-input command path yet: `agentteam startup input --help` returns unsupported command.
- Current `agentteam start` path launches Kevin, but the framework still lacks a public CLI command that routes a typed input envelope into Kevin's tmux stdin.

2026-06-09 control-plane design:
- Added a new `Agent Control Center` module doc as the single-agent control plane.
- The new module owns explicit `attach_tui` and `headless` mode selection, session binding, typed input/output routing, pause/stop/wait/retry, and control-plane snapshots.
- Startup now hands the live agent to Agent Control Center after bootstrap instead of owning per-agent input/output control.
- tmux adapter keeps transport ownership; TUI adapter keeps provider-signal normalization; Agent Control Center sits between them and the higher-level runtime/CLI orchestration.
2026-06-09 headless bridge discovery:
- `python3` cannot import `openai_codex` in the current environment (`ModuleNotFoundError`).
- `/Users/fanzhang/code/codex` does contain real Codex thread/turn/collab control surfaces, including `thread_start`, `thread_resume`, `thread_read`, `turn.run`, `turn.interrupt`, and collab tools `spawnAgent/sendInput/resumeAgent/wait/closeAgent`.
- A Rust headless bridge will need either a local SDK install or a separate bridge process; it is not available directly in the current workspace yet.
2026-06-09 control-plane split:
- `agentteam-runtime::local.rs` still carried `execute_control`; moved control execution into `agentteam-runtime::control` so the local orchestrator stays under the 500-line leaf-file cap.
- Function map must be updated to point `execute_control` / `control_error` at `crates/agentteam-runtime/src/control.rs`.
2026-06-09 Codex SDK bridge result:
- Confirmed local SDK source import works from `/Users/fanzhang/code/codex/sdk/python/src`.
- Confirmed public SDK surface includes `Codex(CodexConfig(codex_bin=...))`, `thread_start`, `thread_resume`, `thread.read(include_turns=False)`, `thread.turn(...).run()`, and `thread.turn(...).interrupt()` via the SDK client.
- The bridge now uses a project-scoped session dir under `~/.agentteam/sessions/<project_slug>/headless/<session_name>/state.json`.
- `headless` in Agent Control Center now routes through the bridge instead of returning explicit unavailable.
- Bridge failures are split from env/path failures: env/path launch issues map to `HeadlessUnavailable`, bridge/runtime/parse issues map to `HeadlessBridge`.
- `turn_interrupt` is not exposed as a public thread method in the SDK surface we inspected; the bridge uses the SDK client interrupt call for an existing turn id.

2026-06-09 persistent headless bridge plan:
- Verified Codex Python SDK `Codex` owns a live app-server client/process, and `Thread.turn(...).run()` consumes notifications through that live client. Persisting only `thread_id` in state.json is insufficient for later turns after the bridge process exits.
- Next implementation must keep one session-scoped Python bridge process alive per headless session, with Rust commands sent over a local request channel. `state.json` should hold pid/port/thread evidence, not replace the live SDK runtime.
- CLI needs explicit headless control actions (`headless-run`, `headless-status`, `headless-interrupt`, `headless-stop`) so AgentTeam can verify SDK turn execution through the framework instead of a standalone Python experiment.

2026-06-09 persistent headless bridge result:
- Implemented session-scoped persistent Codex SDK bridge process with localhost JSON request channel and state evidence under ~/.agentteam/sessions/<project_slug>/headless/<session_name>/state.json.
- First smoke failure `headless bridge did not become ready` was traced to ping response missing the full bridge response schema; fixed by returning full HeadlessBridgeResponse-shaped ping/error payloads.
- Second smoke failure `headless thread not started` after run/status was traced to new bridge processes not resuming state thread_id; fixed status/run/interrupt to load or resume the persisted thread_id before acting.
- Verified real AgentTeam CLI bridge smoke on TA_headless_bridge_smoke_4: headless-run returned details `ready`, headless-status returned idle/thread idle, headless-stop returned ok, and ps showed no remaining headless_bridge.py process. Evidence state remains persisted.

2026-06-09 create/control/recover + workflow verification start:
- Current MVP definition: `control headless` is agent create/bind for a headless Codex agent; `control headless-run` is the control input path; `headless-stop` stops the scoped bridge process; a later `headless-status` or `headless-run` must recover by spawning a new bridge process and resuming persisted `thread_id` from the session state file.
- Minimal workflow verification should not depend on deterministic model prose. Success truth is event-log/task/control projection evidence: ready report persisted, message persisted, task sent, worker claimed, worker headless response captured as evidence, task marked done, task status projects done.
- First workflow attempt reached ready/message/task send/task claim and Alice headless returned `done`, but the outer script's later `task done --detail "headless evidence returned done"` failed with `cannot transition from done to done`. Event log showed sequence 5 was already `task_done` with detail `done`, meaning the worker turn had completed the task itself. Revised workflow truth: worker is responsible for task completion via CLI; outer smoke verifies `task status` instead of reporting completion a second time.
- Second workflow attempt from repo cwd failed to let worker write `~/code/playground/...` through Codex SDK workspace sandbox (`Operation not permitted`), leaving the task running. Revised execution rule: for headless workflow smokes that write playground runtime state, invoke control commands from `~/code/playground` so the SDK workspace-write sandbox matches the runtime home.
- Third workflow attempt passed with runtime home `/Users/fanzhang/code/playground/agentteam-workflow-20260609-03` and session `TA_headless_Alice_workflow_20260609_03`: ready report sequence 1, message sequence 2, task_created sequence 3, task_claimed sequence 4, Alice headless final response `workflow-result: AT-000001 done by Alice, status ok.`, task_done sequence 5, final task.status `done`, and scoped headless-stop projected `offline`.

2026-06-09 visible Kevin TUI correction:
- User checked `TA_local_agentteam` / `TA_local_agentteam_Kevin` and did not see visible Kevin initialization evidence. Verified current startup only injects environment variables into the Codex TUI process; it does not send a visible bootstrap prompt that tells the TUI agent it is Kevin, which skill to read, how to manage tasks/workers, or how to launch worker sessions.
- Corrected startup requirement: Kevin's tmux Codex TUI session is the human-facing management session. A bootstrap prompt must be injected only for a newly created session. Existing sessions must not be reinjected because that pollutes active user conversation and evidence.
2026-06-09 startup lifecycle implementation note:
- Current partial `agentteam-startup/src/lib.rs` exceeded the 500-line Rust leaf limit and mixed models, config loading, selection, env, prompts, and session lifecycle in one file.
- Required implementation shape: session existence check must happen before resource acquisition, tmux launch, and prompt injection; existing sessions return `session_lifecycle=existing` and `bootstrap_prompt_status=skipped_existing_session`.
- User added agent-session recovery requirement: after creating Kevin or a child agent, Startup Manager must record that agent's own session id/binding. This is not the AgentTeam project session. Later starts must read the agent-session binding, verify the tmux session is still alive, and restore/bind instead of creating a duplicate. If the agent-session binding exists but the tmux session is dead, the record is stale evidence and startup may create a new agent session with explicit lifecycle status; no existing live agent session may be reinjected.
- Correction: the agent-session id is the Codex resumable thread/session id, not the tmux session id. Local evidence: `codex resume --help` accepts `SESSION_ID` and Codex CLI exit hint formats `codex resume <thread_id>`; Codex SDK docs expose explicit `thread_start(...)` and `thread_resume(thread_id, ...)`. tmux remains the visible carrier only.

2026-06-09 Codex session-id startup experiment:
- User clarified startup truth: create the Codex agent session first through SDK, obtain the Codex thread/session id, then start the visible TUI by running `codex resume <thread_id>` inside tmux. tmux id is only carrier identity.
- Experiment 1: SDK `thread_start(cwd=/Users/fanzhang/code/playground)` returned `019eaad5-596b-72c0-8554-7f1d6b5a6367`, but visible TUI `codex resume 019eaad5-596b-72c0-8554-7f1d6b5a6367` in `TA_sdk_bare_resume_probe_20260609_01` failed with `No saved session found`. Bare SDK thread creation is not enough for TUI resume.
- Experiment 2: SDK `thread_start` plus one completed seed turn returned thread id `019eaad6-49c3-7e31-964b-e9bcae139702` and `seeded-session-ok`; visible TUI `codex resume 019eaad6-49c3-7e31-964b-e9bcae139702` in `TA_sdk_seeded_resume_probe_20260609_01` loaded prior history and accepted a new prompt. SDK `thread_resume(...).read(include_turns=False)` on the same id returned status `idle`.
- Startup implementation implication: first launch must seed the SDK-created thread before tmux resume; subsequent launch must reuse persisted `agent_session_id` / Codex thread id and skip bootstrap reinjection when a live tmux carrier already exists.
2026-06-09 neutral manager naming correction:
- User clarified that `Kevin` is only the configured default agent name. Code functions, structs, schema fields, feature ids, and red-test ids must use neutral concepts such as root manager, configured manager, bootstrap agent, or manager. Literal `Kevin` may remain only as config/example/test data where the framework is exercising a configured agent name.

2026-06-09 visible TUI input correction:
- User caught a verification error: text visible in a Codex TUI prompt box is not proof that the agent received the prompt. A submit action requires Enter.
- Rechecked code truth: `crates/agentteam-tmux/src/control.rs::send_input` sends `tmux send-keys -l <input>` and then `tmux send-keys Enter`, so the adapter implementation is not text-only.
- Replayed the E2E sessions by sending exact `Enter` to `TA_local_agentteam_e2e_tui_20260609_01_Kevin`, `..._Alice`, and `..._Bob` without cleanup. Subsequent pane captures showed Codex processing/assistant output: Kevin reported the event sequence mismatch, Alice ran skill/CLI checks and reported the same mismatch, Bob produced reviewer checkpoint output.
- Correct verification rule: `control send` success needs both transport projection and post-submit evidence from `control observe` / `tmux capture-pane` showing the submitted user turn left the prompt buffer and caused a Codex turn/output. Mere prompt-buffer visibility is only input staging evidence.

2026-06-09 startup skill install + SDK status result:
- Implemented startup-owned local skill installation into target cwd at `.agents/skills/agentteam/SKILL.md` before seeding manager/worker prompts. Startup result now exposes `skill_install_status`, `skill_path`, and `cli_path`.
- Startup prompt/env now injects absolute `AGENTTEAM_CLI` and `AGENTTEAM_SKILL_PATH`, so workers in `~/code/playground` do not depend on PATH or repo-local skill discovery.
- Agent Control Center now lets attach_tui status/send/wait merge tmux observation with Codex SDK status when the SDK-created session binding exists. `control status --project <slug> --cwd <cwd>` returned `details="sdk_status=status; sdk_details=thread idle; tmux_observed=true; tmux_state=idle"` for `TA_local_agentteam_e2e_tui_20260609_01_Kevin`.
- Real smoke: starting with `--cwd /Users/fanzhang/code/playground/agentteam-skill-install-smoke-20260609` installed the skill into that cwd and returned `skill_install_status=installed`, `skill_path=/Volumes/extension/code/playground/agentteam-skill-install-smoke-20260609/.agents/skills/agentteam/SKILL.md`, and `cli_path=/Users/fanzhang/Documents/github/agentteam/target/debug/agentteam`.
- Gates passed after this slice: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `cargo xtask verify`.
2026-06-09 attach_tui SDK status hardening:
- Removed the swallowed SDK status probe path. For attach_tui status with `--cwd` + `--project`, Control Center now requires an existing persisted Codex thread binding and fails explicitly if state/status is missing.
- attach_tui status without SDK scope reports `sdk_status=not_requested`; it is tmux-only evidence by design, not a Codex session-state claim.
- `send`, `pause`, and `stop` remain transport-action projections and are not overwritten by SDK idle status. Submitted-work evidence still requires post-submit observe/capture.
- Added red-test scanner for the forbidden SDK-status downgrade markers and updated module/verification docs.
- Real smoke: `control status --session TA_local_agentteam_e2e_tui_20260609_01_Kevin --cwd /Users/fanzhang/code/playground/agentteam-e2e-tui-20260609-01 --project agentteam_e2e_tui_20260609_01 --json` returned `sdk_status=status; sdk_details=thread idle; tmux_observed=true; tmux_state=idle`.
- Negative smoke: `control status` with SDK scope for `TA_local_agentteam_Kevin` and no persisted binding returned explicit error reading the missing state file instead of downgrading to tmux-only status.
- User authorized cleanup of non-reusable test TA sessions. Exact tmux cleanup removed `TA_local_agentteam_Kevin_tui_start_smoke`, `TA_local_agentteam_e2e_tui_20260609_01_{Kevin,Alice,Bob}`, and `TA_sdk_seeded_resume_probe_20260609_01`; only reusable `TA_local_agentteam_Kevin` remains. Scoped `control headless-stop` removed residual bridge processes; process audit found no `headless_bridge.py`.
- Verification nuance: running `cargo test --workspace` and `cargo xtask verify` concurrently caused `agentteam-persist` test temp-file collision. Added process id to the persistence test temp path; serialized rerun passed.

2026-06-09 persistence concurrent append result:
- Root cause of prior E2E `event sequence mismatch: expected 7, got 6`: Persistence append computed next sequence before append without a cross-process critical section, so concurrent CLI/module writes could assign duplicate sequence ids.
- Implemented owner-local file exclusive lock in `agentteam-persist::append_event_log`: open/create/read/append one log file, lock it, replay locked content to compute next sequence, append, flush, then release by dropping the file handle.
- Added `concurrent_append_preserves_unique_sequence` with 16 concurrent Rust threads and `red.persist.concurrent_append_sequence` docs/gate coverage.
- Real CLI smoke after rebuilding `target/debug/agentteam`: 20 parallel `ready report` commands wrote `/Users/fanzhang/code/playground/agentteam-persist-concurrent-smoke-20260609162304/events/agentteam.jsonl`; result `line_count=20`, `unique_count=20`, `max_sequence=20`, `error_count=0`, sequences `1..20`.
- Verification passed: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p agentteam-persist`, `cargo test --workspace`, `cargo xtask verify-function-map`, `cargo xtask red-tests`, `cargo xtask verify-code-size`, `cargo xtask verify`.
