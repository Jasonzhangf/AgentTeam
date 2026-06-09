use agentteam_config::{MemberConfig, TeamConfig, ValidatedConfig};

use crate::error::{StartupError, StartupResult};

pub(crate) fn select_team<'a>(
    validated: &'a ValidatedConfig,
    team_id: Option<&str>,
) -> StartupResult<&'a TeamConfig> {
    if let Some(team_id) = team_id {
        return validated
            .user_config
            .teams
            .iter()
            .find(|team| team.id == team_id)
            .ok_or_else(|| StartupError::Team {
                reason: format!("team {team_id} not found in config"),
            });
    }
    if validated.user_config.teams.len() == 1 {
        Ok(&validated.user_config.teams[0])
    } else {
        Err(StartupError::Team {
            reason: "--team is required when more than one team is configured".to_owned(),
        })
    }
}

pub(crate) fn select_root_manager(team: &TeamConfig) -> StartupResult<&MemberConfig> {
    team.members
        .iter()
        .find(|member| member.team_role == "manager")
        .ok_or_else(|| StartupError::Team {
            reason: format!("team {} does not contain a manager", team.id),
        })
}

pub(crate) fn select_worker<'a>(
    team: &'a TeamConfig,
    agent_name: &str,
) -> StartupResult<&'a MemberConfig> {
    team.members
        .iter()
        .find(|member| member.name == agent_name && member.team_role == "worker")
        .ok_or_else(|| StartupError::Team {
            reason: format!("team {} does not contain worker {agent_name}", team.id),
        })
}

pub(crate) fn worker_names(team: &TeamConfig) -> Vec<String> {
    team.members
        .iter()
        .filter(|member| member.team_role == "worker")
        .map(|member| member.name.clone())
        .collect()
}
