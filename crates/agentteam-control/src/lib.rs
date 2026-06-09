mod error;
mod headless_process;
mod headless_protocol;
mod model;
#[cfg(test)]
mod tests;

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use agentteam_contracts::control::{
    AgentControlAction, AgentControlMode, AgentCtlReq01ModeIntent, AgentCtlResp05ControlProjection,
};
use agentteam_tmux::{
    capture_session, interrupt_session, send_input, session_exists, stop_session,
};

pub use error::{tmux_error, ControlError, ControlResult};
pub use model::{
    ControlAgentSessionBinding, ControlRetryInput, ControlSendInput, ControlSessionInput,
    ControlSnapshot,
};

pub const FEATURE_ID: &str = "agent.control_center";

pub struct AgentControlCenter {
    receipt_seq: AtomicU64,
}

impl Default for AgentControlCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentControlCenter {
    pub fn new() -> Self {
        Self {
            receipt_seq: AtomicU64::new(1),
        }
    }

    pub fn attach_tui(
        &self,
        input: AgentCtlReq01ModeIntent,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        self.run_tmux_control(input, AgentControlAction::Attach, |session_name| {
            if session_exists(session_name).map_err(tmux_error)? {
                let captured = capture_session(session_name).map_err(tmux_error)?;
                Ok((status_from_capture(&captured).to_owned(), captured))
            } else {
                Err(ControlError::Validation {
                    reason: format!("tmux session {session_name} was not found"),
                })
            }
        })
    }

    pub fn headless(
        &self,
        input: AgentCtlReq01ModeIntent,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        let resolved = input.clone().resolve_mode();
        if resolved.mode != AgentControlMode::Headless {
            return Err(ControlError::Validation {
                reason: "headless requires headless mode".to_owned(),
            });
        }
        let session = ControlSessionInput::new(
            resolved.agent_name.clone(),
            resolved.team_id.clone(),
            resolved.session_name.clone(),
        );
        let response = headless_process::start_session(&session)?;
        let action = resolved.bind_session().apply_action(
            AgentControlAction::Headless,
            response.state,
            response.details,
        );
        Ok(action.project(self.next_receipt_id("headless")))
    }

    pub fn seed_agent_session(
        &self,
        input: ControlSendInput,
    ) -> ControlResult<ControlAgentSessionBinding> {
        if input.input.trim().is_empty() {
            return Err(ControlError::Validation {
                reason: "agent session seed input is required".to_owned(),
            });
        }
        let response = headless_process::seed_agent_session(&input.session, &input.input)?;
        let agent_session_id = response
            .thread_id
            .ok_or_else(|| ControlError::HeadlessBridge {
                reason: "Codex SDK seed did not return thread_id".to_owned(),
            })?;
        let project_slug = input
            .session
            .project_slug
            .clone()
            .unwrap_or(response.project_slug);
        Ok(ControlAgentSessionBinding::new(
            &input.session,
            project_slug,
            agent_session_id,
            response.turn_id,
            response.state,
            response.details,
        ))
    }

    pub fn headless_run(
        &self,
        input: ControlSendInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        if input.input.trim().is_empty() {
            return Err(ControlError::Validation {
                reason: "headless run input is required".to_owned(),
            });
        }
        let response = headless_process::run_turn(&input.session, &input.input)?;
        Ok(
            self.project_headless_response(
                input.session,
                AgentControlAction::HeadlessRun,
                response,
            ),
        )
    }

    pub fn headless_status(
        &self,
        input: ControlSessionInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        let response = headless_process::session_status(&input)?;
        Ok(self.project_headless_response(input, AgentControlAction::HeadlessStatus, response))
    }

    pub fn headless_interrupt(
        &self,
        input: ControlSessionInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        let response = headless_process::interrupt_turn(&input)?;
        Ok(self.project_headless_response(input, AgentControlAction::HeadlessInterrupt, response))
    }

    pub fn headless_stop(
        &self,
        input: ControlSessionInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        let response = headless_process::stop_session(&input)?;
        Ok(self.project_headless_response(input, AgentControlAction::HeadlessStop, response))
    }

    pub fn send_input(
        &self,
        input: ControlSendInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        self.run_session_control(input.session, AgentControlAction::Send, |session_name| {
            send_input(session_name, &input.input).map_err(tmux_error)?;
            Ok(("busy".to_owned(), "input delivered".to_owned()))
        })
    }

    pub fn observe_output(
        &self,
        input: ControlSessionInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        self.run_session_control(input, AgentControlAction::Observe, |session_name| {
            let captured = capture_session(session_name).map_err(tmux_error)?;
            Ok((status_from_capture(&captured).to_owned(), captured))
        })
    }

    pub fn pause(
        &self,
        input: ControlSessionInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        self.run_session_control(input, AgentControlAction::Pause, |session_name| {
            interrupt_session(session_name).map_err(tmux_error)?;
            Ok(("busy".to_owned(), "interrupt requested".to_owned()))
        })
    }

    pub fn stop(
        &self,
        input: ControlSessionInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        self.run_session_control(input, AgentControlAction::Stop, |session_name| {
            stop_session(session_name).map_err(tmux_error)?;
            Ok(("offline".to_owned(), "session stopped".to_owned()))
        })
    }

    pub fn wait(
        &self,
        input: ControlSessionInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        self.run_session_control(input, AgentControlAction::Wait, |session_name| {
            let mut last = String::new();
            for _ in 0..20 {
                let captured = capture_session(session_name).map_err(tmux_error)?;
                let state = status_from_capture(&captured);
                if state != "busy" {
                    return Ok((state.to_owned(), captured));
                }
                last = captured;
                thread::sleep(Duration::from_millis(50));
            }
            Ok(("busy".to_owned(), last))
        })
    }

