#![cfg(test)]

use std::collections::BTreeMap;

use agentteam_config::{MemberConfig, NormalizedConfig, TeamConfig};

use crate::env::build_agent_env;
use crate::paths::build_session_name;
use crate::prompt::{build_root_manager_bootstrap_prompt, build_worker_bootstrap_prompt};
use crate::select::worker_names;

fn normalized() -> NormalizedConfig {
    NormalizedConfig {
        path: "docs/config/config.toml.example".to_owned(),
        project_slug: "agentteam".to_owned(),
        project_root: "/repo/agentteam".to_owned(),
        runtime_home: "~/.agentteam/runtime/agentteam".to_owned(),
        local_domain_id: "local".to_owned(),
        team_count: 1,
        member_count: 3,
        zterm_endpoint: "127.0.0.1:3333".to_owned(),
        remote_domain_count: 0,
    }
}

fn member(name: &str, team_role: &str, role: &str) -> MemberConfig {
    MemberConfig {
        name: name.to_owned(),
        team_role: team_role.to_owned(),
        role: role.to_owned(),
        cwd: "/repo/agentteam".to_owned(),
        command: "codex".to_owned(),
        args: Vec::new(),
        env: BTreeMap::new(),
    }
}

fn team() -> TeamConfig {
    TeamConfig {
        id: "default".to_owned(),
        category: "development".to_owned(),
        agent_count: 3,
        members: vec![
            member("Kevin", "manager", "manager"),
            member("Alice", "worker", "builder"),
            member("Bob", "worker", "reviewer"),
        ],
    }
}

#[test]
fn root_manager_prompt_teaches_identity_skill_and_worker_start() {
    let normalized = normalized();
    let team = team();
    let prompt = build_root_manager_bootstrap_prompt(
        &normalized,
        "/runtime",
        "default",
        "/repo/agentteam",
        &team.members[0],
        &team,
        Some("docs/config/config.toml.example"),
    );

    assert!(prompt.contains("You are Kevin"));
    assert!(prompt.contains(".agents/skills/agentteam/SKILL.md"));
    assert!(prompt.contains("agentteam start worker --agent <worker>"));
    assert!(prompt.contains("Alice, Bob"));
    assert!(prompt.contains("Do not call tmux directly"));
}

#[test]
fn worker_prompt_teaches_role_and_ready_loop() {
    let normalized = normalized();
    let alice = member("Alice", "worker", "builder");
    let prompt = build_worker_bootstrap_prompt(
        &normalized,
        "/runtime",
        "default",
        "/repo/agentteam",
        &alice,
        Some("docs/config/config.toml.example"),
    );

    assert!(prompt.contains("You are Alice"));
    assert!(prompt.contains("team_role=worker"));
    assert!(prompt.contains("agentteam ready report"));
    assert!(prompt.contains("agentteam task claim"));
    assert!(prompt.contains("agentteam task done"));
}

#[test]
fn worker_names_includes_only_worker_roles() {
    assert_eq!(
        worker_names(&team()),
        vec!["Alice".to_owned(), "Bob".to_owned()]
    );
}

#[test]
fn agent_env_contains_identity_and_scope() {
    let normalized = normalized();
    let alice = member("Alice", "worker", "builder");
    let env = build_agent_env(
        &normalized,
        "/runtime",
        "default",
        "/repo/agentteam",
        "TA_local_agentteam_Alice",
        &alice,
    );

    assert_eq!(env.get("AGENTTEAM_NAME"), Some(&"Alice".to_owned()));
    assert_eq!(env.get("AGENTTEAM_TEAM_ROLE"), Some(&"worker".to_owned()));
    assert_eq!(
        env.get("AGENTTEAM_SESSION_NAME"),
        Some(&"TA_local_agentteam_Alice".to_owned())
    );
    assert_eq!(env.get("AGENTTEAM_SKILL"), Some(&"agentteam".to_owned()));
}

#[test]
fn session_name_uses_domain_project_and_agent() {
    assert_eq!(
        build_session_name("local", "agentteam", "Kevin"),
        "TA_local_agentteam_Kevin"
    );
}
