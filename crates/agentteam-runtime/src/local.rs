use std::path::{Path, PathBuf};

use agentteam_config::{
    check_config_path, normalize_config, validate_config_path, ConfigCenterError,
    RemoteDaemonConfig, UserConfig,
};
use agentteam_contracts::team::TeamReq03ValidatedIntent;
use agentteam_debug::{capture_debug_bundle, DebugBundleInput, DebugError};
use agentteam_resource::ResourceRegistry;
use agentteam_tmux::{run_tmux_loopback, TmuxAdapterError, TmuxLoopbackInput};

use crate::domain::{registered_domain, DomainEndpoint, DomainRegistry, DomainRegistryError};
use crate::local_projection::{
    config_result, daemon_check_result, debug_bundle_result, domain_snapshot_result,
    resolved_domain_result, task_board_result, task_changed_result, tmux_loopback_result,
};
pub use crate::local_projection::{LocalCommandError, LocalCommandResult};
use crate::task::{
    TaskClaimInput, TaskCreateInput, TaskEngine, TaskEngineError, TaskTargetKind,
    TaskTransitionInput,
};

pub fn execute_local_intent(
    intent: TeamReq03ValidatedIntent,
) -> Result<LocalCommandResult, LocalCommandError> {
    match intent {
        TeamReq03ValidatedIntent::ConfigCheck { config_path, .. } => {
            execute_config_check(config_path)
        }
        TeamReq03ValidatedIntent::DaemonCheck { config_path, .. } => {
            execute_daemon_check(config_path)
        }
        TeamReq03ValidatedIntent::DomainResolve {
            target,
            config_path,
            ..
        } => execute_domain_resolve(target, config_path),
        TeamReq03ValidatedIntent::DebugSnapshot {
            config_path,
            runtime_home,
            ..
        } => execute_debug_snapshot(config_path, runtime_home),
        TeamReq03ValidatedIntent::TaskSend {
            runtime_home,
            team_id,
            created_by,
            target_kind,
            target,
            title,
            body,
            ..
        } => execute_task_send(
            runtime_home,
            team_id,
            created_by,
            target_kind,
            target,
            title,
            body,
        ),
        TeamReq03ValidatedIntent::TaskList { runtime_home, .. } => execute_task_list(runtime_home),
        TeamReq03ValidatedIntent::TaskStatus {
            runtime_home,
            task_id,
            ..
        } => execute_task_status(runtime_home, task_id),
        TeamReq03ValidatedIntent::TaskDone {
            runtime_home,
            task_id,
            actor,
            detail,
            ..
        } => execute_task_done(runtime_home, task_id, actor, detail),
        TeamReq03ValidatedIntent::TaskError {
            runtime_home,
            task_id,
            actor,
            detail,
            ..
        } => execute_task_error(runtime_home, task_id, actor, detail),
        TeamReq03ValidatedIntent::TaskClaim {
            runtime_home,
            worker_name,
            worker_role,
            ..
        } => execute_task_claim(runtime_home, worker_name, worker_role),
        TeamReq03ValidatedIntent::TmuxLoopback {
            runtime_home,
            session_count,
            ..
        } => execute_tmux_loopback(runtime_home, session_count),
    }
}

fn execute_config_check(config_path: String) -> Result<LocalCommandResult, LocalCommandError> {
    let normalized = check_config_path(config_path).map_err(config_error)?;
    Ok(LocalCommandResult::ConfigCheck {
        normalized: config_result(normalized),
    })
}

fn execute_daemon_check(config_path: String) -> Result<LocalCommandResult, LocalCommandError> {
    let validated = validate_config_path(config_path).map_err(config_error)?;
    let normalized = config_result(normalize_config(validated.clone()).map_err(config_error)?);
    let registry = build_domain_registry(&validated.user_config)?;
    Ok(LocalCommandResult::DaemonCheck {
        daemon: daemon_check_result(normalized, registry.snapshot()),
    })
}

fn execute_domain_resolve(
    target: String,
    config_path: String,
) -> Result<LocalCommandResult, LocalCommandError> {
    let validated = validate_config_path(config_path).map_err(config_error)?;
    let registry = build_domain_registry(&validated.user_config)?;
    let resolved = registry.resolve_target(target).map_err(domain_error)?;
    Ok(LocalCommandResult::DomainResolve {
        target: resolved_domain_result(resolved),
        registry_snapshot: domain_snapshot_result(registry.snapshot()),
    })
}

fn execute_debug_snapshot(
    config_path: String,
    runtime_home: String,
) -> Result<LocalCommandResult, LocalCommandError> {
    let normalized = check_config_path(config_path).map_err(config_error)?;
    let event_log = event_log_path(&runtime_home);
    let mut resources = ResourceRegistry::new();
    let bundle = capture_debug_bundle(
        &event_log,
        &mut resources,
        DebugBundleInput {
            requested_by: "agentteam-cli".to_owned(),
            scope: normalized.project_slug,
            module: "mvp.local".to_owned(),
        },
    )
    .map_err(debug_error)?;
    Ok(LocalCommandResult::DebugSnapshot {
        bundle: debug_bundle_result(bundle, event_log),
    })
}