    pub fn retry_dispatch(
        &self,
        input: ControlRetryInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        if input.task_id.trim().is_empty() || input.error_fact_id.trim().is_empty() {
            return Err(ControlError::Validation {
                reason: "task_id and error_fact_id are required".to_owned(),
            });
        }
        let intent = AgentCtlReq01ModeIntent::new(
            input.session.agent_name,
            input.session.team_id,
            AgentControlMode::AttachTui,
            input.session.session_name,
        );
        let resolved = intent.resolve_mode();
        let binding = resolved.bind_session();
        let action = binding.apply_action(
            AgentControlAction::Retry,
            "busy",
            format!(
                "retry requested for task {} via {}",
                input.task_id, input.error_fact_id
            ),
        );
        Ok(action.project(self.next_receipt_id("retry")))
    }

    pub fn status(
        &self,
        input: ControlSessionInput,
    ) -> ControlResult<AgentCtlResp05ControlProjection> {
        self.run_session_control(input, AgentControlAction::Status, |session_name| {
            let captured = capture_session(session_name).map_err(tmux_error)?;
            Ok((status_from_capture(&captured).to_owned(), captured))
        })
    }

    pub fn snapshot(&self, projection: &AgentCtlResp05ControlProjection) -> ControlSnapshot {
        ControlSnapshot::from_projection(projection)
    }

    pub fn help(&self, topic: &str) -> String {
        help_text(topic)
    }

    fn run_session_control<F>(
        &self,
        input: ControlSessionInput,
        action: AgentControlAction,
        control: F,
    ) -> ControlResult<AgentCtlResp05ControlProjection>
    where
        F: FnOnce(&str) -> ControlResult<(String, String)>,
    {
        let intent = AgentCtlReq01ModeIntent::new(
            input.agent_name,
            input.team_id,
            AgentControlMode::AttachTui,
            input.session_name,
        );
        self.run_tmux_control(intent, action, control)
    }

    fn run_tmux_control<F>(
        &self,
        input: AgentCtlReq01ModeIntent,
        action: AgentControlAction,
        control: F,
    ) -> ControlResult<AgentCtlResp05ControlProjection>
    where
        F: FnOnce(&str) -> ControlResult<(String, String)>,
    {
        let resolved = input.resolve_mode();
        let binding = resolved.bind_session();
        let action_label = control_action_label(action);
        let (state, details) = control(&binding.session_name)?;
        let action = binding.apply_action(action, state, details);
        Ok(action.project(self.next_receipt_id(action_label)))
    }

    fn project_headless_response(
        &self,
        input: ControlSessionInput,
        action: AgentControlAction,
        response: headless_protocol::HeadlessBridgeResponse,
    ) -> AgentCtlResp05ControlProjection {
        let details = response
            .final_response
            .filter(|response| !response.trim().is_empty())
            .unwrap_or(response.details);
        AgentCtlReq01ModeIntent::new(
            input.agent_name,
            input.team_id,
            AgentControlMode::Headless,
            input.session_name,
        )
        .resolve_mode()
        .bind_session()
        .apply_action(action, response.state, details)
        .project(self.next_receipt_id(control_action_label(action)))
    }

    fn next_receipt_id(&self, suffix: &str) -> String {
        let seq = self.receipt_seq.fetch_add(1, Ordering::Relaxed);
        format!("control-{suffix}-{seq:06}")
    }
}

fn status_from_capture(captured: &str) -> &'static str {
    let lower = captured.to_ascii_lowercase();
    if lower.contains("error")
        || lower.contains("panic")
        || lower.contains("exception")
        || lower.contains("traceback")
    {
        "error"
    } else if lower.contains("waiting")
        || lower.contains("busy")
        || lower.contains("running")
        || lower.contains("thinking")
        || lower.contains("processing")
    {
        "busy"
    } else {
        "idle"
    }
}

fn control_action_label(action: AgentControlAction) -> &'static str {
    match action {
        AgentControlAction::Attach => "attach",
        AgentControlAction::Send => "send",
        AgentControlAction::Observe => "observe",
        AgentControlAction::Pause => "pause",
        AgentControlAction::Stop => "stop",
        AgentControlAction::Wait => "wait",
        AgentControlAction::Retry => "retry",
        AgentControlAction::Status => "status",
        AgentControlAction::Headless => "headless",
        AgentControlAction::HeadlessRun => "headless-run",
        AgentControlAction::HeadlessStatus => "headless-status",
        AgentControlAction::HeadlessInterrupt => "headless-interrupt",
        AgentControlAction::HeadlessStop => "headless-stop",
    }
}

fn help_text(topic: &str) -> String {
    match topic {
        "attach" => "attach_tui binds one visible tmux session to one agent".to_owned(),
        "headless" => "headless uses a persistent Codex SDK bridge driven by AGENTTEAM_CODEX_SDK_SRC and AGENTTEAM_CODEX_BIN".to_owned(),
        "input" => "input is always a typed control envelope, not a raw hidden string".to_owned(),
        "output" => "output is observed through the control plane and captured evidence".to_owned(),
        "pause" => "pause sends an interrupt request to the active tmux session".to_owned(),
        "stop" => "stop closes the active tmux session explicitly".to_owned(),
        "wait" => "wait polls for a stable control status and never silently falls back".to_owned(),
        "retry" => "retry requests task re-dispatch after an explicit error fact".to_owned(),
        "status" => "status projects offline, idle, busy, or error from control evidence".to_owned(),
        "red-tests" => "red tests must block mode fallback, raw stdin bypass, and private state access".to_owned(),
        _ => "agentteam help agent-control [attach|headless|input|output|pause|stop|wait|retry|status|red-tests]".to_owned(),
    }
}
