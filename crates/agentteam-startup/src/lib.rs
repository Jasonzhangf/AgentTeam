mod config;
mod context;
mod env;
mod error;
mod model;
mod paths;
mod prompt;
mod select;
mod session;
mod skill;
mod tests;

pub use error::{StartupError, StartupResult};
pub use model::{
    StartupBootstrapInput, StartupBootstrapResult, StartupWorkerInput, StartupWorkerResult,
};

use config::load_validated_config;
use context::AgentStartupContext;
use paths::{build_session_name, expand_home_path, resolve_cwd, session_dir};
use prompt::{build_root_manager_bootstrap_prompt, build_worker_bootstrap_prompt};
use select::{select_root_manager, select_team, select_worker, worker_names};
use session::{ensure_agent_session, AgentSessionPlan};
use skill::install_agentteam_skill;

pub const FEATURE_ID: &str = "startup.session";

pub fn start_bootstrap(input: StartupBootstrapInput) -> StartupResult<StartupBootstrapResult> {
    let config_path = input.config_path;
    let validated = load_validated_config(config_path.clone())?;
    let normalized =
        agentteam_config::normalize_config(validated.clone()).map_err(error::config_error)?;
    let runtime_home = expand_home_path(&normalized.runtime_home)?;
    let cwd = resolve_cwd(input.cwd)?;
    let skill_install = install_agentteam_skill(&cwd)?;
    let team = select_team(&validated, input.team_id.as_deref())?;
    let manager = select_root_manager(team)?;
    let session_dir_path = session_dir(&normalized.project_slug)?;
    let session_name = build_session_name(
        &normalized.local_domain_id,
        &normalized.project_slug,
        &manager.name,
    );
    let context = AgentStartupContext {
        normalized: &normalized,
        runtime_home: &runtime_home,
        team_id: &team.id,
        cwd: &cwd,
        member: manager,
        skill_path: &skill_install.skill_path,
        cli_path: &skill_install.cli_path,
        config_path: config_path.as_deref(),
    };
    let outcome = ensure_agent_session(AgentSessionPlan {
        context: &context,
        session_name: &session_name,
        bootstrap_prompt: build_root_manager_bootstrap_prompt(&context, team),
    })?;
    Ok(StartupBootstrapResult {
        project_slug: normalized.project_slug,
        project_root: normalized.project_root,
        runtime_home,
        cwd,
        team_id: team.id.clone(),
        manager_name: manager.name.clone(),
        manager_role: manager.role.clone(),
        manager_session_name: session_name,
        manager_agent_session_id: outcome.agent_session_id,
        manager_seed_turn_id: outcome.seed_turn_id,
        session_dir: session_dir_path,
        planned_worker_count: worker_names(team).len(),
        worker_names: worker_names(team),
        launch_status: outcome.launch_status,
        session_lifecycle: outcome.session_lifecycle,
        bootstrap_prompt_status: outcome.bootstrap_prompt_status,
        skill_install_status: skill_install.status,
        skill_path: skill_install.skill_path,
        cli_path: skill_install.cli_path,
        agent_session_status: outcome.agent_session_status,
        control_handoff_status: outcome.control_handoff_status,
        tui_resume_command: outcome.tui_resume_command,
        tui_resume_arg_count: outcome.tui_resume_arg_count,
        resource_lease_id: outcome.resource_lease_id,
        resource_snapshot_id: outcome.resource_snapshot_id,
        tmux_session_observed: outcome.tmux_session_observed,
    })
}

pub fn start_worker(input: StartupWorkerInput) -> StartupResult<StartupWorkerResult> {
    let config_path = input.config_path;
    let validated = load_validated_config(config_path.clone())?;
    let normalized =
        agentteam_config::normalize_config(validated.clone()).map_err(error::config_error)?;
    let runtime_home = expand_home_path(&normalized.runtime_home)?;
    let team = select_team(&validated, input.team_id.as_deref())?;
    let worker = select_worker(team, &input.agent_name)?;
    let cwd = resolve_cwd(input.cwd.or_else(|| Some(worker.cwd.clone())))?;
    let skill_install = install_agentteam_skill(&cwd)?;
    let session_dir_path = session_dir(&normalized.project_slug)?;
    let session_name = build_session_name(
        &normalized.local_domain_id,
        &normalized.project_slug,
        &worker.name,
    );
    let context = AgentStartupContext {
        normalized: &normalized,
        runtime_home: &runtime_home,
        team_id: &team.id,
        cwd: &cwd,
        member: worker,
        skill_path: &skill_install.skill_path,
        cli_path: &skill_install.cli_path,
        config_path: config_path.as_deref(),
    };
    let outcome = ensure_agent_session(AgentSessionPlan {
        context: &context,
        session_name: &session_name,
        bootstrap_prompt: build_worker_bootstrap_prompt(&context),
    })?;
    Ok(StartupWorkerResult {
        project_slug: normalized.project_slug,
        project_root: normalized.project_root,
        runtime_home,
        cwd,
        team_id: team.id.clone(),
        agent_name: worker.name.clone(),
        agent_role: worker.role.clone(),
        team_role: worker.team_role.clone(),
        session_name,
        agent_session_id: outcome.agent_session_id,
        seed_turn_id: outcome.seed_turn_id,
        session_dir: session_dir_path,
        launch_status: outcome.launch_status,
        session_lifecycle: outcome.session_lifecycle,
        bootstrap_prompt_status: outcome.bootstrap_prompt_status,
        skill_install_status: skill_install.status,
        skill_path: skill_install.skill_path,
        cli_path: skill_install.cli_path,
        agent_session_status: outcome.agent_session_status,
        control_handoff_status: outcome.control_handoff_status,
        tui_resume_command: outcome.tui_resume_command,
        tui_resume_arg_count: outcome.tui_resume_arg_count,
        resource_lease_id: outcome.resource_lease_id,
        resource_snapshot_id: outcome.resource_snapshot_id,
        tmux_session_observed: outcome.tmux_session_observed,
    })
}
