use agentteam_control::{
    AgentControlCenter, ControlRetryInput, ControlSendInput, ControlSessionInput,
};

use crate::local_projection::control_result;
use crate::local_projection::{LocalCommandError, LocalCommandResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlExecutionInput {
    pub action: String,
    pub agent_name: String,
    pub team_id: String,
    pub session_name: String,
    pub cwd: Option<String>,
    pub project_slug: Option<String>,
    pub input: Option<String>,
    pub task_id: Option<String>,
    pub error_fact_id: Option<String>,
}

pub fn execute_control(
    input: ControlExecutionInput,
) -> Result<LocalCommandResult, LocalCommandError> {
    let control = AgentControlCenter::new();
    let session = || {
        let mut session = ControlSessionInput::new(
            input.agent_name.clone(),
            input.team_id.clone(),
            input.session_name.clone(),
        );
        if let (Some(cwd), Some(project_slug)) = (input.cwd.clone(), input.project_slug.clone()) {
            session = session.with_scope(cwd, project_slug);
        }
        session
    };
    let projection = match input.action.as_str() {
        "attach" => control
            .attach_tui(agentteam_contracts::control::AgentCtlReq01ModeIntent::new(
                input.agent_name.clone(),
                input.team_id.clone(),
                agentteam_contracts::control::AgentControlMode::AttachTui,
                input.session_name.clone(),
            ))
            .map_err(control_error)?,
        "headless" => control
            .headless(agentteam_contracts::control::AgentCtlReq01ModeIntent::new(
                input.agent_name.clone(),
                input.team_id.clone(),
                agentteam_contracts::control::AgentControlMode::Headless,
                input.session_name.clone(),
            ))
            .map_err(control_error)?,
        "headless-run" => {
            let prompt = input.input.ok_or_else(|| LocalCommandError::Control {
                reason: "--input is required for control headless-run".to_owned(),
            })?;
            control
                .headless_run(ControlSendInput::new(session(), prompt))
                .map_err(control_error)?
        }
        "headless-status" => control.headless_status(session()).map_err(control_error)?,
        "headless-interrupt" => control
            .headless_interrupt(session())
            .map_err(control_error)?,
        "headless-stop" => control.headless_stop(session()).map_err(control_error)?,
        "send" => {
            let prompt = input.input.ok_or_else(|| LocalCommandError::Control {
                reason: "--input is required for control send".to_owned(),
            })?;
            control
                .send_input(ControlSendInput::new(session(), prompt))
                .map_err(control_error)?
        }
        "observe" => control.observe_output(session()).map_err(control_error)?,
        "pause" => control.pause(session()).map_err(control_error)?,
        "stop" => control.stop(session()).map_err(control_error)?,
        "wait" => control.wait(session()).map_err(control_error)?,
        "status" => control.status(session()).map_err(control_error)?,
        "retry" => {
            let task_id = input.task_id.ok_or_else(|| LocalCommandError::Control {
                reason: "--task is required for control retry".to_owned(),
            })?;
            let error_fact_id = input
                .error_fact_id
                .ok_or_else(|| LocalCommandError::Control {
                    reason: "--error-fact is required for control retry".to_owned(),
                })?;
            control
                .retry_dispatch(ControlRetryInput::new(session(), task_id, error_fact_id))
                .map_err(control_error)?
        }
        other => {
            return Err(LocalCommandError::Control {
                reason: format!("unsupported control action {other}"),
            })
        }
    };
    let snapshot = control.snapshot(&projection);
    Ok(LocalCommandResult::Control {
        control: control_result(snapshot),
    })
}

fn control_error(error: agentteam_control::ControlError) -> LocalCommandError {
    LocalCommandError::Control {
        reason: error.reason(),
    }
}
