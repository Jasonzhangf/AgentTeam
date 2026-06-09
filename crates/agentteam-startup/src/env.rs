use std::collections::BTreeMap;

use agentteam_config::{MemberConfig, NormalizedConfig};

pub(crate) fn build_agent_env(
    normalized: &NormalizedConfig,
    runtime_home: &str,
    team_id: &str,
    cwd: &str,
    session_name: &str,
    member: &MemberConfig,
) -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    env.insert("AGENTTEAM_NAME".to_owned(), member.name.clone());
    env.insert("AGENTTEAM_ROLE".to_owned(), member.role.clone());
    env.insert("AGENTTEAM_TEAM_ROLE".to_owned(), member.team_role.clone());
    env.insert("AGENTTEAM_TEAM".to_owned(), team_id.to_owned());
    env.insert(
        "AGENTTEAM_PROJECT_SLUG".to_owned(),
        normalized.project_slug.clone(),
    );
    env.insert(
        "AGENTTEAM_PROJECT_ROOT".to_owned(),
        normalized.project_root.clone(),
    );
    env.insert("AGENTTEAM_RUNTIME_HOME".to_owned(), runtime_home.to_owned());
    env.insert("AGENTTEAM_SCOPE_CWD".to_owned(), cwd.to_owned());
    env.insert(
        "AGENTTEAM_DOMAIN".to_owned(),
        normalized.local_domain_id.clone(),
    );
    env.insert("AGENTTEAM_SESSION_NAME".to_owned(), session_name.to_owned());
    env.insert("AGENTTEAM_SKILL".to_owned(), "agentteam".to_owned());
    for (key, value) in &member.env {
        env.insert(key.clone(), value.clone());
    }
    env
}
