use crate::domain::model::{DomainEndpoint, DomainRegistrySnapshot, RegisteredDomain};
use crate::domain::resolve;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainRegistryError {
    Validation { reason: String },
    UnknownDomain { domain: String },
    AmbiguousBareTarget { target: String },
    InvalidTarget { target: String, reason: String },
}

pub type DomainRegistryResult<T> = Result<T, DomainRegistryError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainRegistry {
    local_domain_id: String,
    domains: BTreeMap<String, RegisteredDomain>,
    aliases: BTreeMap<String, String>,
}

impl DomainRegistry {
    pub fn new(local_domain: RegisteredDomain) -> DomainRegistryResult<Self> {
        if !local_domain.is_local {
            return validation_error("local domain must be marked local");
        }
        validate_domain_id(&local_domain.id)?;
        let mut registry = Self {
            local_domain_id: local_domain.id.clone(),
            domains: BTreeMap::new(),
            aliases: BTreeMap::new(),
        };
        registry.insert_domain(local_domain)?;
        Ok(registry)
    }

    pub fn register_remote(&mut self, remote: RegisteredDomain) -> DomainRegistryResult<()> {
        if remote.is_local {
            return validation_error("remote domain must not be marked local");
        }
        validate_domain_id(&remote.id)?;
        self.insert_domain(remote)
    }

    pub fn resolve_target(
        &self,
        raw_target: impl Into<String>,
    ) -> DomainRegistryResult<crate::domain::model::ResolvedDomainTarget> {
        resolve::resolve_target(self, raw_target.into())
    }

    pub fn snapshot(&self) -> DomainRegistrySnapshot {
        let local = self.domains.get(&self.local_domain_id);
        let aliases = local.map_or_else(Vec::new, |domain| domain.aliases.clone());
        let remote_domain_ids = self
            .domains
            .values()
            .filter(|domain| !domain.is_local)
            .map(|domain| domain.id.clone())
            .collect();
        DomainRegistrySnapshot {
            local_domain_id: self.local_domain_id.clone(),
            aliases,
            remote_domain_ids,
            endpoint_count: self.domains.len(),
            token_redaction_status: "redacted".to_owned(),
        }
    }

    pub(crate) fn local_domain_id(&self) -> &str {
        &self.local_domain_id
    }

    pub(crate) fn domain_by_id_or_alias(&self, value: &str) -> Option<&RegisteredDomain> {
        if let Some(domain) = self.domains.get(value) {
            return Some(domain);
        }
        self.aliases
            .get(value)
            .and_then(|domain_id| self.domains.get(domain_id))
    }

    fn insert_domain(&mut self, domain: RegisteredDomain) -> DomainRegistryResult<()> {
        let mut local_aliases = BTreeSet::new();
        for alias in &domain.aliases {
            validate_domain_id(alias)?;
            if !local_aliases.insert(alias.clone()) {
                return validation_error(format!("duplicate alias on domain {}", domain.id));
            }
        }
        if self.domains.contains_key(&domain.id) || self.aliases.contains_key(&domain.id) {
            return validation_error(format!("duplicate domain id {}", domain.id));
        }
        for alias in &domain.aliases {
            if self.domains.contains_key(alias) || self.aliases.contains_key(alias) {
                return validation_error(format!("duplicate domain alias {alias}"));
            }
        }
        for alias in &domain.aliases {
            self.aliases.insert(alias.clone(), domain.id.clone());
        }
        self.domains.insert(domain.id.clone(), domain);
        Ok(())
    }
}

pub fn registered_domain(
    id: impl Into<String>,
    aliases: Vec<String>,
    endpoint: DomainEndpoint,
    is_local: bool,
) -> RegisteredDomain {
    RegisteredDomain {
        id: id.into(),
        aliases,
        endpoint,
        is_local,
    }
}

fn validate_domain_id(id: &str) -> DomainRegistryResult<()> {
    if id.is_empty() {
        return validation_error("domain id is required");
    }
    if id
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
    {
        Ok(())
    } else {
        validation_error(format!("invalid domain id {id}"))
    }
}

fn validation_error<T>(reason: impl Into<String>) -> DomainRegistryResult<T> {
    Err(DomainRegistryError::Validation {
        reason: reason.into(),
    })
}
