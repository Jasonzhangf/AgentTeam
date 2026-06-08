use std::path::{Path, PathBuf};

use agentteam_config::{
    check_config_path, validate_config_path, ConfigCenterError, NormalizedConfig,
    RemoteDaemonConfig, UserConfig,
};
use agentteam_contracts::debug::DebugResp03Bundle;
use agentteam_contracts::team::TeamReq03ValidatedIntent;
use agentteam_debug::{capture_debug_bundle, DebugBundleInput, DebugError};
use agentteam_resource::ResourceRegistry;
use serde::Serialize;

use crate::domain::{
    registered_domain, DomainEndpoint, DomainRegistry, DomainRegistryError, DomainRegistrySnapshot,
    DomainRouteKind, DomainTargetKind, ResolvedDomainTarget,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalCommandError {
    Config { reason: String },
    Domain { reason: String },
    Debug { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum LocalCommandResult {
    ConfigCheck {
        normalized: ConfigCheckResult,
    },
    DomainResolve {
        target: ResolvedDomainTargetResult,
        registry_snapshot: DomainRegistrySnapshotResult,
    },
    DebugSnapshot {
        bundle: DebugBundleResult,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigCheckResult {
    pub path: String,
    pub project_slug: String,
    pub project_root: String,
    pub runtime_home: String,
    pub local_domain_id: String,
    pub team_count: usize,
    pub member_count: usize,
    pub zterm_endpoint: String,
    pub remote_domain_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResolvedDomainTargetResult {
    pub original_target: String,
    pub target_kind: String,
    pub target_value: String,
    pub domain_id: String,
    pub route_kind: String,
    pub endpoint_host: String,
    pub endpoint_port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DomainRegistrySnapshotResult {
    pub local_domain_id: String,
    pub aliases: Vec<String>,
    pub remote_domain_ids: Vec<String>,
    pub endpoint_count: usize,
    pub token_redaction_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DebugBundleResult {
    pub bundle_id: String,
    pub persistence_receipt_id: String,
    pub resource_snapshot_id: String,
    pub module_count: usize,
    pub event_log_path: String,
}

pub fn execute_local_intent(
    intent: TeamReq03ValidatedIntent,
) -> Result<LocalCommandResult, LocalCommandError> {
    match intent {
        TeamReq03ValidatedIntent::ConfigCheck { config_path, .. } => {
            execute_config_check(config_path)
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
    }
}

fn execute_config_check(config_path: String) -> Result<LocalCommandResult, LocalCommandError> {
    let normalized = check_config_path(config_path).map_err(config_error)?;
    Ok(LocalCommandResult::ConfigCheck {
        normalized: config_result(normalized),
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

fn config_result(normalized: NormalizedConfig) -> ConfigCheckResult {
    ConfigCheckResult {
        path: normalized.path,
        project_slug: normalized.project_slug,
        project_root: normalized.project_root,
        runtime_home: normalized.runtime_home,
        local_domain_id: normalized.local_domain_id,
        team_count: normalized.team_count,
        member_count: normalized.member_count,
        zterm_endpoint: normalized.zterm_endpoint,
        remote_domain_count: normalized.remote_domain_count,
    }
}

fn resolved_domain_result(resolved: ResolvedDomainTarget) -> ResolvedDomainTargetResult {
    let (target_kind, target_value) = target_kind_parts(resolved.target_kind);
    ResolvedDomainTargetResult {
        original_target: resolved.original_target,
        target_kind,
        target_value,
        domain_id: resolved.domain_id,
        route_kind: route_kind_label(resolved.route_kind).to_owned(),
        endpoint_host: resolved.endpoint.host,
        endpoint_port: resolved.endpoint.port,
    }
}

fn domain_snapshot_result(snapshot: DomainRegistrySnapshot) -> DomainRegistrySnapshotResult {
    DomainRegistrySnapshotResult {
        local_domain_id: snapshot.local_domain_id,
        aliases: snapshot.aliases,
        remote_domain_ids: snapshot.remote_domain_ids,
        endpoint_count: snapshot.endpoint_count,
        token_redaction_status: snapshot.token_redaction_status,
    }
}

fn debug_bundle_result(bundle: DebugResp03Bundle, event_log: PathBuf) -> DebugBundleResult {
    DebugBundleResult {
        bundle_id: bundle.bundle_id,
        persistence_receipt_id: bundle.persistence_receipt_id,
        resource_snapshot_id: bundle.resource_snapshot_id,
        module_count: bundle.module_count,
        event_log_path: event_log.display().to_string(),
    }
}

fn event_log_path(runtime_home: impl AsRef<Path>) -> PathBuf {
    runtime_home.as_ref().join("events").join("agentteam.jsonl")
}

fn target_kind_parts(kind: DomainTargetKind) -> (String, String) {
    match kind {
        DomainTargetKind::Agent(value) => ("agent".to_owned(), value),
        DomainTargetKind::Role(value) => ("role".to_owned(), value),
        DomainTargetKind::Team(value) => ("team".to_owned(), value),
        DomainTargetKind::All => ("all".to_owned(), "all".to_owned()),
    }
}

fn route_kind_label(kind: DomainRouteKind) -> &'static str {
    match kind {
        DomainRouteKind::Local => "local",
        DomainRouteKind::Remote => "remote",
    }
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
