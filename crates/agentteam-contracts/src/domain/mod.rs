use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "domain.registry";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainReq01RawTarget {
    pub target: String,
}

impl DomainReq01RawTarget {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Domain", "Req", 1, "RawTarget");

    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
        }
    }

    pub fn validate(self, local_domain_id: impl Into<String>) -> DomainReq02Validated {
        DomainReq02Validated {
            target: self.target,
            local_domain_id: local_domain_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainReq02Validated {
    pub target: String,
    pub local_domain_id: String,
}

impl DomainReq02Validated {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Domain", "Req", 2, "Validated");

    pub fn resolve_agent(
        self,
        agent_name: impl Into<String>,
        domain_id: impl Into<String>,
    ) -> DomainAgentAddr03Resolved {
        DomainAgentAddr03Resolved {
            original_target: self.target,
            agent_name: agent_name.into(),
            domain_id: domain_id.into(),
            local_domain_id: self.local_domain_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainAgentAddr03Resolved {
    pub original_target: String,
    pub agent_name: String,
    pub domain_id: String,
    pub local_domain_id: String,
}

impl DomainAgentAddr03Resolved {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("DomainAgentAddr", "", 3, "Resolved");

    pub fn plan_route(self, endpoint: DomainRouteEndpoint) -> DomainRoute04Plan {
        let is_local = self.domain_id == self.local_domain_id;
        DomainRoute04Plan {
            agent_name: self.agent_name,
            domain_id: self.domain_id,
            is_local,
            endpoint,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainRoute04Plan {
    pub agent_name: String,
    pub domain_id: String,
    pub is_local: bool,
    pub endpoint: DomainRouteEndpoint,
}

impl DomainRoute04Plan {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("DomainRoute", "", 4, "Plan");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainRouteEndpoint {
    pub host: String,
    pub port: u16,
}

impl DomainRouteEndpoint {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_chain_plans_local_route() {
        let route = DomainReq01RawTarget::new("Kevin@local")
            .validate("local")
            .resolve_agent("Kevin", "local")
            .plan_route(DomainRouteEndpoint::new("127.0.0.1", 17680));

        assert!(route.is_local);
        assert_eq!(route.agent_name, "Kevin");
        assert_eq!(DomainReq02Validated::NODE.number, 2);
        assert_eq!(DomainRoute04Plan::NODE.number, 4);
    }

    #[test]
    fn domain_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "domain.registry");
    }
}
