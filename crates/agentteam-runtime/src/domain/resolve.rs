use crate::domain::model::{
    DomainRouteKind, DomainTargetKind, RegisteredDomain, ResolvedDomainTarget,
};
use crate::domain::registry::{DomainRegistry, DomainRegistryError, DomainRegistryResult};
use agentteam_contracts::domain::DomainReq01RawTarget;

pub fn resolve_target(
    registry: &DomainRegistry,
    raw_target: String,
) -> DomainRegistryResult<ResolvedDomainTarget> {
    let validated =
        DomainReq01RawTarget::new(raw_target.clone()).validate(registry.local_domain_id());
    let (target_text, domain_text) = split_target_domain(&raw_target)?;
    let domain = resolve_domain(registry, domain_text, &raw_target)?;
    let target_kind = parse_target_kind(target_text)?;
    let agent_name = agent_name_for_contract(&target_kind);
    let route_plan = validated
        .resolve_agent(agent_name, domain.id.clone())
        .plan_route(domain.endpoint.route_endpoint());

    Ok(ResolvedDomainTarget {
        original_target: raw_target,
        target_kind,
        domain_id: route_plan.domain_id,
        route_kind: if route_plan.is_local {
            DomainRouteKind::Local
        } else {
            DomainRouteKind::Remote
        },
        endpoint: route_plan.endpoint,
    })
}

fn split_target_domain(raw_target: &str) -> DomainRegistryResult<(&str, Option<&str>)> {
    let mut parts = raw_target.split('@');
    let target = parts.next().unwrap_or_default();
    let domain = parts.next();
    if target.is_empty() || parts.next().is_some() {
        return Err(DomainRegistryError::InvalidTarget {
            target: raw_target.to_owned(),
            reason: "target must be target@domain or local bare target".to_owned(),
        });
    }
    Ok((target, domain))
}

fn resolve_domain<'a>(
    registry: &'a DomainRegistry,
    domain_text: Option<&str>,
    raw_target: &str,
) -> DomainRegistryResult<&'a RegisteredDomain> {
    match domain_text {
        Some(domain) => registry.domain_by_id_or_alias(domain).ok_or_else(|| {
            DomainRegistryError::UnknownDomain {
                domain: domain.to_owned(),
            }
        }),
        None => registry
            .domain_by_id_or_alias(registry.local_domain_id())
            .ok_or_else(|| DomainRegistryError::AmbiguousBareTarget {
                target: raw_target.to_owned(),
            }),
    }
}

fn parse_target_kind(target: &str) -> DomainRegistryResult<DomainTargetKind> {
    if target == "all" {
        return Ok(DomainTargetKind::All);
    }
    if let Some(role) = target.strip_prefix("role:") {
        return non_empty_target(role, "role").map(|role| DomainTargetKind::Role(role.to_owned()));
    }
    if let Some(team) = target.strip_prefix("team:") {
        return non_empty_target(team, "team").map(|team| DomainTargetKind::Team(team.to_owned()));
    }
    non_empty_target(target, "agent").map(|agent| DomainTargetKind::Agent(agent.to_owned()))
}

fn non_empty_target<'a>(value: &'a str, label: &str) -> DomainRegistryResult<&'a str> {
    if value.is_empty() {
        Err(DomainRegistryError::InvalidTarget {
            target: value.to_owned(),
            reason: format!("{label} target is required"),
        })
    } else {
        Ok(value)
    }
}

fn agent_name_for_contract(target_kind: &DomainTargetKind) -> String {
    match target_kind {
        DomainTargetKind::Agent(agent) => agent.clone(),
        DomainTargetKind::Role(role) => format!("role:{role}"),
        DomainTargetKind::Team(team) => format!("team:{team}"),
        DomainTargetKind::All => "all".to_owned(),
    }
}
