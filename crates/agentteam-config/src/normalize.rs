use crate::error::ConfigCenterResult;
use crate::model::NormalizedConfig;
use crate::validate::ValidatedConfig;

pub fn normalize_config(validated: ValidatedConfig) -> ConfigCenterResult<NormalizedConfig> {
    let user_config = validated.user_config;
    let remote_domain_count = user_config
        .daemon_domains
        .as_ref()
        .and_then(|domains| domains.remote.as_ref())
        .map_or(0, Vec::len);
    let member_count = user_config
        .teams
        .iter()
        .map(|team| team.members.len())
        .sum();

    let node = validated
        .node
        .normalize_runtime(user_config.runtime.home.clone());

    Ok(NormalizedConfig {
        path: node.path,
        project_slug: node.project_slug,
        project_root: user_config.project.root,
        runtime_home: node.runtime_home,
        local_domain_id: node.local_domain_id,
        team_count: user_config.teams.len(),
        member_count,
        zterm_endpoint: format!("{}:{}", user_config.zterm.host, user_config.zterm.port),
        remote_domain_count,
    })
}