fn execute_task_send(
    runtime_home: String,
    team_id: String,
    created_by: String,
    target_kind: String,
    target: String,
    title: String,
    body: String,
) -> Result<LocalCommandResult, LocalCommandError> {
    let engine = TaskEngine::new(event_log_path(runtime_home));
    let changed = engine
        .create_task(TaskCreateInput {
            team_id,
            created_by,
            target_kind: parse_task_target_kind(&target_kind)?,
            target,
            title,
            body,
            priority: 100,
            blocked: false,
        })
        .map_err(task_error)?;
    Ok(LocalCommandResult::TaskSend {
        task: task_changed_result(changed),
    })
}

fn execute_task_list(runtime_home: String) -> Result<LocalCommandResult, LocalCommandError> {
    let engine = TaskEngine::new(event_log_path(runtime_home));
    Ok(LocalCommandResult::TaskList {
        board: task_board_result(engine.board().map_err(task_error)?),
    })
}

fn execute_task_status(
    runtime_home: String,
    task_id: String,
) -> Result<LocalCommandResult, LocalCommandError> {
    let engine = TaskEngine::new(event_log_path(runtime_home));
    Ok(LocalCommandResult::TaskStatus {
        board: task_board_result(engine.status(&task_id).map_err(task_error)?),
    })
}

fn execute_task_done(
    runtime_home: String,
    task_id: String,
    actor: String,
    detail: String,
) -> Result<LocalCommandResult, LocalCommandError> {
    let engine = TaskEngine::new(event_log_path(runtime_home));
    let changed = engine
        .mark_done(TaskTransitionInput {
            task_id,
            actor,
            detail,
        })
        .map_err(task_error)?;
    Ok(LocalCommandResult::TaskDone {
        task: task_changed_result(changed),
    })
}

fn execute_task_error(
    runtime_home: String,
    task_id: String,
    actor: String,
    detail: String,
) -> Result<LocalCommandResult, LocalCommandError> {
    let engine = TaskEngine::new(event_log_path(runtime_home));
    let changed = engine
        .mark_error(TaskTransitionInput {
            task_id,
            actor,
            detail,
        })
        .map_err(task_error)?;
    Ok(LocalCommandResult::TaskError {
        task: task_changed_result(changed),
    })
}

fn execute_task_claim(
    runtime_home: String,
    worker_name: String,
    worker_role: String,
) -> Result<LocalCommandResult, LocalCommandError> {
    let engine = TaskEngine::new(event_log_path(runtime_home));
    let changed = engine
        .claim_task(TaskClaimInput {
            worker_name,
            worker_role,
        })
        .map_err(task_error)?;
    Ok(LocalCommandResult::TaskClaim {
        task: task_changed_result(changed),
    })
}

fn execute_tmux_loopback(
    runtime_home: String,
    session_count: String,
) -> Result<LocalCommandResult, LocalCommandError> {
    let count = parse_session_count(&session_count)?;
    let report =
        run_tmux_loopback(TmuxLoopbackInput::new(runtime_home, count)).map_err(tmux_error)?;
    Ok(LocalCommandResult::TmuxLoopback {
        loopback: tmux_loopback_result(report),
    })
}

fn build_domain_registry(user_config: &UserConfig) -> Result<DomainRegistry, LocalCommandError> {
    let mut registry = DomainRegistry::new(registered_domain(
        user_config.daemon_domain.id.clone(),
        user_config.daemon_domain.aliases.clone(),
        DomainEndpoint::new(
            user_config.runtime.host.clone(),
            user_config.runtime.port,
            "",
        ),
        true,
    ))
    .map_err(domain_error)?;
    if let Some(domains) = &user_config.daemon_domains {
        if let Some(remotes) = &domains.remote {
            for remote in remotes {
                register_remote_domain(&mut registry, remote)?;
            }
        }
    }
    Ok(registry)
}

fn register_remote_domain(
    registry: &mut DomainRegistry,
    remote: &RemoteDaemonConfig,
) -> Result<(), LocalCommandError> {
    registry
        .register_remote(registered_domain(
            remote.id.clone(),
            remote.aliases.clone(),
            DomainEndpoint::new(remote.host.clone(), remote.port, &remote.auth_token),
            false,
        ))
        .map_err(domain_error)
}

fn parse_task_target_kind(value: &str) -> Result<TaskTargetKind, LocalCommandError> {
    match value {
        "agent" => Ok(TaskTargetKind::Agent),
        "role" => Ok(TaskTargetKind::Role),
        other => Err(LocalCommandError::Task {
            reason: format!("unsupported task target kind {other}; expected agent or role"),
        }),
    }
}

fn parse_session_count(value: &str) -> Result<usize, LocalCommandError> {
    value
        .parse::<usize>()
        .map_err(|error| LocalCommandError::Tmux {
            reason: format!("invalid --session-count {value}: {error}"),
        })
}

fn event_log_path(runtime_home: impl AsRef<Path>) -> PathBuf {
    runtime_home.as_ref().join("events").join("agentteam.jsonl")
}

fn config_error(error: ConfigCenterError) -> LocalCommandError {
    LocalCommandError::Config {
        reason: error.reason().to_owned(),
    }
}

fn domain_error(error: DomainRegistryError) -> LocalCommandError {
    LocalCommandError::Domain {
        reason: format!("{error:?}"),
    }
}

fn debug_error(error: DebugError) -> LocalCommandError {
    LocalCommandError::Debug {
        reason: error.reason().to_owned(),
    }
}

fn task_error(error: TaskEngineError) -> LocalCommandError {
    LocalCommandError::Task {
        reason: error.reason(),
    }
}

fn tmux_error(error: TmuxAdapterError) -> LocalCommandError {
    LocalCommandError::Tmux {
        reason: error.reason(),
    }
}
