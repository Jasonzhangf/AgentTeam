use agentteam_config::TeamConfig;

use crate::context::AgentStartupContext;
use crate::select::worker_names;

pub(crate) fn build_root_manager_bootstrap_prompt(
    context: &AgentStartupContext<'_>,
    team: &TeamConfig,
) -> String {
    let config = context.config_path.unwrap_or("~/.agentteam/config.toml");
    let workers = worker_names(team).join(", ");
    format!(
        "You are {name}, the AgentTeam root manager for project {project}.\n\
Read {skill_path} now and follow it as your operating skill.\n\
Your startup params: name={name}, role={role}, team_role=manager, team={team}, domain={domain}, project_root={root}, runtime_home={runtime_home}, cwd={cwd}.\n\
Config path: {config}.\n\
AgentTeam CLI path: {cli_path}. Use this absolute CLI path when `agentteam` is not on PATH.\n\
Your worker pool from config: {workers}.\n\
Use only AgentTeam CLI commands for framework operations. Do not call tmux directly and do not edit state files.\n\
Your manager loop: report ready with `$AGENTTEAM_CLI ready report`, inspect tasks with `$AGENTTEAM_CLI task list/status`, create work with `$AGENTTEAM_CLI task send`, contact workers with `$AGENTTEAM_CLI msg send` or `$AGENTTEAM_CLI msg broadcast`, and wait by task/message/debug projections.\n\
To initialize a worker TUI session, use `$AGENTTEAM_CLI start worker --agent <worker> --team {team} --config {config} --json`.\n\
Acknowledge by stating your identity, the skill file you will read, and the first CLI command you will use to report ready.",
        name = context.member.name,
        project = context.normalized.project_slug,
        role = context.member.role,
        team = context.team_id,
        domain = context.normalized.local_domain_id,
        root = context.normalized.project_root,
        runtime_home = context.runtime_home,
        cwd = context.cwd,
        skill_path = context.skill_path,
        cli_path = context.cli_path,
        config = config,
        workers = workers
    )
}

pub(crate) fn build_worker_bootstrap_prompt(context: &AgentStartupContext<'_>) -> String {
    format!(
        "You are {name}, an AgentTeam worker for project {project}.\n\
Read {skill_path} now and follow it as your operating skill.\n\
Your startup params: name={name}, role={role}, team_role=worker, team={team}, domain={domain}, project_root={root}, runtime_home={runtime_home}, cwd={cwd}.\n\
Config path: {config}.\n\
AgentTeam CLI path: {cli_path}. Use this absolute CLI path when `agentteam` is not on PATH.\n\
Use only AgentTeam CLI commands for framework operations. Do not call tmux directly and do not edit state files.\n\
Your worker loop: send `$AGENTTEAM_CLI ready report`, claim work with `$AGENTTEAM_CLI task claim`, inspect with `$AGENTTEAM_CLI task status`, complete with `$AGENTTEAM_CLI task done`, and report normal task failure with `$AGENTTEAM_CLI task error`.\n\
Acknowledge by stating your identity, your role, and the first CLI command you will use to report ready.",
        name = context.member.name,
        project = context.normalized.project_slug,
        role = context.member.role,
        team = context.team_id,
        domain = context.normalized.local_domain_id,
        root = context.normalized.project_root,
        runtime_home = context.runtime_home,
        cwd = context.cwd,
        skill_path = context.skill_path,
        cli_path = context.cli_path,
        config = context.config_path.unwrap_or("~/.agentteam/config.toml")
    )
}
