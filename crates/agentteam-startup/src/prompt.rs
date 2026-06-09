use agentteam_config::{MemberConfig, NormalizedConfig, TeamConfig};

use crate::select::worker_names;

pub(crate) fn build_root_manager_bootstrap_prompt(
    normalized: &NormalizedConfig,
    runtime_home: &str,
    team_id: &str,
    cwd: &str,
    member: &MemberConfig,
    team: &TeamConfig,
    config_path: Option<&str>,
) -> String {
    let config = config_path.unwrap_or("~/.agentteam/config.toml");
    let workers = worker_names(team).join(", ");
    format!(
        "You are {name}, the AgentTeam root manager for project {project}.\n\
Read .agents/skills/agentteam/SKILL.md now and follow it as your operating skill.\n\
Your startup params: name={name}, role={role}, team_role=manager, team={team}, domain={domain}, project_root={root}, runtime_home={runtime_home}, cwd={cwd}.\n\
Config path: {config}.\n\
Your worker pool from config: {workers}.\n\
Use only AgentTeam CLI commands for framework operations. Do not call tmux directly and do not edit state files.\n\
Your manager loop: report ready with `agentteam ready report`, inspect tasks with `agentteam task list/status`, create work with `agentteam task send`, contact workers with `agentteam msg send` or `agentteam msg broadcast`, and wait by task/message/debug projections.\n\
To initialize a worker TUI session, use `agentteam start worker --agent <worker> --team {team} --config {config} --json`.\n\
Acknowledge by stating your identity, the skill file you will read, and the first CLI command you will use to report ready.",
        name = member.name,
        project = normalized.project_slug,
        role = member.role,
        team = team_id,
        domain = normalized.local_domain_id,
        root = normalized.project_root,
        runtime_home = runtime_home,
        cwd = cwd,
        config = config,
        workers = workers
    )
}

pub(crate) fn build_worker_bootstrap_prompt(
    normalized: &NormalizedConfig,
    runtime_home: &str,
    team_id: &str,
    cwd: &str,
    member: &MemberConfig,
    config_path: Option<&str>,
) -> String {
    format!(
        "You are {name}, an AgentTeam worker for project {project}.\n\
Read .agents/skills/agentteam/SKILL.md now and follow it as your operating skill.\n\
Your startup params: name={name}, role={role}, team_role=worker, team={team}, domain={domain}, project_root={root}, runtime_home={runtime_home}, cwd={cwd}.\n\
Config path: {config}.\n\
Use only AgentTeam CLI commands for framework operations. Do not call tmux directly and do not edit state files.\n\
Your worker loop: send `agentteam ready report`, claim work with `agentteam task claim`, inspect with `agentteam task status`, complete with `agentteam task done`, and report normal task failure with `agentteam task error`.\n\
Acknowledge by stating your identity, your role, and the first CLI command you will use to report ready.",
        name = member.name,
        project = normalized.project_slug,
        role = member.role,
        team = team_id,
        domain = normalized.local_domain_id,
        root = normalized.project_root,
        runtime_home = runtime_home,
        cwd = cwd,
        config = config_path.unwrap_or("~/.agentteam/config.toml")
    )
}
