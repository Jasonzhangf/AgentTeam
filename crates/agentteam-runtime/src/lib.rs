pub mod domain;
pub mod local;
#[cfg(test)]
mod local_tests;

pub const TEAM_ORCHESTRATION_FEATURE_ID: &str = "team.orchestration";
pub const TASK_ENGINE_FEATURE_ID: &str = "task.engine";
pub const AGENT_NAMING_POOL_FEATURE_ID: &str = "agent.naming_pool";
pub const DOMAIN_REGISTRY_FEATURE_ID: &str = "domain.registry";
