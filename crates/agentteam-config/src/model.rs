use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct UserConfig {
    pub project: ProjectConfig,
    pub runtime: RuntimeConfig,
    pub tmux: TmuxConfig,
    pub zterm: ZtermConfig,
    pub daemon_domain: DaemonDomainConfig,
    pub daemon_domains: Option<DaemonDomainsConfig>,
    pub team_defaults: Option<TeamDefaultsConfig>,
    pub teams: Vec<TeamConfig>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ProjectConfig {
    pub slug: String,
    pub root: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub home: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TmuxConfig {
    pub managed_prefix: String,
    pub binary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ZtermConfig {
    pub host: String,
    pub port: u16,
    pub auth_token: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DaemonDomainConfig {
    pub id: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DaemonDomainsConfig {
    pub remote: Option<Vec<RemoteDaemonConfig>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RemoteDaemonConfig {
    pub id: String,
    pub aliases: Vec<String>,
    pub host: String,
    pub port: u16,
    pub auth_token: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TeamDefaultsConfig {
    pub category: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TeamConfig {
    pub id: String,
    pub category: String,
    pub agent_count: usize,
    pub members: Vec<MemberConfig>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberConfig {
    pub name: String,
    pub team_role: String,
    pub role: String,
    pub cwd: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedConfig {
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
