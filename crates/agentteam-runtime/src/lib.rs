mod control;
pub mod domain;
pub mod local;
pub mod local_projection;
mod local_startup_projection;
#[cfg(test)]
mod local_tests;
mod startup;
pub mod task;

pub const TEAM_ORCHESTRATION_FEATURE_ID: &str = "team.orchestration";
pub const TASK_ENGINE_FEATURE_ID: &str = "task.engine";
pub const AGENT_NAMING_POOL_FEATURE_ID: &str = "agent.naming_pool";
pub const DOMAIN_REGISTRY_FEATURE_ID: &str = "domain.registry";
