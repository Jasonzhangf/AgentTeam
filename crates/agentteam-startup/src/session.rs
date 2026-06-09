use agentteam_control::{
    AgentControlCenter, ControlAgentSessionBinding, ControlSendInput, ControlSessionInput,
};
use agentteam_resource::{ResourceAcquireInput, ResourceRegistry};
use agentteam_tmux::{launch_managed_session, session_exists, TmuxLaunchInput};

use crate::context::AgentStartupContext;
use crate::env::build_agent_env;
use crate::error::{control_error, resource_error, tmux_error, StartupError, StartupResult};
use crate::paths::runtime_event_log_path;
use crate::FEATURE_ID;

pub(crate) struct AgentSessionPlan<'a> {
    pub context: &'a AgentStartupContext<'a>,
    pub session_name: &'a str,
    pub bootstrap_prompt: String,
}

pub(crate) struct AgentSessionOutcome {
    pub launch_status: String,
    pub session_lifecycle: String,
    pub bootstrap_prompt_status: String,
    pub agent_session_status: String,
    pub control_handoff_status: String,
    pub agent_session_id: String,
    pub seed_turn_id: Option<String>,
    pub tui_resume_command: String,
    pub tui_resume_arg_count: usize,
    pub resource_lease_id: String,
    pub resource_snapshot_id: String,
    pub tmux_session_observed: bool,
}

pub(crate) fn ensure_agent_session(
    plan: AgentSessionPlan<'_>,
) -> StartupResult<AgentSessionOutcome> {
    let mut resource_registry = ResourceRegistry::new();
    if session_exists(plan.session_name).map_err(tmux_error)? {
        let snapshot = resource_registry.snapshot();
        return Ok(AgentSessionOutcome {
            launch_status: "already_running".to_owned(),
            session_lifecycle: "existing".to_owned(),
            bootstrap_prompt_status: "skipped_existing_session".to_owned(),
            agent_session_status: "not_reseeded_existing_tui".to_owned(),
            control_handoff_status: "attach_tui_existing".to_owned(),
            agent_session_id: String::new(),
            seed_turn_id: None,
            tui_resume_command: plan.context.member.command.clone(),
            tui_resume_arg_count: plan.context.member.args.len(),
            resource_lease_id: String::new(),
            resource_snapshot_id: snapshot.snapshot_id,
            tmux_session_observed: true,
        });
    }
    let lease = resource_registry
        .acquire(
            runtime_event_log_path(plan.context.runtime_home),
            ResourceAcquireInput {
                owner_module: FEATURE_ID.to_owned(),
                owner_entity_id: format!(
                    "{}@{}",
                    plan.context.member.name, plan.context.normalized.local_domain_id
                ),
                resource_class: "tmux_session".to_owned(),
                scope: plan.context.normalized.project_slug.clone(),
                memory_bytes_estimate: 0,
                handle_count: 1,
            },
        )
        .map_err(resource_error)?;
    launch_new_agent_session(&plan, &lease.lease_id, &mut resource_registry)
}

fn launch_new_agent_session(
    plan: &AgentSessionPlan<'_>,
    lease_id: &str,
    resource_registry: &mut ResourceRegistry,
) -> StartupResult<AgentSessionOutcome> {
    let binding = seed_codex_agent_session(plan)?;
    let resume_args = build_resume_args(&plan.context.member.args, &binding.agent_session_id);
    if let Err(error) = stop_seed_bridge(plan) {
        return release_after_launch_failure(
            plan.context.runtime_home,
            resource_registry,
            lease_id,
            error.reason(),
        );
    }
    let launch = launch_managed_session(TmuxLaunchInput {
        session_name: plan.session_name.to_owned(),
        cwd: plan.context.cwd.to_owned(),
        command: plan.context.member.command.clone(),
        args: resume_args.clone(),
        env: build_agent_env(plan.context, plan.session_name),
    });
    if let Err(error) = launch {
        return release_after_launch_failure(
            plan.context.runtime_home,
            resource_registry,
            lease_id,
            error.reason(),
        );
    }
    let snapshot = resource_registry.snapshot();
    Ok(AgentSessionOutcome {
        launch_status: "launched".to_owned(),
        session_lifecycle: "created".to_owned(),
        bootstrap_prompt_status: "seeded_before_tui_resume".to_owned(),
        agent_session_status: binding.state,
        control_handoff_status: "attach_tui_resumed_agent_session".to_owned(),
        agent_session_id: binding.agent_session_id,
        seed_turn_id: binding.seed_turn_id,
        tui_resume_command: plan.context.member.command.clone(),
        tui_resume_arg_count: resume_args.len(),
        resource_lease_id: lease_id.to_owned(),
        resource_snapshot_id: snapshot.snapshot_id,
        tmux_session_observed: true,
    })
}

fn seed_codex_agent_session(
    plan: &AgentSessionPlan<'_>,
) -> StartupResult<ControlAgentSessionBinding> {
    let control = AgentControlCenter::new();
    control
        .seed_agent_session(ControlSendInput::new(
            control_session_input(plan),
            plan.bootstrap_prompt.clone(),
        ))
        .map_err(control_error)
}

fn stop_seed_bridge(plan: &AgentSessionPlan<'_>) -> StartupResult<()> {
    AgentControlCenter::new()
        .headless_stop(control_session_input(plan))
        .map(|_| ())
        .map_err(control_error)
}

fn control_session_input(plan: &AgentSessionPlan<'_>) -> ControlSessionInput {
    ControlSessionInput::new(
        plan.context.member.name.clone(),
        plan.context.team_id.to_owned(),
        plan.session_name.to_owned(),
    )
    .with_scope(
        plan.context.cwd.to_owned(),
        plan.context.normalized.project_slug.clone(),
    )
}

fn build_resume_args(base_args: &[String], agent_session_id: &str) -> Vec<String> {
    let mut args = base_args.to_vec();
    args.push("resume".to_owned());
    args.push(agent_session_id.to_owned());
    args
}

fn release_after_launch_failure<T>(
    runtime_home: &str,
    resource_registry: &mut ResourceRegistry,
    lease_id: &str,
    primary: String,
) -> StartupResult<T> {
    let cleanup = resource_registry
        .release(runtime_event_log_path(runtime_home), lease_id, FEATURE_ID)
        .map_err(resource_error);
    match cleanup {
        Ok(_) => Err(StartupError::Launch { reason: primary }),
        Err(cleanup_error) => Err(StartupError::Cleanup {
            primary,
            cleanup: cleanup_error.reason(),
        }),
    }
}
