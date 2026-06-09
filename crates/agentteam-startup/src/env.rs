use std::collections::BTreeMap;

use crate::context::AgentStartupContext;

pub(crate) fn build_agent_env(
    context: &AgentStartupContext<'_>,
    session_name: &str,
) -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    env.insert("AGENTTEAM_NAME".to_owned(), context.member.name.clone());
    env.insert("AGENTTEAM_ROLE".to_owned(), context.member.role.clone());
    env.insert(
        "AGENTTEAM_TEAM_ROLE".to_owned(),
        context.member.team_role.clone(),
    );
    env.insert("AGENTTEAM_TEAM".to_owned(), context.team_id.to_owned());
    env.insert(
        "AGENTTEAM_PROJECT_SLUG".to_owned(),
        context.normalized.project_slug.clone(),
    );
    env.insert(
        "AGENTTEAM_PROJECT_ROOT".to_owned(),
        context.normalized.project_root.clone(),
    );
    env.insert(
        "AGENTTEAM_RUNTIME_HOME".to_owned(),
        context.runtime_home.to_owned(),
    );
    env.insert("AGENTTEAM_SCOPE_CWD".to_owned(), context.cwd.to_owned());
    env.insert(
        "AGENTTEAM_DOMAIN".to_owned(),
        context.normalized.local_domain_id.clone(),
    );
    env.insert("AGENTTEAM_SESSION_NAME".to_owned(), session_name.to_owned());
    env.insert("AGENTTEAM_SKILL".to_owned(), "agentteam".to_owned());
    env.insert(
        "AGENTTEAM_SKILL_PATH".to_owned(),
        context.skill_path.to_owned(),
    );
    env.insert("AGENTTEAM_CLI".to_owned(), context.cli_path.to_owned());
    for (key, value) in &context.member.env {
        env.insert(key.clone(), value.clone());
    }
    env
}
