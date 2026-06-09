# Function Map

This map is the feature owner truth. If a feature is not listed here, implementation must not begin.

The hard gate is `cargo xtask verify-function-map`.

| feature_id | Owner module | Canonical contracts | Allowed paths | Forbidden paths | Required gates |
|---|---|---|---|---|---|
| `architecture.gate` | Architecture Gate / xtask | `xtask verify-*`, red-test scans | `xtask`, `docs/architecture/*`, `docs/red-tests/*` | runtime/business crates bypassing gates; unregistered Rust functions | xtask unit, function map gate, red tests |
| `contract.pipeline` | Contracts Pipeline | `PipelineNodeName`, typed node constructors | `agentteam-contracts`, `docs/architecture/function-map.md` | non-adjacent conversion helpers; duplicate DTOs outside contracts | contract unit, function map gate |
| `config.center` | Config Center | `ConfigReq*`, `ConfigResp*`, `ConfigErr*` | `agentteam-config`, `docs/modules/01-*`, `docs/config/config.toml.example` | runtime, gateway, CLI parsing TOML directly; runtime/task/debug state inside user config | config unit, config red tests |
| `error.center` | Error Center | `TeamErr*` | `agentteam-error`, contracts error chain | all modules rendering final error text | error unit, success-wrapped-error red test |
| `comm.center` | Communication Center | `CommReq*`, `CommResp*` | `agentteam-comm` | tmux adapter owning task/message semantics | routing unit, duplicate-owner red test |
| `domain.registry` | Daemon Domain Registry | `DomainReq*`, `DomainAgentAddr*`, `DomainRoute*` | `agentteam-runtime`, contracts domain chain, `docs/modules/18-*` | Communication Center parsing domain addresses; Agent Registry treating local names as globally unique; zterm adapter resolving business target domains | domain unit, cross-daemon routing red tests |
| `gateway.input` | Input Gateway | `TeamReq*` | `agentteam-gateway` | runtime/UI parsing raw CLI/API/UI payload directly | parser/validator tests |
| `gateway.output` | Output Gateway | `TeamResp*` | `agentteam-gateway` | modules/UI formatting final CLI/UI text directly or exposing private state | projection tests |
| `gateway.ui` | UI Gateway | `UiReq*`, `UiResp*` | `agentteam-gateway`, future web UI | UI/WebUI mutating runtime state directly or calling framework internals | UI contract tests |
| `agent.naming_pool` | Agent Registry | `AgentName*`, `AgentMember*`, `AgentStatus*` | `agentteam-runtime`, contracts | tmux adapter inventing names; Codex SDK as generic status truth | naming/status red tests |
| `team.orchestration` | Team Orchestrator | `TeamCommand*` | `agentteam-runtime` | gateways deciding execution order | orchestration tests |
| `task.engine` | Task Engine | `TaskEvent*`, `TaskState*` | `agentteam-runtime` | comm center mutating queue internals | queue tests |
| `debug.center` | Debug Center | `DebugSnapshot*` | `agentteam-debug` | debug center reading private fields | snapshot tests |
| `persist.event_log` | Persistence | `PersistEvent*` | `agentteam-persist` | modules writing state files directly | replay tests |
| `adapter.zterm_tmux` | zterm/tmux Adapter | `TerminalReq*`, `TerminalResp*` | `agentteam-tmux` | runtime shelling out to tmux directly; adapter owning task/status truth | adapter contract tests |
| `adapter.tui_agent` | TUI Agent Adapter Center | `TuiSignalReq*`, `TuiSignalResp*` | `agentteam-tui-adapter` | stdout-only final status; Codex SDK as universal truth; provider payload in runtime business state | tui adapter red tests |
| `agent.control_center` | Agent Control Center | `AgentCtlReq*`, `AgentCtlResp*` | `agentteam-control`, `agentteam-tmux`, `agentteam-tui-adapter`, `agentteam-runtime`, `docs/modules/19-*` | tmux adapter owning headless session truth; SDK adapter owning task truth; implicit mode fallback | control unit, attach/headless red tests |
| `startup.session` | Startup Session Manager | `StartupReq*`, `StartupOp*` | `agentteam-startup`, `agentteam-control`, `agentteam-tmux`, `agentteam-resource`, `agentteam-config` | manager as persistence truth; direct tmux execution outside adapter; direct state file write | startup/session red tests |
| `tanote.board` | TANote Collaboration Board | `TANoteReq*`, `TANoteEvent*`, `TANoteProjection*` | `agentteam-tanote`, `docs/tanote/TANote.md.example` | Task Engine/Comm/agents directly mutating TANote format or treating notes as task truth | TANote format/order/thread red tests |
| `resource.lifecycle` | Resource Lifecycle Manager | `ResourceReq*`, `ResourceLease*`, `ResourceMetric*`, `ResourceLeak*` | `agentteam-resource`, contracts resource chain | modules creating long-lived resources without leases; broad cleanup; orphan/leak/growth hidden from event log/debug | lifecycle/leak/orphan/growth red tests |
| `cli.agent_skill` | CLI/Skill | `CliCommand*` | `agentteam-cli`, `.agents/skills` | skill depending on hidden wire protocol; agent-facing tmux/session internals; manager missing framework-operation guidance | CLI smoke tests |

## Owner Rule

Each row has one owner module. Shared helpers must be moved into `agentteam-contracts` or explicit block modules, not duplicated in owner crates.

## Function Registry

Every Rust function or method under `crates/` and `xtask/src/` must be listed here before the function-map gate passes.

