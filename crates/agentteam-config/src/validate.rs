use crate::error::{ConfigCenterError, ConfigCenterResult};
use crate::model::{RemoteDaemonConfig, TeamConfig, UserConfig};
use crate::parse::ParsedConfig;
use agentteam_contracts::config::ConfigReq04ValidatedUserConfig;
use std::collections::BTreeSet;

const FORBIDDEN_RUNTIME_KEYS: &[&str] = &[
    "task_list",
    "message_list",
    "event_log",
    "debug_snapshot",
    "daemon_pid",
    "agent_runtime_status",
    "tmux_pane_state",
    "zterm_buffer_state",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedConfig {
    pub node: ConfigReq04ValidatedUserConfig,
    pub user_config: UserConfig,
}

pub fn validate_config(parsed: ParsedConfig) -> ConfigCenterResult<ValidatedConfig> {
    reject_runtime_state_keys(&parsed.node.path, &parsed.user_config)?;
    require_valid_project_slug(&parsed.node.path, &parsed.user_config.project.slug)?;
    require_non_empty(
        &parsed.node.path,
        "project.root",
        &parsed.user_config.project.root,
    )?;
    require_non_empty(
        &parsed.node.path,
        "runtime.home",
        &parsed.user_config.runtime.home,
    )?;
    require_non_empty(
        &parsed.node.path,
        "runtime.host",
        &parsed.user_config.runtime.host,
    )?;
    require_non_empty(
        &parsed.node.path,
        "tmux.binary",
        &parsed.user_config.tmux.binary,
    )?;
    require_tmux_prefix(&parsed.node.path, &parsed.user_config.tmux.managed_prefix)?;
    validate_domains(&parsed.node.path, &parsed.user_config)?;
    validate_teams(&parsed.node.path, &parsed.user_config.teams)?;

    let node = parsed.node.validate_user_config(
        parsed.user_config.project.slug.clone(),
        parsed.user_config.daemon_domain.id.clone(),
    );
    Ok(ValidatedConfig {
        node,
        user_config: parsed.user_config,
    })
}

fn reject_runtime_state_keys(path: &str, user_config: &UserConfig) -> ConfigCenterResult<()> {
    let debug = format!("{user_config:?}");
    for forbidden in FORBIDDEN_RUNTIME_KEYS {
        if debug.contains(forbidden) {
            return validation_error(path, format!("runtime state key is forbidden: {forbidden}"));
        }
    }
    Ok(())
}

fn require_valid_project_slug(path: &str, slug: &str) -> ConfigCenterResult<()> {
    require_non_empty(path, "project.slug", slug)?;
    if slug
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
    {
        Ok(())
    } else {
        validation_error(path, format!("invalid project.slug: {slug}"))
    }
}

fn require_tmux_prefix(path: &str, prefix: &str) -> ConfigCenterResult<()> {
    if prefix == "TA" {
        Ok(())
    } else {
        validation_error(
            path,
            format!("tmux.managed_prefix must be TA, got {prefix}"),
        )
    }
}

fn validate_domains(path: &str, user_config: &UserConfig) -> ConfigCenterResult<()> {
    require_non_empty(path, "daemon_domain.id", &user_config.daemon_domain.id)?;
    let mut ids = BTreeSet::new();
    insert_unique(path, &mut ids, "domain id", &user_config.daemon_domain.id)?;
    let mut aliases = BTreeSet::new();
    for alias in &user_config.daemon_domain.aliases {
        insert_unique(path, &mut aliases, "domain alias", alias)?;
    }
    for remote in remote_domains(user_config) {
        validate_remote_domain(path, remote, &mut ids, &mut aliases)?;
    }
    Ok(())
}

fn validate_remote_domain(
    path: &str,
    remote: &RemoteDaemonConfig,
    ids: &mut BTreeSet<String>,
    aliases: &mut BTreeSet<String>,
) -> ConfigCenterResult<()> {
    require_non_empty(path, "daemon_domains.remote.id", &remote.id)?;
    require_non_empty(path, "daemon_domains.remote.host", &remote.host)?;
    insert_unique(path, ids, "domain id", &remote.id)?;
    for alias in &remote.aliases {
        insert_unique(path, aliases, "domain alias", alias)?;
    }
    Ok(())
}

fn validate_teams(path: &str, teams: &[TeamConfig]) -> ConfigCenterResult<()> {
    if teams.is_empty() {
        return validation_error(path, "at least one team is required");
    }
    let mut team_ids = BTreeSet::new();
    let mut manager_count = 0usize;
    for team in teams {
        insert_unique(path, &mut team_ids, "team id", &team.id)?;
        require_non_empty(path, "teams.category", &team.category)?;
        if team.agent_count != team.members.len() {
            return validation_error(
                path,
                format!(
                    "team {} agent_count {} does not match members {}",
                    team.id,
                    team.agent_count,
                    team.members.len()
                ),
            );
        }
        manager_count += validate_team_members(path, team)?;
    }
    if manager_count == 1 {
        Ok(())
    } else {
        validation_error(
            path,
            format!("v1 requires exactly one manager Kevin, got {manager_count}"),
        )
    }
}

fn validate_team_members(path: &str, team: &TeamConfig) -> ConfigCenterResult<usize> {
    let mut names = BTreeSet::new();
    let mut manager_count = 0usize;
    for member in &team.members {
        require_non_empty(path, "teams.members.name", &member.name)?;
        require_non_empty(path, "teams.members.team_role", &member.team_role)?;
        require_non_empty(path, "teams.members.role", &member.role)?;
        require_non_empty(path, "teams.members.cwd", &member.cwd)?;
        require_non_empty(path, "teams.members.command", &member.command)?;
        insert_unique(path, &mut names, "member name", &member.name)?;
        if member.team_role == "manager" {
            if member.name != "Kevin" {
                return validation_error(path, "v1 manager name must be Kevin");
            }
            manager_count += 1;
        }
    }
    Ok(manager_count)
}

fn remote_domains(user_config: &UserConfig) -> &[RemoteDaemonConfig] {
    user_config
        .daemon_domains
        .as_ref()
        .and_then(|domains| domains.remote.as_deref())
        .unwrap_or(&[])
}

fn insert_unique(
    path: &str,
    values: &mut BTreeSet<String>,
    label: &str,
    value: &str,
) -> ConfigCenterResult<()> {
    require_non_empty(path, label, value)?;
    if values.insert(value.to_owned()) {
        Ok(())
    } else {
        validation_error(path, format!("duplicate {label}: {value}"))
    }
}

fn require_non_empty(path: &str, field: &str, value: &str) -> ConfigCenterResult<()> {
    if value.trim().is_empty() {
        validation_error(path, format!("{field} is required"))
    } else {
        Ok(())
    }
}

fn validation_error<T>(path: &str, reason: impl Into<String>) -> ConfigCenterResult<T> {
    Err(ConfigCenterError::Validation {
        path: path.to_owned(),
        reason: reason.into(),
    })
}
