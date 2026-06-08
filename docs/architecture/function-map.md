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
| `startup.session` | Startup Session Manager | `StartupReq*`, `StartupOp*` | `agentteam-startup` | Kevin as persistence truth; direct tmux execution; direct state file write | startup/session red tests |
| `tanote.board` | TANote Collaboration Board | `TANoteReq*`, `TANoteEvent*`, `TANoteProjection*` | `agentteam-tanote`, `docs/tanote/TANote.md.example` | Task Engine/Comm/agents directly mutating TANote format or treating notes as task truth | TANote format/order/thread red tests |
| `resource.lifecycle` | Resource Lifecycle Manager | `ResourceReq*`, `ResourceLease*`, `ResourceMetric*`, `ResourceLeak*` | `agentteam-resource`, contracts resource chain | modules creating long-lived resources without leases; broad cleanup; orphan/leak/growth hidden from event log/debug | lifecycle/leak/orphan/growth red tests |
| `cli.agent_skill` | CLI/Skill | `CliCommand*` | `agentteam-cli`, `.agents/skills` | skill depending on hidden wire protocol; agent-facing tmux/session internals; Kevin missing framework-operation guidance | CLI smoke tests |

## Owner Rule

Each row has one owner module. Shared helpers must be moved into `agentteam-contracts` or explicit block modules, not duplicated in owner crates.

## Function Registry

Every Rust function or method under `crates/` and `xtask/src/` must be listed here before the function-map gate passes.

| symbol | owner | feature_id | allowed paths | required gates |
|---|---|---|---|---|
| `crates::agentteam-cli::src::main::main` | CLI/Skill | `cli.agent_skill` | `crates/agentteam-cli/src/main.rs` | cargo test, function map gate |
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
| `crates::agentteam-error::src::code::payload_hash` | Error Center | `error.center` | `crates/agentteam-error/src/code.rs` | error unit, function map gate |
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
| `crates::agentteam-config::src::error::ConfigCenterError::path` | Config Center | `config.center` | `crates/agentteam-config/src/error.rs` | config unit, function map gate |
| `crates::agentteam-config::src::error::ConfigCenterError::reason` | Config Center | `config.center` | `crates/agentteam-config/src/error.rs` | config unit, function map gate |
| `crates::agentteam-config::src::lib::check_config_path` | Config Center | `config.center` | `crates/agentteam-config/src/lib.rs` | config unit, function map gate |
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
| `xtask::src::red_tests::scan_domain_owner_boundaries` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_non_adjacent_pipeline_conversions` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_rust_files` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_state_file_write_owner` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_text_files` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |
| `xtask::src::red_tests::scan_toml_parsing_owner` | Architecture Gate | `architecture.gate` | `xtask/src/red_tests.rs` | red-tests gate |

## Discussion Items

- Confirm whether `agentteam-runtime` owns both team orchestration and task engine, or whether task engine becomes its own crate.
- Confirm whether UI Gateway includes terminal render surface in v1 or only returns render attachment metadata.
