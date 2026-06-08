use super::*;

fn registry() -> DomainRegistry {
    let local = registered_domain(
        "local",
        vec!["agentteam-main".to_owned()],
        DomainEndpoint::new("127.0.0.1", 43210, ""),
        true,
    );
    let mut registry = DomainRegistry::new(local).unwrap();
    registry
        .register_remote(registered_domain(
            "review-daemon",
            vec!["review".to_owned()],
            DomainEndpoint::new("127.0.0.1", 43211, "secret"),
            false,
        ))
        .unwrap();
    registry
}

#[test]
fn resolves_local_agent_target() {
    let resolved = registry().resolve_target("Kevin@local").unwrap();
    assert_eq!(resolved.domain_id, "local");
    assert_eq!(resolved.route_kind, DomainRouteKind::Local);
    assert_eq!(
        resolved.target_kind,
        DomainTargetKind::Agent("Kevin".to_owned())
    );
}

#[test]
fn resolves_remote_alias_target() {
    let resolved = registry().resolve_target("Alice@review").unwrap();
    assert_eq!(resolved.domain_id, "review-daemon");
    assert_eq!(resolved.route_kind, DomainRouteKind::Remote);
    assert_eq!(resolved.endpoint.port, 43211);
}

#[test]
fn resolves_role_team_and_all_targets() {
    assert_eq!(
        registry()
            .resolve_target("role:builder@review")
            .unwrap()
            .target_kind,
        DomainTargetKind::Role("builder".to_owned())
    );
    assert_eq!(
        registry()
            .resolve_target("team:default@local")
            .unwrap()
            .target_kind,
        DomainTargetKind::Team("default".to_owned())
    );
    assert_eq!(
        registry().resolve_target("all@local").unwrap().target_kind,
        DomainTargetKind::All
    );
}

#[test]
fn bare_target_resolves_only_to_local() {
    let resolved = registry().resolve_target("Kevin").unwrap();
    assert_eq!(resolved.domain_id, "local");
    assert_eq!(resolved.route_kind, DomainRouteKind::Local);
}

#[test]
fn unknown_remote_domain_does_not_fallback_to_local() {
    let error = registry().resolve_target("Alice@missing").unwrap_err();
    assert_eq!(
        error,
        DomainRegistryError::UnknownDomain {
            domain: "missing".to_owned()
        }
    );
}

#[test]
fn duplicate_domain_alias_fails() {
    let mut registry = registry();
    let error = registry
        .register_remote(registered_domain(
            "other",
            vec!["review".to_owned()],
            DomainEndpoint::new("127.0.0.1", 43212, ""),
            false,
        ))
        .unwrap_err();
    assert!(matches!(error, DomainRegistryError::Validation { .. }));
}

#[test]
fn snapshot_does_not_expose_tokens() {
    let snapshot = registry().snapshot();
    assert_eq!(snapshot.local_domain_id, "local");
    assert_eq!(snapshot.remote_domain_ids, vec!["review-daemon"]);
    assert_eq!(snapshot.token_redaction_status, "redacted");
    assert!(!format!("{snapshot:?}").contains("secret"));
}