| symbol | owner | feature_id | allowed paths | required gates |
|---|---|---|---|---|
| `crates::agentteam-cli::src::main::main` | CLI/Skill | `cli.agent_skill` | `crates/agentteam-cli/src/main.rs` | cargo test, function map gate |
| `crates::agentteam-cli::src::main::render_gateway_error` | CLI/Skill | `cli.agent_skill` | `crates/agentteam-cli/src/main.rs` | CLI smoke, function map gate |
| `crates::agentteam-cli::src::main::render_local_error` | CLI/Skill | `cli.agent_skill` | `crates/agentteam-cli/src/main.rs` | CLI smoke, function map gate |
| `crates::agentteam-cli::src::main::render_local_result` | CLI/Skill | `cli.agent_skill` | `crates/agentteam-cli/src/main.rs` | CLI smoke, function map gate |
| `crates::agentteam-gateway::src::error::GatewayError::output` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/error.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::error::GatewayError::parse` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/error.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::error::GatewayError::validation` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/error.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::options::contains_flag` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/options.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::option_value` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_cli_args` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_cli_raw` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_config_check` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_daemon_check` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_debug_snapshot` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::broadcast::parse_msg_broadcast` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/broadcast.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_ready_report` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_domain_resolve` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::broadcast::parse_members_list` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/broadcast.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::options::parse_options` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/options.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::require_json` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::require_value` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::validate_intent` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::model::TeamReq01CliRaw::new` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/model.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::model::TeamReq03ValidatedIntent::command_name` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/model.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::encode_error_projection` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::encode_intent_projection` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::encode_local_result_projection` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::local_error_class` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::local_error_reason` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::local_result_command_name` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::render_gateway_error_json` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::render_intent_json` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::render_local_error_json` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::output::render_local_result_json` | Output Gateway | `gateway.output` | `crates/agentteam-gateway/src/output.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::tests::missing_json_is_validation_error` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_config_check_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_daemon_check_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_debug_snapshot_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_domain_resolve_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_intent_json_marks_parse_only` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_local_result_json_does_not_mark_parse_only` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::strings` | Input Gateway test helper | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::unknown_flag_is_parse_error` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::config::mod::ConfigErr01Parse::new` | Config Center contracts | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::config::mod::ConfigErr02Validation::new` | Config Center contracts | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::config::mod::ConfigReq01TomlPath::new` | Config Center contracts | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::config::mod::ConfigReq01TomlPath::read_as_raw` | Config Center contracts | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::config::mod::ConfigReq02TomlRaw::parse_as_document` | Config Center contracts | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::config::mod::ConfigReq03ParsedToml::validate_user_config` | Config Center contracts | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::config::mod::ConfigReq04ValidatedUserConfig::normalize_runtime` | Config Center contracts | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::config::mod::ConfigResp05RuntimeConfig::snapshot` | Config Center contracts | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::config::mod::config_chain_uses_adjacent_nodes` | Config Center contract test | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::config::mod::config_feature_id_is_stable` | Config Center contract test | `config.center` | `crates/agentteam-contracts/src/config/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::debug::mod::DebugReq01SnapshotIntent::new` | Debug Center contracts | `debug.center` | `crates/agentteam-contracts/src/debug/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::debug::mod::DebugReq01SnapshotIntent::request_module` | Debug Center contracts | `debug.center` | `crates/agentteam-contracts/src/debug/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::debug::mod::DebugReq02ModuleSnapshotRequest::bundle` | Debug Center contracts | `debug.center` | `crates/agentteam-contracts/src/debug/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::debug::mod::debug_bundle_requires_persistence_receipt` | Debug Center contract test | `debug.center` | `crates/agentteam-contracts/src/debug/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::debug::mod::debug_feature_id_is_stable` | Debug Center contract test | `debug.center` | `crates/agentteam-contracts/src/debug/mod.rs` | cargo test, function map gate |
| `crates::agentteam-debug::src::bundle::bundle_id` | Debug Center | `debug.center` | `crates/agentteam-debug/src/bundle.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::bundle::capture_debug_bundle` | Debug Center | `debug.center` | `crates/agentteam-debug/src/bundle.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::bundle::receipt_id_for_sequence` | Debug Center | `debug.center` | `crates/agentteam-debug/src/bundle.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::bundle::resource_input` | Debug Center | `debug.center` | `crates/agentteam-debug/src/bundle.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::bundle::validate_input` | Debug Center | `debug.center` | `crates/agentteam-debug/src/bundle.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::error::DebugError::reason` | Debug Center | `debug.center` | `crates/agentteam-debug/src/error.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::error::persistence_error` | Debug Center | `debug.center` | `crates/agentteam-debug/src/error.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::error::resource_error` | Debug Center | `debug.center` | `crates/agentteam-debug/src/error.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::persist::encode_payload<T: Serialize>` | Debug Center | `debug.center` | `crates/agentteam-debug/src/persist.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::persist::persist_debug_bundle<T: Serialize>` | Debug Center | `debug.center` | `crates/agentteam-debug/src/persist.rs` | debug unit, function map gate |
| `crates::agentteam-debug::src::tests::debug_bundle_includes_resource_snapshot` | Debug Center test | `debug.center` | `crates/agentteam-debug/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-debug::src::tests::debug_bundle_persists_before_projection` | Debug Center test | `debug.center` | `crates/agentteam-debug/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-debug::src::tests::input` | Debug Center test helper | `debug.center` | `crates/agentteam-debug/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-debug::src::tests::invalid_debug_input_fails_validation` | Debug Center test | `debug.center` | `crates/agentteam-debug/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-debug::src::tests::temp_log_path` | Debug Center test helper | `debug.center` | `crates/agentteam-debug/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::domain::mod::DomainAgentAddr03Resolved::plan_route` | Daemon Domain Registry contracts | `domain.registry` | `crates/agentteam-contracts/src/domain/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::domain::mod::DomainReq01RawTarget::new` | Daemon Domain Registry contracts | `domain.registry` | `crates/agentteam-contracts/src/domain/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::domain::mod::DomainReq01RawTarget::validate` | Daemon Domain Registry contracts | `domain.registry` | `crates/agentteam-contracts/src/domain/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::domain::mod::DomainReq02Validated::resolve_agent` | Daemon Domain Registry contracts | `domain.registry` | `crates/agentteam-contracts/src/domain/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::domain::mod::DomainRouteEndpoint::new` | Daemon Domain Registry contracts | `domain.registry` | `crates/agentteam-contracts/src/domain/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::domain::mod::domain_chain_plans_local_route` | Daemon Domain Registry contract test | `domain.registry` | `crates/agentteam-contracts/src/domain/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::domain::mod::domain_feature_id_is_stable` | Daemon Domain Registry contract test | `domain.registry` | `crates/agentteam-contracts/src/domain/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::error::mod::TeamErr01FaultFact::classify` | Error Center contracts | `error.center` | `crates/agentteam-contracts/src/error/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::error::mod::TeamErr01FaultFact::new` | Error Center contracts | `error.center` | `crates/agentteam-contracts/src/error/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::error::mod::TeamErr02Classified::link_evidence` | Error Center contracts | `error.center` | `crates/agentteam-contracts/src/error/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::error::mod::TeamErr02EvidenceLinked::persist_as_event` | Error Center contracts | `error.center` | `crates/agentteam-contracts/src/error/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::error::mod::TeamErr03RuntimeEvent::project` | Error Center contracts | `error.center` | `crates/agentteam-contracts/src/error/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::error::mod::error_chain_keeps_evidence_and_receipt` | Error Center contract test | `error.center` | `crates/agentteam-contracts/src/error/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::error::mod::error_feature_id_is_stable` | Error Center contract test | `error.center` | `crates/agentteam-contracts/src/error/mod.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::classify::classify_fault` | Error Center | `error.center` | `crates/agentteam-error/src/classify.rs` | error unit, function map gate |
| `crates::agentteam-error::src::classify::handle_framework_fault` | Error Center | `error.center` | `crates/agentteam-error/src/classify.rs` | error unit, function map gate |
| `crates::agentteam-error::src::classify::link_error_evidence` | Error Center | `error.center` | `crates/agentteam-error/src/classify.rs` | error unit, function map gate |
| `crates::agentteam-error::src::classify::project_error` | Error Center | `error.center` | `crates/agentteam-error/src/classify.rs` | error unit, function map gate |
| `crates::agentteam-error::src::classify::reject_normal_task_error` | Error Center | `error.center` | `crates/agentteam-error/src/classify.rs` | error unit, function map gate |
| `crates::agentteam-error::src::code::evidence_id_for_code` | Error Center | `error.center` | `crates/agentteam-error/src/code.rs` | error unit, function map gate |
| `crates::agentteam-error::src::code::generate_error_code` | Error Center | `error.center` | `crates/agentteam-error/src/code.rs` | error unit, function map gate |
| `crates::agentteam-error::src::code::severity_label` | Error Center | `error.center` | `crates/agentteam-error/src/code.rs` | error unit, function map gate |
| `crates::agentteam-error::src::code::validate_segment` | Error Center | `error.center` | `crates/agentteam-error/src/code.rs` | error unit, function map gate |
| `crates::agentteam-error::src::code::validate_timestamp` | Error Center | `error.center` | `crates/agentteam-error/src/code.rs` | error unit, function map gate |
| `crates::agentteam-error::src::error::ErrorCenterError::reason` | Error Center | `error.center` | `crates/agentteam-error/src/error.rs` | error unit, function map gate |
| `crates::agentteam-error::src::error::persistence_error` | Error Center | `error.center` | `crates/agentteam-error/src/error.rs` | error unit, function map gate |
| `crates::agentteam-error::src::model::ErrorCodeSeed::new` | Error Center | `error.center` | `crates/agentteam-error/src/model.rs` | error unit, function map gate |
| `crates::agentteam-error::src::persist::encode_payload` | Error Center | `error.center` | `crates/agentteam-error/src/persist.rs` | error unit, function map gate |
| `crates::agentteam-error::src::persist::payload_for_linked_error` | Error Center | `error.center` | `crates/agentteam-error/src/persist.rs` | error unit, function map gate |
| `crates::agentteam-error::src::persist::persist_error_event` | Error Center | `error.center` | `crates/agentteam-error/src/persist.rs` | error unit, function map gate |
| `crates::agentteam-error::src::persist::receipt_id_for_sequence` | Error Center | `error.center` | `crates/agentteam-error/src/persist.rs` | error unit, function map gate |
| `crates::agentteam-error::src::tests::classifies_with_severity_and_code` | Error Center test | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::tests::fault` | Error Center test helper | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::tests::links_independent_evidence_id` | Error Center test | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::tests::malformed_code_seed_fails_validation` | Error Center test | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::tests::normal_agent_task_error_is_rejected` | Error Center test | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::tests::persist_failure_does_not_project_success` | Error Center test | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::tests::persists_before_projection` | Error Center test | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::tests::seed` | Error Center test helper | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-error::src::tests::temp_log_path` | Error Center test helper | `error.center` | `crates/agentteam-error/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::persist::mod::PersistReq01EventDraft::new` | Persistence contracts | `persist.event_log` | `crates/agentteam-contracts/src/persist/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::persist::mod::PersistReq01EventDraft::validate` | Persistence contracts | `persist.event_log` | `crates/agentteam-contracts/src/persist/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::persist::mod::PersistReq02ValidatedEvent::append_receipt` | Persistence contracts | `persist.event_log` | `crates/agentteam-contracts/src/persist/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::persist::mod::PersistReq04Replay::materialize` | Persistence contracts | `persist.event_log` | `crates/agentteam-contracts/src/persist/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::persist::mod::PersistReq04Replay::new` | Persistence contracts | `persist.event_log` | `crates/agentteam-contracts/src/persist/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::persist::mod::persist_append_chain_uses_adjacent_nodes` | Persistence contract test | `persist.event_log` | `crates/agentteam-contracts/src/persist/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::persist::mod::persist_feature_id_is_stable` | Persistence contract test | `persist.event_log` | `crates/agentteam-contracts/src/persist/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::persist::mod::persist_replay_chain_materializes_state` | Persistence contract test | `persist.event_log` | `crates/agentteam-contracts/src/persist/mod.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::append::append_event_log` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/append.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::append::encode_record` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/append.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::append::ensure_parent_dir` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/append.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::append::io_error` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/append.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::append::next_sequence` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/append.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::append::validate_draft` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/append.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::error::PersistenceError::reason` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/error.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::materialize::materialize_event_log` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/materialize.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::materialize::snapshot_id_for_sequence` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/materialize.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::model::event_id_for_sequence` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/model.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::replay::parse_record` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/replay.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::replay::replay_event_log` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/replay.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::replay::validate_record_sequence` | Persistence | `persist.event_log` | `crates/agentteam-persist/src/replay.rs` | persist unit, function map gate |
| `crates::agentteam-persist::src::tests::append_returns_receipt` | Persistence test | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::tests::corrupt_record_fails_explicitly` | Persistence test | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::tests::draft` | Persistence test helper | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::tests::empty_draft_fails_validation` | Persistence test | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::tests::materialized_state_uses_latest_sequence` | Persistence test | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::tests::replay_from_sequence_filters_older_events` | Persistence test | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::tests::replay_returns_events_in_sequence` | Persistence test | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::tests::sequence_mismatch_fails_explicitly` | Persistence test | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-persist::src::tests::temp_log_path` | Persistence test helper | `persist.event_log` | `crates/agentteam-persist/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::pipeline::mod::PipelineNodeName::new` | Contracts Pipeline | `contract.pipeline` | `crates/agentteam-contracts/src/pipeline/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::resource::mod::ResourceLease04Active::release` | Resource Lifecycle contracts | `resource.lifecycle` | `crates/agentteam-contracts/src/resource/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::resource::mod::ResourceMetric03Initial::activate` | Resource Lifecycle contracts | `resource.lifecycle` | `crates/agentteam-contracts/src/resource/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::resource::mod::ResourceReq01AcquireIntent::new` | Resource Lifecycle contracts | `resource.lifecycle` | `crates/agentteam-contracts/src/resource/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::resource::mod::ResourceReq01AcquireIntent::validate_scope` | Resource Lifecycle contracts | `resource.lifecycle` | `crates/agentteam-contracts/src/resource/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::resource::mod::ResourceReq02ValidatedScope::initial_metric` | Resource Lifecycle contracts | `resource.lifecycle` | `crates/agentteam-contracts/src/resource/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::resource::mod::resource_chain_requires_active_lease_before_release` | Resource Lifecycle contract test | `resource.lifecycle` | `crates/agentteam-contracts/src/resource/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::resource::mod::resource_feature_id_is_stable` | Resource Lifecycle contract test | `resource.lifecycle` | `crates/agentteam-contracts/src/resource/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::team::mod::TeamReq01CliRaw::new` | Input Gateway contracts | `gateway.input` | `crates/agentteam-contracts/src/team/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::team::mod::TeamReq03ValidatedIntent::command_name` | Input Gateway contracts | `gateway.input` | `crates/agentteam-contracts/src/team/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::team::mod::team_feature_id_is_gateway_input` | Input Gateway contract test | `gateway.input` | `crates/agentteam-contracts/src/team/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::team::mod::team_request_command_name_is_stable` | Input Gateway contract test | `gateway.input` | `crates/agentteam-contracts/src/team/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::TerminalReq01AdapterCommand::loopback` | zterm/tmux Adapter contracts | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::TerminalReq01AdapterCommand::prepare_transport` | zterm/tmux Adapter contracts | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::TerminalReq02TransportRequest::transport_event` | zterm/tmux Adapter contracts | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::TerminalResp03TransportEvent::observe` | zterm/tmux Adapter contracts | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::terminal_chain_uses_adjacent_nodes` | zterm/tmux Adapter contract test | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::terminal_feature_id_is_stable` | zterm/tmux Adapter contract test | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | cargo test, function map gate |
| `crates::agentteam-resource::src::error::ResourceError::reason` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/error.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::error::persistence_error` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/error.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::persist::encode_payload<T: Serialize>` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/persist.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::persist::persist_resource_event<T: Serialize>` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/persist.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::ResourceRegistry::acquire` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::ResourceRegistry::leases` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::ResourceRegistry::mark_leak` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::ResourceRegistry::new` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::ResourceRegistry::release` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::ResourceRegistry::snapshot` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::receipt_id_for_sequence` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::record_from_active` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::registry::validate_acquire_input` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/registry.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::snapshot::snapshot_registry` | Resource Lifecycle Manager | `resource.lifecycle` | `crates/agentteam-resource/src/snapshot.rs` | resource unit, function map gate |
| `crates::agentteam-resource::src::tests::acquire_registers_lease_and_persists_event` | Resource Lifecycle Manager test | `resource.lifecycle` | `crates/agentteam-resource/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-resource::src::tests::input` | Resource Lifecycle Manager test helper | `resource.lifecycle` | `crates/agentteam-resource/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-resource::src::tests::invalid_acquire_input_fails_validation` | Resource Lifecycle Manager test | `resource.lifecycle` | `crates/agentteam-resource/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-resource::src::tests::leak_projection_is_persisted_and_visible_in_snapshot` | Resource Lifecycle Manager test | `resource.lifecycle` | `crates/agentteam-resource/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-resource::src::tests::release_by_non_owner_fails` | Resource Lifecycle Manager test | `resource.lifecycle` | `crates/agentteam-resource/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-resource::src::tests::release_requires_owner_and_persists_event` | Resource Lifecycle Manager test | `resource.lifecycle` | `crates/agentteam-resource/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-resource::src::tests::temp_log_path` | Resource Lifecycle Manager test helper | `resource.lifecycle` | `crates/agentteam-resource/src/tests.rs` | cargo test, function map gate |
| `crates::agentteamd::src::main::main` | Daemon binary scaffold | `startup.session` | `crates/agentteamd/src/main.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::domain::model::DomainEndpoint::new` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/model.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::model::DomainEndpoint::route_endpoint` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/model.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::registry::DomainRegistry::insert_domain` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/registry.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::registry::DomainRegistry::new` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/registry.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::registry::DomainRegistry::register_remote` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/registry.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::registry::DomainRegistry::resolve_target` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/registry.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::registry::DomainRegistry::snapshot` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/registry.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::registry::registered_domain` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/registry.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::registry::validate_domain_id` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/registry.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::registry::validation_error<T>` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/registry.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::resolve::agent_name_for_contract` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/resolve.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::resolve::non_empty_target<'a>` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/resolve.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::resolve::parse_target_kind` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/resolve.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::resolve::resolve_domain<'a>` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/resolve.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::resolve::resolve_target` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/resolve.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::resolve::split_target_domain` | Daemon Domain Registry | `domain.registry` | `crates/agentteam-runtime/src/domain/resolve.rs` | domain unit, function map gate |
| `crates::agentteam-runtime::src::domain::tests::bare_target_resolves_only_to_local` | Daemon Domain Registry test | `domain.registry` | `crates/agentteam-runtime/src/domain/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::domain::tests::duplicate_domain_alias_fails` | Daemon Domain Registry test | `domain.registry` | `crates/agentteam-runtime/src/domain/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::domain::tests::registry` | Daemon Domain Registry test helper | `domain.registry` | `crates/agentteam-runtime/src/domain/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::domain::tests::resolves_local_agent_target` | Daemon Domain Registry test | `domain.registry` | `crates/agentteam-runtime/src/domain/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::domain::tests::resolves_remote_alias_target` | Daemon Domain Registry test | `domain.registry` | `crates/agentteam-runtime/src/domain/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::domain::tests::resolves_role_team_and_all_targets` | Daemon Domain Registry test | `domain.registry` | `crates/agentteam-runtime/src/domain/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::domain::tests::snapshot_does_not_expose_tokens` | Daemon Domain Registry test | `domain.registry` | `crates/agentteam-runtime/src/domain/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::domain::tests::unknown_remote_domain_does_not_fallback_to_local` | Daemon Domain Registry test | `domain.registry` | `crates/agentteam-runtime/src/domain/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::control::control_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/control.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::build_domain_registry` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::config_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::config_result` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::daemon_check_result` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::debug_bundle_result` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::debug_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::domain_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::domain_snapshot_result` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::event_log_path` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_config_check` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_daemon_check` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_debug_snapshot` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_domain_resolve` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_local_intent` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_done` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_list` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_send` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_status` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_tmux_loopback` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::parse_session_count` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::register_remote_domain` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::task_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::tmux_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::resolved_domain_result` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::route_kind_label` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::target_kind_parts` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_tests::example_config_path` | Team Orchestrator test helper | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_config_check_executes_config_center` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_daemon_check_reports_routeability_without_starting_processes` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_debug_snapshot_persists_event_log` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_domain_resolve_executes_domain_registry` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_task_commands_persist_and_replay_state` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_tmux_loopback_rejects_invalid_session_count` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::temp_runtime_home` | Team Orchestrator test helper | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::error::ConfigCenterError::path` | Config Center | `config.center` | `crates/agentteam-config/src/error.rs` | config unit, function map gate |
| `crates::agentteam-config::src::error::ConfigCenterError::reason` | Config Center | `config.center` | `crates/agentteam-config/src/error.rs` | config unit, function map gate |
| `crates::agentteam-config::src::lib::check_config_path` | Config Center | `config.center` | `crates/agentteam-config/src/lib.rs` | config unit, function map gate |
| `crates::agentteam-config::src::lib::validate_config_path` | Config Center | `config.center` | `crates/agentteam-config/src/lib.rs` | config unit, function map gate |
| `crates::agentteam-config::src::load::load_config_file` | Config Center | `config.center` | `crates/agentteam-config/src/load.rs` | config unit, function map gate |
| `crates::agentteam-config::src::load::load_default_config_file` | Config Center | `config.center` | `crates/agentteam-config/src/load.rs` | config unit, function map gate |
| `crates::agentteam-config::src::load::missing_file_is_explicit_load_error` | Config Center test | `config.center` | `crates/agentteam-config/src/load.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::normalize::normalize_config` | Config Center | `config.center` | `crates/agentteam-config/src/normalize.rs` | config unit, function map gate |
| `crates::agentteam-config::src::parse::malformed_toml_is_explicit_parse_error` | Config Center test | `config.center` | `crates/agentteam-config/src/parse.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::parse::parse_config_toml` | Config Center | `config.center` | `crates/agentteam-config/src/parse.rs` | config unit, function map gate |
| `crates::agentteam-config::src::snapshot::snapshot_config` | Config Center | `config.center` | `crates/agentteam-config/src/snapshot.rs` | config unit, function map gate |
| `crates::agentteam-config::src::tests::agent_count_mismatch_fails` | Config Center test | `config.center` | `crates/agentteam-config/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::tests::assert_validation_reason` | Config Center test helper | `config.center` | `crates/agentteam-config/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::tests::duplicate_domain_id_fails` | Config Center test | `config.center` | `crates/agentteam-config/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::tests::duplicate_member_name_fails` | Config Center test | `config.center` | `crates/agentteam-config/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::tests::example_config_path` | Config Center test helper | `config.center` | `crates/agentteam-config/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::tests::example_config_normalizes` | Config Center test | `config.center` | `crates/agentteam-config/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::tests::snapshot_redacts_token_state` | Config Center test | `config.center` | `crates/agentteam-config/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::tests::validate_config_path_returns_verified_user_config` | Config Center test | `config.center` | `crates/agentteam-config/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-config::src::validate::insert_unique` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::reject_runtime_state_keys` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::remote_domains` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::require_non_empty` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::require_tmux_prefix` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::require_valid_project_slug` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::validate_config` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::validate_domains` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::validate_remote_domain` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::validate_team_members` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::validate_teams` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-config::src::validate::validation_error<T>` | Config Center | `config.center` | `crates/agentteam-config/src/validate.rs` | config unit, function map gate |
| `crates::agentteam-contracts::src::event_hash::mod::event_payload_hash` | Contracts Event Hash | `contract.pipeline` | `crates/agentteam-contracts/src/event_hash/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::event_hash::mod::event_payload_hash_is_stable` | Contracts Event Hash test | `contract.pipeline` | `crates/agentteam-contracts/src/event_hash/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::task::mod::TaskReq01Create::new` | Task Engine contracts | `task.engine` | `crates/agentteam-contracts/src/task/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::task::mod::TaskReq01Create::queue` | Task Engine contracts | `task.engine` | `crates/agentteam-contracts/src/task/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::task::mod::TaskReq02Queued::dispatch_ready` | Task Engine contracts | `task.engine` | `crates/agentteam-contracts/src/task/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::task::mod::TaskReq03DispatchReady::state_changed` | Task Engine contracts | `task.engine` | `crates/agentteam-contracts/src/task/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::task::mod::task_chain_uses_adjacent_nodes` | Task Engine contract test | `task.engine` | `crates/agentteam-contracts/src/task/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::task::mod::task_feature_id_is_stable` | Task Engine contract test | `task.engine` | `crates/agentteam-contracts/src/task/mod.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::input::parse_task_done` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_task_error` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_task_list` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_task_claim` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_msg_send` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_task_send` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_task_status` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_tmux_loopback` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::tests::parses_task_send_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_ready_report_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_task_claim_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_msg_send_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_msg_broadcast_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_task_status_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_tmux_loopback_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_task_result_json_uses_task_command_name` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_message_result_json_uses_msg_command_name` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_broadcast_result_json_uses_msg_command_name` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_ready_result_json_uses_ready_command_name` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq01RouteIntent::new` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq01RouteIntent::resolve_target` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq02ResolvedTarget::delivery_envelope` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq03DeliveryEnvelope::accept` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq05ReadyReport::new` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq05ReadyReport::resolve_agent` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq06ResolvedAgent::delivery_envelope` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq07ReadyEnvelope::accept` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq11BroadcastIntent::new` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq11BroadcastIntent::resolve_team_members` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq12ResolvedTeamMembers::delivery_envelope` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq13BroadcastEnvelope::accept` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq21TaskBoardQuery::new` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq21TaskBoardQuery::resolve_team` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq22AuthorizedQuery::delivery_envelope` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq23TaskBoardQueryEnvelope::accept` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq31TaskClaim::new` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq31TaskClaim::resolve_claim` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq32AuthorizedClaim::delivery_envelope` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::mod::CommReq33TaskClaimEnvelope::accept` | Communication Center contracts | `comm.center` | `crates/agentteam-contracts/src/comm/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::comm::tests::comm_broadcast_chain_uses_adjacent_nodes` | Communication Center contract test | `comm.center` | `crates/agentteam-contracts/src/comm/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::comm::tests::comm_feature_id_is_stable` | Communication Center contract test | `comm.center` | `crates/agentteam-contracts/src/comm/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::comm::tests::comm_message_chain_uses_adjacent_nodes` | Communication Center contract test | `comm.center` | `crates/agentteam-contracts/src/comm/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::comm::tests::comm_ready_chain_uses_adjacent_nodes` | Communication Center contract test | `comm.center` | `crates/agentteam-contracts/src/comm/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::comm::tests::comm_task_board_chain_uses_adjacent_nodes` | Communication Center contract test | `comm.center` | `crates/agentteam-contracts/src/comm/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::comm::tests::comm_task_claim_chain_uses_adjacent_nodes` | Communication Center contract test | `comm.center` | `crates/agentteam-contracts/src/comm/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::error::CommCenterError::reason` | Communication Center | `comm.center` | `crates/agentteam-comm/src/error.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::error::persistence_error` | Communication Center | `comm.center` | `crates/agentteam-comm/src/error.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommRouteRequest::new` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommRouteTarget::delivery_envelope` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommReadyReportEnvelope::accept` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommReadyReportRequest::new` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommReadyReportRequest::resolve_agent` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommReadyReportTarget::delivery_envelope` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTaskBoardQueryEnvelope::accept` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTaskBoardQueryRequest::new` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTaskBoardQueryRequest::resolve_team` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTaskBoardQueryTarget::delivery_envelope` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTaskClaimEnvelope::accept` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTaskClaimRequest::new` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTaskClaimRequest::resolve_claim` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTaskClaimTarget::delivery_envelope` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::model::CommTeamBroadcastRequest::new` | Communication Center | `comm.center` | `crates/agentteam-comm/src/model.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::persist::persist_delivery_event<T: Serialize>` | Communication Center + Persistence | `comm.center` | `crates/agentteam-comm/src/persist.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::new` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::route_broadcast` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::route_message` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::route_ready_report` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::route_task_board_query` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::route_task_claim` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::send_message` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::send_broadcast` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::CommCenter::send_ready_report` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::delivery_id_for` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::message_send_result` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::broadcast_send_result` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::ready_report_send_result` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::route_broadcast` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::route_message` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::route_ready_report` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::route_task_board_query` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::route_task_claim` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::send_message` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::send_broadcast` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::send_ready_report` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::validate_broadcast_members` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::validate_message_target` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::validate_ready_report` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::validate_task_board_query` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::route::validate_task_claim` | Communication Center | `comm.center` | `crates/agentteam-comm/src/route.rs` | routing unit, function map gate |
| `crates::agentteam-comm::src::tests::route_broadcast_accepts_exact_member_list` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::persist_delivery_event_reports_persistence_failure` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::persist_delivery_event_writes_replayable_jsonl` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::temp_log_path` | Communication Center test helper | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::send_message_persists_delivery_and_returns_receipt` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::send_broadcast_persists_delivery_and_returns_receipt` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::send_ready_report_persists_delivery_and_returns_receipt` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::route_ready_report_accepts_agent_name` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::route_message_accepts_non_empty_target` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::route_message_rejects_empty_target` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::route_task_board_query_preserves_query` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-comm::src::tests::route_task_claim_accepts_worker_scope` | Communication Center test | `comm.center` | `crates/agentteam-comm/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_done` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_claim` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_list` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_msg_send` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_msg_broadcast` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_ready_report` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_send` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::execute_task_status` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::comm_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::parse_task_target_kind` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local::task_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/local.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::config_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::daemon_check_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::debug_bundle_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::domain_snapshot_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::message_send_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::broadcast_send_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::ready_report_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::resolved_domain_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::tmux_loopback_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_msg_broadcast_persists_delivery_event` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_projection::route_kind_label` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::target_kind_parts` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::task_board_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::task_changed_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_msg_send_persists_delivery_event` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_ready_report_persists_delivery_event` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_task_commands_persist_and_replay_state` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::board` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::create_task` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::claim_task` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::mark_done` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::mark_error` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::mark_running` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::new` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::status` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::TaskEngine::transition_task` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::next_task_id` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::persist_state_change` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::claim_scope_label` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::claim_scope_matches` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::require_non_empty` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::validate_claim_input` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::validate_create_input` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::validate_transition` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::engine::validate_transition_input` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/engine.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::error::TaskEngineError::reason` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/error.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::error::persistence_error` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/error.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::materialize::apply_task_event` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/materialize.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::materialize::apply_transition` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/materialize.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::materialize::decode_payload` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/materialize.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::materialize::insert_created` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/materialize.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::materialize::materialize_task_board` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/materialize.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::model::TaskStatus::label` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/model.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::model::TaskTargetKind::label` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/model.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::model::default_task_priority` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/model.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::persist::encode_payload<T: Serialize>` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/persist.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::persist::persist_task_event<T: Serialize>` | Task Engine | `task.engine` | `crates/agentteam-runtime/src/task/persist.rs` | task unit, function map gate |
| `crates::agentteam-runtime::src::task::tests::create_input` | Task Engine test helper | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::tests::create_task_persists_and_replays_board` | Task Engine test | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::tests::claim_prefers_assigned_task_over_role_match` | Task Engine test | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::tests::claim_prefers_blocked_task_inside_same_class` | Task Engine test | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::tests::task_error_is_normal_task_state` | Task Engine test | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::tests::temp_log_path` | Task Engine test helper | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::tests::terminal_state_requires_explicit_done_or_error` | Task Engine test | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::tests::transition` | Task Engine test helper | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::task::tests::unknown_task_status_fails` | Task Engine test | `task.engine` | `crates/agentteam-runtime/src/task/tests.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::TerminalReq01AdapterCommand::loopback` | zterm/tmux Adapter contracts | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::TerminalReq01AdapterCommand::prepare_transport` | zterm/tmux Adapter contracts | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::TerminalReq02TransportRequest::transport_event` | zterm/tmux Adapter contracts | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::TerminalResp03TransportEvent::observe` | zterm/tmux Adapter contracts | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::terminal_chain_uses_adjacent_nodes` | zterm/tmux Adapter contract test | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::terminal::mod::terminal_feature_id_is_stable` | zterm/tmux Adapter contract test | `adapter.zterm_tmux` | `crates/agentteam-contracts/src/terminal/mod.rs` | cargo test, function map gate |
| `crates::agentteam-tmux::src::error::TmuxAdapterError::reason` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/error.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::build_sessions` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::capture_session` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::cleanup_sessions` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::combine_step_and_cleanup` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::launch_session` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::run_loopback_steps` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::run_tmux_loopback` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::runtime_scope` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::sanitized_runtime_scope` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::send_input` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::session_prefix` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::session_shell_command` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::validate_loopback_input` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::loopback::wait_for_marker` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/loopback.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::model::TmuxLoopbackInput::new` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/model.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::model::TmuxLoopbackReport::all_observed` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/model.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::tests::managed_sessions_use_ta_prefix_and_logical_ids` | zterm/tmux Adapter test | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-tmux::src::tests::rejects_zero_session_count` | zterm/tmux Adapter test | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-tmux::src::tests::runtime_scope_requires_directory_name` | zterm/tmux Adapter test | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-tmux::src::tests::scope_sanitizer_keeps_ta_safe_names` | zterm/tmux Adapter test | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/tests.rs` | cargo test, function map gate |
| `xtask::src::function_map::collect_rust_function_symbols` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::collect_rust_function_symbols_in` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::collect_symbols_from_content` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::module_symbol` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::parse_function_name` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::parse_impl_type` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::read` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::require_contains` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::run` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::verify_feature_ids` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::verify_module_docs` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::function_map::verify_rust_function_registry` | Architecture Gate | `architecture.gate` | `xtask/src/function_map.rs` | function map gate |
| `xtask::src::main::collect_oversized_rust_files` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | code-size gate |
| `xtask::src::main::main` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | xtask command smoke |
| `xtask::src::main::read` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | xtask gates |
| `xtask::src::main::red_tests` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | red-tests gate |
| `xtask::src::main::require_contains` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | xtask gates |
| `xtask::src::main::require_file` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | required-file gate |
| `xtask::src::main::run_command` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | canonical verify gate |
| `xtask::src::main::verify_all` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | canonical verify gate |
| `xtask::src::main::verify_code_size` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | code-size gate |
| `xtask::src::main::verify_function_map` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | function map gate |
| `xtask::src::main::verify_one_skill_frontmatter` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | skill-frontmatter gate |
| `xtask::src::main::verify_required_files` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | required-file gate |
| `xtask::src::main::verify_resource_lifecycle` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | resource lifecycle gate |
| `xtask::src::main::verify_skill_frontmatter` | Architecture Gate | `architecture.gate` | `xtask/src/main.rs` | skill-frontmatter gate |
| `xtask::src::red_tests::allowed_broad_kill_reference` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::contains_forbidden_context` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::extract_declared_feature_id` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::ignored_path` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::is_text_candidate` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::no_violations` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::normalize_repo_path` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::read` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::require_contains` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::require_plan_entries` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::run` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_agent_facing_internal_leaks` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_broad_kill_patterns` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_contract_feature_ids` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_configured_agent_name_concepts` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_domain_owner_boundaries` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_non_adjacent_pipeline_conversions` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_rust_files` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_state_file_write_owner` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_text_files` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_toml_parsing_owner` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |

