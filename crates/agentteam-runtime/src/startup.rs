use agentteam_startup::{
    start_bootstrap, start_worker, StartupBootstrapInput, StartupError, StartupWorkerInput,
};

use crate::local_projection::{LocalCommandError, LocalCommandResult};
use crate::local_projection::{StartupStartResult, StartupWorkerResult};

pub(crate) fn execute_startup(
    cwd: Option<String>,
    config_path: Option<String>,
    team_id: Option<String>,
) -> Result<LocalCommandResult, LocalCommandError> {
    let bootstrap = start_bootstrap(StartupBootstrapInput {
        cwd,
        config_path,
        team_id,
    })
    .map_err(startup_error)?;
    Ok(LocalCommandResult::StartupStart {
        bootstrap: StartupStartResult {
            project_slug: bootstrap.project_slug,
            project_root: bootstrap.project_root,
            runtime_home: bootstrap.runtime_home,
            cwd: bootstrap.cwd,
            team_id: bootstrap.team_id,
            manager_name: bootstrap.manager_name,
            manager_role: bootstrap.manager_role,
            manager_session_name: bootstrap.manager_session_name,
            manager_agent_session_id: bootstrap.manager_agent_session_id,
            manager_seed_turn_id: bootstrap.manager_seed_turn_id,
            session_dir: bootstrap.session_dir,
            planned_worker_count: bootstrap.planned_worker_count,
            worker_names: bootstrap.worker_names,
            launch_status: bootstrap.launch_status,
            session_lifecycle: bootstrap.session_lifecycle,
            bootstrap_prompt_status: bootstrap.bootstrap_prompt_status,
            agent_session_status: bootstrap.agent_session_status,
            control_handoff_status: bootstrap.control_handoff_status,
            tui_resume_command: bootstrap.tui_resume_command,
            tui_resume_arg_count: bootstrap.tui_resume_arg_count,
            resource_lease_id: bootstrap.resource_lease_id,
            resource_snapshot_id: bootstrap.resource_snapshot_id,
            tmux_session_observed: bootstrap.tmux_session_observed,
        },
    })
}

pub(crate) fn execute_startup_worker(
    agent_name: String,
    cwd: Option<String>,
    config_path: Option<String>,
    team_id: Option<String>,
) -> Result<LocalCommandResult, LocalCommandError> {
    let worker = start_worker(StartupWorkerInput {
        agent_name,
        cwd,
        config_path,
        team_id,
    })
    .map_err(startup_error)?;
    Ok(LocalCommandResult::StartupWorker {
        worker: StartupWorkerResult {
            project_slug: worker.project_slug,
            project_root: worker.project_root,
            runtime_home: worker.runtime_home,
            cwd: worker.cwd,
            team_id: worker.team_id,
            agent_name: worker.agent_name,
            agent_role: worker.agent_role,
            team_role: worker.team_role,
            session_name: worker.session_name,
            agent_session_id: worker.agent_session_id,
            seed_turn_id: worker.seed_turn_id,
            session_dir: worker.session_dir,
            launch_status: worker.launch_status,
            session_lifecycle: worker.session_lifecycle,
            bootstrap_prompt_status: worker.bootstrap_prompt_status,
            agent_session_status: worker.agent_session_status,
            control_handoff_status: worker.control_handoff_status,
            tui_resume_command: worker.tui_resume_command,
            tui_resume_arg_count: worker.tui_resume_arg_count,
            resource_lease_id: worker.resource_lease_id,
            resource_snapshot_id: worker.resource_snapshot_id,
            tmux_session_observed: worker.tmux_session_observed,
        },
    })
}

pub(crate) fn startup_error(error: StartupError) -> LocalCommandError {
    LocalCommandError::Startup {
        reason: error.reason(),
    }
}
