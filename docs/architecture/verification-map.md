# Verification Map

This map links every critical feature to required tests and build gates.

## Global Gates

Future full gate:

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

Current no-code phase gate:

```text
find docs -type f | sort
git status --short
```

## Feature Gates

| feature_id | Unit tests | Contract/red tests | Integration/smoke |
|---|---|---|---|
| `architecture.gate` | xtask scanner unit intent through cargo xtask commands | function registry missing entry fails; red-test scans block owner bypass | full xtask gate sequence |
| `contract.pipeline` | pipeline node naming and adjacent contract tests | non-adjacent conversion helpers rejected by review/gate | contracts compile through workspace |
| `config.center` | parse TOML, normalize user config, reject duplicate names, reject agent count mismatch | no module can parse config directly; runtime-state keys in user config fail | load `~/.agentteam/config.toml` or sample config |
| `error.center` | classify module fault facts, assign severity, generate code, generate evidence_id | no success-wrapped error, no swallowed error, every error persisted, evidence fetchable | CLI shows explicit error with code/severity/evidence_id |
| `comm.center` | route message/broadcast/task/ready/claim/task-board envelopes, enforce sender scope | no adapter-owned business routing; no task priority ownership; every delivery persisted; no partial broadcast success | super manager publishes task, worker ready/query/claim/update, broadcast all through daemon |
| `domain.registry` | validate local/remote daemon domains, parse `agent@domain`, resolve domain route plan | no Communication Center domain parsing; no globally unique local agent names; no remote lookup fallback to local; no token leak | route message from `Kevin@local` to `Alice@review-daemon` through resolved daemon domain |
| `gateway.input` | parse CLI/API/UI/WebUI raw input | no non-adjacent conversion; UI cannot bypass Input Gateway | CLI and UI command smoke |
| `gateway.output` | render response/error/debug/UI projection output | no module final text rendering; UI consumes projections only | CLI output and UI projection snapshot |
| `gateway.ui` | UI/WebUI projection consumption and input submission | UI cannot mutate runtime internals; UI cannot call framework modules directly | render one agent from projection |
| `agent.naming_pool` | fixed Kevin manager, 20-name worker pool, overflow names, TA session name build, status projection | non-TA group operation rejected; unauthorized capability rejected; multiple managers rejected; SDK-only status rejected | list scoped sessions, capabilities, statuses |
| `team.orchestration` | create team/member, route command | duplicate owner rejected | create team with roles |
| `task.engine` | per-role serial queue, cross-role concurrent queue, assigned/role-matching claim, assigned-first and blocked-first ordering, task board projection | normal agent-reported task failure stays task state; framework/agent runtime fault enters Error Center; Communication Center cannot decide priority | dispatch two roles, claim assigned/blocked priority task, report task failure |
| `debug.center` | collect and persist module snapshots | private-state access rejected; print-only debug capture rejected; every debug bundle has persistence receipt | save debug bundle and fetch it by bundle id |
| `persist.event_log` | append/replay, snapshot rebuild | direct state-file write rejected | daemon restart restores state |
| `adapter.zterm_tmux` | build terminal requests, observe stdout/buffer, detect launch/session/transport faults | runtime direct tmux shell rejected; adapter cannot own task/status truth; SDK-only status rejected | launch TUI, send input, read output, observe missing session |
| `adapter.tui_agent` | select provider adapter, normalize provider status signals, support generic TUI agent | stdout-only final status rejected; Codex SDK not universal; provider payload cannot leak | generic shell adapter signal, Codex adapter diagnostic signal |
| `startup.session` | init project session, init Kevin, spawn workers, build typed input/output ops | Kevin not persistence truth; no direct tmux; session path under home; startup events persisted | startup init, Kevin ready, worker spawn |
| `tanote.board` | append ordered notes, validate note block format, thread/reply projection, build agent-visible envelopes | direct manual note mutation rejected; notes cannot mutate task truth; missing from/to/action rejected; no raw tmux payload | Kevin posts task thread, Alice replies, Bob adds evidence, projected thread remains ordered |
| `resource.lifecycle` | register leases, track owner/scope/refcount/ttl/metrics, release handles, cleanup temp files, detect orphan/leak/unbounded growth | long-lived resource without lease rejected; broad cleanup rejected; leak/orphan must persist event and debug evidence; unbounded growth visible | spawn agents, close daemon/session, cleanup tracked temp files, orphan scan reports exact leaked handle |
| `cli.agent_skill` | CLI command mapping and agent-facing help | skill hidden-wire dependency rejected; tmux/session internals hidden from agents; Kevin guidance complete; TANote commands documented | Kevin initializes, publishes task, messages worker, waits by task status, reviews TANote thread |

## Build Regression Rule

Every compile gate must run regression tests. No “compile only” success may be reported as feature completion.
