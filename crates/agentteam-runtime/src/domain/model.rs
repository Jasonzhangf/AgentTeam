use agentteam_contracts::domain::DomainRouteEndpoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainEndpoint {
    pub host: String,
    pub port: u16,
    pub auth_token_present: bool,
}

impl DomainEndpoint {
    pub fn new(host: impl Into<String>, port: u16, auth_token: impl AsRef<str>) -> Self {
        Self {
            host: host.into(),
            port,
            auth_token_present: !auth_token.as_ref().is_empty(),
        }
    }

    pub fn route_endpoint(&self) -> DomainRouteEndpoint {
        DomainRouteEndpoint::new(self.host.clone(), self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredDomain {
    pub id: String,
    pub aliases: Vec<String>,
    pub endpoint: DomainEndpoint,
    pub is_local: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainTargetKind {
    Agent(String),
    Role(String),
    Team(String),
    All,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainRouteKind {
    Local,
    Remote,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDomainTarget {
    pub original_target: String,
    pub target_kind: DomainTargetKind,
    pub domain_id: String,
    pub route_kind: DomainRouteKind,
    pub endpoint: DomainRouteEndpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainRegistrySnapshot {
    pub local_domain_id: String,
    pub aliases: Vec<String>,
    pub remote_domain_ids: Vec<String>,
    pub endpoint_count: usize,
    pub token_redaction_status: String,
}
