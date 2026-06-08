use crate::model::NormalizedConfig;
use agentteam_contracts::config::ConfigResp05RuntimeConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigSnapshot {
    pub snapshot_id: String,
    pub project_slug: String,
    pub runtime_home: String,
    pub local_domain_id: String,
    pub team_count: usize,
    pub member_count: usize,
    pub zterm_endpoint: String,
    pub zterm_token_redacted: bool,
    pub remote_domain_count: usize,
    pub validation_status: String,
}

pub fn snapshot_config(
    config: &NormalizedConfig,
    snapshot_id: impl Into<String>,
) -> ConfigSnapshot {
    let response = ConfigResp05RuntimeConfig {
        path: config.path.clone(),
        project_slug: config.project_slug.clone(),
        local_domain_id: config.local_domain_id.clone(),
        runtime_home: config.runtime_home.clone(),
    }
    .snapshot(snapshot_id);

    ConfigSnapshot {
        snapshot_id: response.snapshot_id,
        project_slug: response.project_slug,
        runtime_home: response.runtime_home,
        local_domain_id: response.local_domain_id,
        team_count: config.team_count,
        member_count: config.member_count,
        zterm_endpoint: config.zterm_endpoint.clone(),
        zterm_token_redacted: true,
        remote_domain_count: config.remote_domain_count,
        validation_status: "valid".to_owned(),
    }
}
