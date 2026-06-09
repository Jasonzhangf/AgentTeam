use agentteam_control::{
    AgentControlCenter, ControlRetryInput, ControlSendInput, ControlSessionInput,
};

use crate::local_projection::control_result;
use crate::local_projection::{LocalCommandError, LocalCommandResult};

pub fn execute_control(
    action: String,
    agent_name: String,
    team_id: String,
    session_name: String,
    input: Option<String>,
    task_id: Option<String>,
    error_fact_id: Option<String>,
) -> Result<LocalCommandResult, LocalCommandError> {
    let control = AgentControlCenter::new();
    let projection = match action.as_str() {
        "attach" => control
            .attach_tui(agentteam_contracts::control::AgentCtlReq01ModeIntent::new(
                agent_name.clone(),
                team_id.clone(),
                agentteam_contracts::control::AgentControlMode::AttachTui,
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "headless" => control
            .headless(agentteam_contracts::control::AgentCtlReq01ModeIntent::new(
                agent_name.clone(),
                team_id.clone(),
                agentteam_contracts::control::AgentControlMode::Headless,
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "headless-run" => {
            let input = input.ok_or_else(|| LocalCommandError::Control {
                reason: "--input is required for control headless-run".to_owned(),
            })?;
            control
                .headless_run(ControlSendInput::new(
                    ControlSessionInput::new(
                        agent_name.clone(),
                        team_id.clone(),
                        session_name.clone(),
                    ),
                    input,
                ))
                .map_err(control_error)?
        }
        "headless-status" => control
            .headless_status(ControlSessionInput::new(
                agent_name.clone(),
                team_id.clone(),
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "headless-interrupt" => control
            .headless_interrupt(ControlSessionInput::new(
                agent_name.clone(),
                team_id.clone(),
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "headless-stop" => control
            .headless_stop(ControlSessionInput::new(
                agent_name.clone(),
                team_id.clone(),
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "send" => {
            let input = input.ok_or_else(|| LocalCommandError::Control {
                reason: "--input is required for control send".to_owned(),
            })?;
            control
                .send_input(ControlSendInput::new(
                    ControlSessionInput::new(
                        agent_name.clone(),
                        team_id.clone(),
                        session_name.clone(),
                    ),
                    input,
                ))
                .map_err(control_error)?
        }
        "observe" => control
            .observe_output(ControlSessionInput::new(
                agent_name.clone(),
                team_id.clone(),
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "pause" => control
            .pause(ControlSessionInput::new(
                agent_name.clone(),
                team_id.clone(),
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "stop" => control
            .stop(ControlSessionInput::new(
                agent_name.clone(),
                team_id.clone(),
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "wait" => control
            .wait(ControlSessionInput::new(
                agent_name.clone(),
                team_id.clone(),
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "status" => control
            .status(ControlSessionInput::new(
                agent_name.clone(),
                team_id.clone(),
                session_name.clone(),
            ))
            .map_err(control_error)?,
        "retry" => {
            let task_id = task_id.ok_or_else(|| LocalCommandError::Control {
                reason: "--task is required for control retry".to_owned(),
            })?;
            let error_fact_id = error_fact_id.ok_or_else(|| LocalCommandError::Control {
                reason: "--error-fact is required for control retry".to_owned(),
            })?;
            control
                .retry_dispatch(ControlRetryInput::new(
                    ControlSessionInput::new(
                        agent_name.clone(),
                        team_id.clone(),
                        session_name.clone(),
                    ),
                    task_id,
                    error_fact_id,
                ))
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
