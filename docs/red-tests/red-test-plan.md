# Red Test Plan

Red tests prove forbidden architecture behavior stays forbidden.

## Required Red Test Families

1. Naming and boundary tests.
2. No fallback tests.
3. No duplicate owner tests.
4. No hidden transport bypass tests.
5. No error swallowing tests.
6. No debug/private-state leakage tests.
7. No untracked required file tests.

## Initial Red Tests

| red_test_id | Forbidden behavior | Expected failure |
|---|---|---|
| `red.config.parse_outside_config_center` | Any crate parses `config.toml` outside Config Center | architecture gate fails |
| `red.config.runtime_state_in_user_config` | Runtime/task/message/debug state appears in `~/.agentteam/config.toml` | config validation fails |
| `red.config.agent_count_mismatch` | Team `agent_count` differs from configured member count | config validation fails |
| `red.error.success_wrapped_error` | Error projected as success | error red test fails |
| `red.error.not_persisted` | Any classified error lacks event-log append | error red test fails |
| `red.error.missing_evidence_id` | Any classified error lacks independent evidence id | error red test fails |
| `red.error.missing_severity` | Any classified error lacks `fatal/error/warn/info` severity | error red test fails |
| `red.error.malformed_code` | Error code does not match `<module>.<class>.<specific>.<time>.<seq>` | error red test fails |
| `red.error.agent_fault_as_task_error` | Agent/framework fault is recorded only as normal task failure | error red test fails |
| `red.tmux.non_ta_group_operation` | Group operation touches non-TA session | adapter/naming test fails |
| `red.runtime.direct_tmux_call` | Runtime shells out to tmux | architecture gate fails |
| `red.debug.secret_leak` | zterm token in debug snapshot | debug red test fails |
| `red.debug.not_persisted` | Debug bundle/evidence is rendered without persisted bundle/evidence receipt | debug/persistence red test fails |
| `red.debug.print_only` | Debug capture supports print-only or no-save mode in v1 | debug red test fails |
| `red.persist.direct_state_write` | Non-persistence crate writes state file | architecture gate fails |
| `red.persist.concurrent_append_sequence` | Concurrent append assigns duplicate or non-monotonic event sequence numbers | persistence/concurrency red test fails |
| `red.comm.payload_crop` | Message/task text cropped or semantically rewritten | comm contract test fails |
| `red.comm.unauthorized_sender` | Agent performs manager/worker action without capability | comm/registry red test fails |
| `red.comm.claim_without_task_engine` | Communication Center decides task claim result itself | architecture gate fails |
| `red.domain.comm_parses_domain_directly` | Communication Center parses daemon domain address instead of consuming DomainRoute | architecture gate fails |
| `red.domain.global_unique_local_name` | Agent Registry treats local agent names as globally unique across daemons | architecture gate fails |
| `red.domain.remote_fallback_to_local` | Remote domain lookup failure is delivered to local daemon instead | domain/comm red test fails |
| `red.domain.token_leak` | Remote daemon auth token appears in help, output, or debug snapshot | config/debug red test fails |
| `red.comm.delivery_not_persisted` | Delivery/ready/claim envelope lacks persistence request | comm red test fails |
| `red.comm.ready_delivery_not_persisted` | Ready report lacks persistence request | comm red test fails |
| `red.comm.broadcast_delivery_not_persisted` | Broadcast delivery lacks persistence request | comm red test fails |
| `red.comm.partial_broadcast_success` | Broadcast skips failed recipients but reports success | comm red test fails |
| `red.registry.multiple_super_managers_v1` | Config/runtime accepts more than one super manager | registry/config red test fails |
| `red.registry.manager_missing_or_duplicate` | v1 manager is missing or duplicated | registry/config red test fails |
| `red.registry.sample_agent_name_as_code_concept` | Sample configured agent name becomes a Rust declaration, feature id, function id, or red-test id | architecture gate fails |
| `red.registry.worker_pool_order` | Worker 1-20 name allocation differs from fixed pool | registry red test fails |
| `red.registry.worker_overflow_name` | Worker 21+ does not use `<project_slug>_worker_<seq>` | registry red test fails |
| `red.registry.ready_without_ta_session` | Agent marked ready without managed TA session existence | registry/adapter red test fails |
| `red.registry.sdk_only_status` | Agent status depends only on Codex SDK | registry red test fails |
| `red.registry.session_dir_outside_home` | Session metadata path is outside `~/.agentteam/sessions/<project_slug>/` | registry/persistence red test fails |
| `red.task.claim_outside_scope` | Worker claims task that is neither assigned nor role-matching | task red test fails |
| `red.task.claim_order_wrong` | Role-matching/non-blocked task outranks assigned/blocked candidate | task red test fails |
| `red.adapter.stdout_as_final_state` | Adapter classifies stdout text as final task/framework state | adapter red test fails |
| `red.adapter.resolves_domain_target` | zterm/tmux Adapter parses `agent@domain` business target directly | adapter/domain architecture gate fails |
| `red.tui_adapter.codex_required` | Generic/non-Codex agent requires Codex SDK for status | tui adapter red test fails |
| `red.tui_adapter.provider_payload_leak` | Provider-specific payload enters runtime business status | tui adapter red test fails |
| `red.tui_adapter.unknown_as_success` | Unknown provider signal becomes idle/success | tui adapter red test fails |
| `red.agent.mode_fallback` | Agent Control Center silently falls back between attach_tui and headless | control red test fails |
| `red.agent.attach_sdk_status_downgrade` | SDK-seeded attach_tui status loses SDK binding/status and silently downgrades to stdout-only state | architecture gate fails |
| `red.agent.attach_without_tmux_binding` | attach_tui mode is selected without tmux binding | control red test fails |
| `red.agent.headless_without_sdk_binding` | headless mode is selected without SDK binding | control red test fails |
| `red.agent.pause_not_propagated` | pause request does not reach active control session | control red test fails |
| `red.agent.stop_not_propagated` | stop request does not reach active control session | control red test fails |
| `red.agent.retry_without_error_fact` | retry dispatch happens without a persisted error fact | control/error red test fails |
| `red.agent.session_state_leak` | control-plane snapshot exposes private session internals to agent-facing output | control/debug red test fails |
| `red.startup.manager_persistence_truth` | configured root manager writes/owns durable state directly | startup/persistence red test fails |
| `red.startup.direct_tmux` | Startup Manager calls tmux directly instead of adapter | startup/adapter red test fails |
| `red.startup.no_reply_forced_error` | A live manager session with no immediate semantic reply is forced to `error` instead of staying `busy`/pending | startup/status red test fails |
| `red.startup.session_path_outside_home` | Session descriptor path outside `~/.agentteam/sessions/<project_slug>/` | startup red test fails |
| `red.startup.worker_identity_missing` | Spawned worker lacks name/role/team identity injection | startup red test fails |
| `red.startup.raw_input_bypass` | Agent input op bypasses typed operation envelope | startup/input red test fails |
| `red.tanote.missing_route_fields` | TANote block lacks `from`, `to`, or `action` | TANote format red test fails |
| `red.tanote.duplicate_or_unordered` | TANote block duplicates `note_id` or uses non-monotonic `seq` | TANote order red test fails |
| `red.tanote.manual_edit_as_truth` | Manual direct edit of `TANote.md` is accepted as durable truth | TANote/persistence red test fails |
| `red.tanote.task_state_mutation` | TANote entry marks task done/error without Task Engine command | task/TANote red test fails |
| `red.tanote.projection_without_event` | `TANote.md` projection appears without prior persisted note event | persistence/TANote red test fails |
| `red.tanote.raw_tmux_payload` | Agent communication through tmux lacks AgentTeam envelope | comm/TANote/adapter red test fails |
| `red.tanote.envelope_exposes_tmux` | Agent-visible envelope exposes tmux session, pane, session path, or zterm endpoint | TANote/adapter red test fails |
| `red.resource.no_lease` | Long-lived resource is created without Resource Lifecycle lease | resource lifecycle red test fails |
| `red.resource.non_owner_release` | Module releases resource owned by another module/entity | resource lifecycle red test fails |
| `red.resource.double_acquire_exclusive` | Exclusive resource has two active leases | resource lifecycle red test fails |
| `red.resource.orphan_hidden` | Orphan resource is not persisted and not visible in debug snapshot | resource/debug red test fails |
| `red.resource.leak_hidden` | Leaked resource is absent from debug bundle | resource/debug red test fails |
| `red.resource.cleanup_without_receipt` | Cleanup is reported successful without owner receipt and event receipt | resource/persistence red test fails |
| `red.resource.broad_cleanup` | Cleanup uses broad process kill or broad file deletion | resource/safety red test fails |
| `red.resource.growth_silent` | Resource grows without bounded release/drain/cleanup visibility | resource/efficiency red test fails |
| `red.resource.temp_left_after_shutdown` | Scoped daemon/session shutdown leaves tracked temporary files without cleanup result | startup/resource red test fails |
| `red.debug.missing_resource_snapshot` | MVP debug bundle lacks Resource Lifecycle snapshot | debug/resource red test fails |
| `red.agent.exposes_tmux_session` | Agent-facing CLI/skill/projection exposes tmux session name, pane id, session path, or zterm endpoint | doc/API red test fails |
| `red.manager.skill_missing_ops` | manager skill lacks init/query/task/message/wait instructions | skill doc red test fails |
| `red.manager.skill_missing_tanote` | manager/agent skill lacks TANote post/read/thread instructions | skill doc red test fails |
| `red.gateway.non_adjacent_conversion` | Raw CLI becomes daemon command directly | pipeline red test fails |
| `red.ui.bypass_input_gateway` | UI/WebUI sends command directly to runtime/module | UI/input red test fails |
| `red.ui.bypass_output_gateway` | UI/WebUI reads module state instead of projection | UI/output red test fails |
| `red.ui.owns_framework_state` | UI/WebUI persists or mutates agent/task/comm state | UI red test fails |
| `red.ui.direct_transport` | UI/WebUI calls tmux/zterm directly | UI/adapter red test fails |
| `red.cli.broad_kill_doc` | Docs or code use broad-kill command patterns | doc/code scan fails |
| `red.required_file_untracked` | Required file missing from git tracking | xtask gate fails |

## Future Implementation

Red tests should run through:

```text
cargo xtask red-tests
```

No production code should be accepted until matching red tests exist for its feature.