| `crates::agentteam-gateway::src::input::parse_start` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::tests::parses_start_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_local_start_result_uses_start_command_name` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::startup::execute_startup` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/startup.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::startup::execute_startup_worker` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/startup.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::startup::startup_error` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/startup.rs` | runtime unit, function map gate |
| `crates::agentteam-startup::src::config::default_config_path_missing_home` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/config.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::config::load_validated_config` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/config.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::env::build_agent_env` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/env.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::error::StartupError::reason` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/error.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::error::config_error` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/error.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::error::control_error` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/error.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::error::resource_error` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/error.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::error::tmux_error` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/error.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::paths::build_session_name` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/paths.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::paths::expand_default_config_path` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/paths.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::paths::expand_home_path` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/paths.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::paths::resolve_cwd` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/paths.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::paths::runtime_event_log_path` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/paths.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::paths::session_dir` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/paths.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::prompt::build_root_manager_bootstrap_prompt` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/prompt.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::prompt::build_worker_bootstrap_prompt` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/prompt.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::select::select_root_manager` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/select.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::select::select_team<'a>` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/select.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::select::select_worker<'a>` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/select.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::select::worker_names` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/select.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::lib::start_bootstrap` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/lib.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::lib::start_worker` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/lib.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::session::build_resume_args` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/session.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::session::control_session_input` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/session.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::session::ensure_agent_session` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/session.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::session::launch_new_agent_session` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/session.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::session::release_after_launch_failure<T>` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/session.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::session::seed_codex_agent_session` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/session.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::session::stop_seed_bridge` | Startup Session Manager | `startup.session` | `crates/agentteam-startup/src/session.rs` | startup unit, function map gate |
| `crates::agentteam-startup::src::tests::agent_env_contains_identity_and_scope` | Startup Session Manager test | `startup.session` | `crates/agentteam-startup/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-startup::src::tests::member` | Startup Session Manager test helper | `startup.session` | `crates/agentteam-startup/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-startup::src::tests::normalized` | Startup Session Manager test helper | `startup.session` | `crates/agentteam-startup/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-startup::src::tests::root_manager_prompt_teaches_identity_skill_and_worker_start` | Startup Session Manager test | `startup.session` | `crates/agentteam-startup/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-startup::src::tests::session_name_uses_domain_project_and_agent` | Startup Session Manager test | `startup.session` | `crates/agentteam-startup/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-startup::src::tests::team` | Startup Session Manager test helper | `startup.session` | `crates/agentteam-startup/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-startup::src::tests::worker_names_includes_only_worker_roles` | Startup Session Manager test | `startup.session` | `crates/agentteam-startup/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-startup::src::tests::worker_prompt_teaches_role_and_ready_loop` | Startup Session Manager test | `startup.session` | `crates/agentteam-startup/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-tmux::src::launch::build_shell_command` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/launch.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::launch::launch_managed_session` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/launch.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::launch::quote_shell_word` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/launch.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::launch::validate_launch_input` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/launch.rs` | adapter unit, function map gate |
| `crates::agentteam-contracts::src::control::mod::AgentCtlReq01ModeIntent::new` | Agent Control Center contracts | `agent.control_center` | `crates/agentteam-contracts/src/control/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::control::mod::AgentCtlReq01ModeIntent::resolve_mode` | Agent Control Center contracts | `agent.control_center` | `crates/agentteam-contracts/src/control/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::control::mod::AgentCtlReq02ResolvedMode::bind_session` | Agent Control Center contracts | `agent.control_center` | `crates/agentteam-contracts/src/control/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::control::mod::AgentCtlReq03SessionBinding::apply_action` | Agent Control Center contracts | `agent.control_center` | `crates/agentteam-contracts/src/control/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::control::mod::AgentCtlReq04ControlAction::project` | Agent Control Center contracts | `agent.control_center` | `crates/agentteam-contracts/src/control/mod.rs` | contract unit, function map gate |
| `crates::agentteam-contracts::src::control::mod::control_chain_keeps_mode_and_receipt` | Agent Control Center contract test | `agent.control_center` | `crates/agentteam-contracts/src/control/mod.rs` | cargo test, function map gate |
| `crates::agentteam-contracts::src::control::mod::control_feature_id_is_stable` | Agent Control Center contract test | `agent.control_center` | `crates/agentteam-contracts/src/control/mod.rs` | cargo test, function map gate |
| `crates::agentteam-control::src::error::ControlError::reason` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/error.rs` | control unit, function map gate |
| `crates::agentteam-control::src::error::tmux_error` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/error.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::bridge_state_is_running` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::ensure_bridge_running` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::interrupt_turn` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::run_bridge` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::run_turn` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::seed_agent_session` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::send_request` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::session_status` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::spawn_bridge` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::start_session` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::stop_session` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_process::wait_until_running` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_process.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_protocol::HeadlessBridgePaths::read_state` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_protocol.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_protocol::HeadlessBridgePaths::request` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_protocol.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_protocol::HeadlessBridgePaths::resolve` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_protocol.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_protocol::headless_session_dir` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_protocol.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_protocol::project_slug_from_cwd` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_protocol.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_protocol::require_existing_path` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_protocol.rs` | control unit, function map gate |
| `crates::agentteam-control::src::headless_protocol::sanitize_session_name` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/headless_protocol.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::default` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::new` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::attach_tui` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::headless` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::seed_agent_session` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::headless_interrupt` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::headless_run` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::headless_status` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::headless_stop` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::help` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::observe_output` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::pause` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::project_headless_response` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::retry_dispatch` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::run_session_control<F>` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::run_tmux_control<F>` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::send_input` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::next_receipt_id` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::snapshot` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::status` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::stop` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::AgentControlCenter::wait` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::control_action_label` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::help_text` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::lib::status_from_capture` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/lib.rs` | control unit, function map gate |
| `crates::agentteam-control::src::model::ControlRetryInput::new` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/model.rs` | control unit, function map gate |
| `crates::agentteam-control::src::model::ControlAgentSessionBinding::new` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/model.rs` | control unit, function map gate |
| `crates::agentteam-control::src::model::ControlSendInput::new` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/model.rs` | control unit, function map gate |
| `crates::agentteam-control::src::model::ControlSessionInput::new` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/model.rs` | control unit, function map gate |
| `crates::agentteam-control::src::model::ControlSessionInput::with_scope` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/model.rs` | control unit, function map gate |
| `crates::agentteam-control::src::model::ControlSnapshot::from_projection` | Agent Control Center | `agent.control_center` | `crates/agentteam-control/src/model.rs` | control unit, function map gate |
| `crates::agentteam-control::src::tests::attach_help_returns_tmux_contract` | Agent Control Center test | `agent.control_center` | `crates/agentteam-control/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-control::src::tests::agent_session_binding_requires_thread_id` | Agent Control Center test | `agent.control_center` | `crates/agentteam-control/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-control::src::tests::control_chain_projects_receipt` | Agent Control Center test | `agent.control_center` | `crates/agentteam-control/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-control::src::tests::headless_bridge_response_parses_sdk_payload` | Agent Control Center test | `agent.control_center` | `crates/agentteam-control/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-control::src::tests::headless_run_requires_input` | Agent Control Center test | `agent.control_center` | `crates/agentteam-control/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-control::src::tests::retry_input_requires_fact_ids` | Agent Control Center test | `agent.control_center` | `crates/agentteam-control/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-control::src::tests::send_input_requires_text` | Agent Control Center test | `agent.control_center` | `crates/agentteam-control/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-control::src::tests::stopped_headless_bridge_projects_offline` | Agent Control Center test | `agent.control_center` | `crates/agentteam-control/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::input::parse_control` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::input::parse_start_worker` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/input.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::validate::require_json` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/validate.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::validate::require_value` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/validate.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::validate::validate_intent` | Input Gateway | `gateway.input` | `crates/agentteam-gateway/src/validate.rs` | gateway unit, function map gate |
| `crates::agentteam-gateway::src::tests::parses_control_attach_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_control_headless_run_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::parses_control_retry_intent` | Input Gateway test | `gateway.input` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_headless_run_result_json_uses_dynamic_control_command_name` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-gateway::src::tests::render_control_result_json_uses_dynamic_control_command_name` | Output Gateway test | `gateway.output` | `crates/agentteam-gateway/src/tests.rs` | cargo test, function map gate |
| `crates::agentteam-runtime::src::control::execute_control` | Team Orchestrator | `team.orchestration` | `crates/agentteam-runtime/src/control.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_projection::control_result` | Team Orchestrator projection | `team.orchestration` | `crates/agentteam-runtime/src/local_projection.rs` | runtime unit, function map gate |
| `crates::agentteam-runtime::src::local_tests::local_control_headless_run_requires_input` | Team Orchestrator test | `team.orchestration` | `crates/agentteam-runtime/src/local_tests.rs` | cargo test, function map gate |
| `crates::agentteam-tmux::src::control::capture_session` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/control.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::control::interrupt_session` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/control.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::control::send_input` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/control.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::control::session_exists` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/control.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::control::stop_session` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/control.rs` | adapter unit, function map gate |
| `crates::agentteam-tmux::src::control::validate_session_name` | zterm/tmux Adapter | `adapter.zterm_tmux` | `crates/agentteam-tmux/src/control.rs` | adapter unit, function map gate |

## Discussion Items

- Confirm whether `agentteam-runtime` owns both team orchestration and task engine, or whether task engine becomes its own crate.
- Confirm whether UI Gateway includes terminal render surface in v1 or only returns render attachment metadata.
