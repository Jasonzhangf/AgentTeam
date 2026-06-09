use crate::error::{TmuxAdapterError, TmuxAdapterResult};
use crate::process::run_tmux_command;

pub fn session_exists(session_name: &str) -> TmuxAdapterResult<bool> {
    validate_session_name(session_name)?;
    let args = vec![
        "has-session".to_owned(),
        "-t".to_owned(),
        session_name.to_owned(),
    ];
    match run_tmux_command("has-session", &args) {
        Ok(_) => Ok(true),
        Err(TmuxAdapterError::CommandFailed { .. }) => Ok(false),
        Err(error) => Err(error),
    }
}

pub fn capture_session(session_name: &str) -> TmuxAdapterResult<String> {
    validate_session_name(session_name)?;
    let args = vec![
        "capture-pane".to_owned(),
        "-p".to_owned(),
        "-J".to_owned(),
        "-t".to_owned(),
        session_name.to_owned(),
    ];
    run_tmux_command("capture-pane", &args).map(|output| output.stdout)
}

pub fn send_input(session_name: &str, input: &str) -> TmuxAdapterResult<()> {
    validate_session_name(session_name)?;
    if input.trim().is_empty() {
        return Err(TmuxAdapterError::Validation {
            reason: "input must not be empty".to_owned(),
        });
    }
    let literal_args = vec![
        "send-keys".to_owned(),
        "-t".to_owned(),
        session_name.to_owned(),
        "-l".to_owned(),
        input.to_owned(),
    ];
    run_tmux_command("send-keys literal", &literal_args)?;
    let enter_args = vec![
        "send-keys".to_owned(),
        "-t".to_owned(),
        session_name.to_owned(),
        "Enter".to_owned(),
    ];
    run_tmux_command("send-keys enter", &enter_args).map(|_| ())
}

pub fn interrupt_session(session_name: &str) -> TmuxAdapterResult<()> {
    validate_session_name(session_name)?;
    let args = vec![
        "send-keys".to_owned(),
        "-t".to_owned(),
        session_name.to_owned(),
        "C-c".to_owned(),
    ];
    run_tmux_command("send-keys ctrl-c", &args).map(|_| ())
}

pub fn stop_session(session_name: &str) -> TmuxAdapterResult<()> {
    validate_session_name(session_name)?;
    let args = vec![
        "kill-session".to_owned(),
        "-t".to_owned(),
        session_name.to_owned(),
    ];
    run_tmux_command("kill-session", &args).map(|_| ())
}

fn validate_session_name(session_name: &str) -> TmuxAdapterResult<()> {
    if session_name.trim().is_empty() {
        return Err(TmuxAdapterError::Validation {
            reason: "session_name must not be empty".to_owned(),
        });
    }
    Ok(())
}
