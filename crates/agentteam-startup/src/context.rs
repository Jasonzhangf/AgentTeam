use agentteam_config::{MemberConfig, NormalizedConfig};

pub(crate) struct AgentStartupContext<'a> {
    pub normalized: &'a NormalizedConfig,
    pub runtime_home: &'a str,
    pub team_id: &'a str,
    pub cwd: &'a str,
    pub member: &'a MemberConfig,
    pub skill_path: &'a str,
    pub cli_path: &'a str,
    pub config_path: Option<&'a str>,
}
