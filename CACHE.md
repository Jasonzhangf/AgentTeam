# CACHE

2026-06-08:
- Use `agentteam-dev` for repo development. Required truth docs: `docs/goals/mvp-runtime-vertical-slice-plan.md`, `docs/architecture/function-map.md`, `docs/architecture/verification-map.md`.
- Hard gates: every Rust function under `crates/` and `xtask/src/` must be in `docs/architecture/function-map.md`; required new files must be tracked before `verify-required-files`; hand-written Rust leaf files stay under 500 lines.
- Completed commits: `2196768` Phase 0/1 gates/contracts, `d081d1d` Config Center, `17f08f6` Domain Registry, `b82467d` Persistence Event Log, `7a18c69` Error Center, `6a68152` Debug/Resource, `4c76dbf` parse-only CLI.
- Current staged slice: local owner execution for `config check`, `domain resolve`, `debug snapshot`.
- New contract truth: `TeamReq01CliRaw`, `TeamReq02ParsedCommand`, and `TeamReq03ValidatedIntent` moved to `agentteam-contracts::team`; Gateway reuses them and does not own business execution.
- New runtime truth: `agentteam-runtime::local::execute_local_intent` consumes validated intents and calls owner APIs only: Config Center for config, Config Center + Domain Registry for domain route, Debug/Resource/Persistence for debug snapshot.
- New CLI truth: `agentteam-cli` is parse -> local runtime execute -> Output Gateway render. CLI does not parse TOML/domain targets or write event files directly.
- Verification passed after implementation: `cargo xtask verify`.
- Smoke passed: config check returns normalized config JSON; domain resolve returns remote route plan for `Alice@review-daemon`; debug snapshot writes `target/agentteam-smoke/events/agentteam.jsonl` with 3 events.
- Still out of scope: no daemon loop, no tmux/zterm launch, no real TUI agent startup, no Task/Comm/TANote MVP.
- Next main phase: Task/Comm/TANote MVP from Phase 8, unless choosing daemon-check skeleton first.
