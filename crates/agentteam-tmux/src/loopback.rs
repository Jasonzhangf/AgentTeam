use std::path::Path;
use std::thread;
use std::time::Duration;

use agentteam_contracts::terminal::TerminalReq01AdapterCommand;

use crate::error::{TmuxAdapterError, TmuxAdapterResult};
use crate::model::TmuxLoopbackReport;
use crate::model::{ManagedTmuxSession, TmuxLoopbackInput, TmuxLoopbackObservation};
use crate::process::run_tmux_command;

const MAX_LOOPBACK_SESSIONS: usize = 8;
const OBSERVE_ATTEMPTS: usize = 20;
const OBSERVE_INTERVAL: Duration = Duration::from_millis(50);

pub fn run_tmux_loopback(input: TmuxLoopbackInput) -> TmuxAdapterResult<TmuxLoopbackReport> {
    validate_loopback_input(&input)?;
    let scope = runtime_scope(&input.runtime_home)?;
    let command = TerminalReq01AdapterCommand::loopback(scope.clone(), input.session_count);
    let transport = command.prepare_transport(session_prefix(&scope));
    let sessions = build_sessions(&transport.session_prefix, transport.session_count);

    let step_result = run_loopback_steps(&sessions);
    let cleanup_result = cleanup_sessions(&sessions);
    combine_step_and_cleanup(input.session_count, step_result, cleanup_result)
}

fn run_loopback_steps(
    sessions: &[ManagedTmuxSession],
) -> TmuxAdapterResult<Vec<TmuxLoopbackObservation>> {
    for session in sessions {
        launch_session(session)?;
    }
    for session in sessions {
        wait_for_marker(session, &session.ready_marker)?;
    }
    for session in sessions {
        send_input(session)?;
    }

    let mut observations = Vec::new();
    for session in sessions {
        let observed_text = wait_for_marker(session, &session.output_marker)?;
        observations.push(TmuxLoopbackObservation {
            logical_id: session.logical_id.clone(),
            input_marker: session.input_marker.clone(),
            output_marker: session.output_marker.clone(),
            observed_text,
        });
    }
    Ok(observations)
}

pub(crate) fn validate_loopback_input(input: &TmuxLoopbackInput) -> TmuxAdapterResult<()> {
    if input.runtime_home.trim().is_empty() {
        return Err(TmuxAdapterError::Validation {
            reason: "--runtime-home is required".to_owned(),
        });
    }
    if input.session_count == 0 || input.session_count > MAX_LOOPBACK_SESSIONS {
        return Err(TmuxAdapterError::Validation {
            reason: format!("--session-count must be between 1 and {MAX_LOOPBACK_SESSIONS}"),
        });
    }
    Ok(())
}

pub(crate) fn runtime_scope(runtime_home: &str) -> TmuxAdapterResult<String> {
    let path = Path::new(runtime_home);
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return Err(TmuxAdapterError::Validation {
            reason: "--runtime-home must end with a project/runtime directory name".to_owned(),
        });
    };
    sanitized_runtime_scope(name)
}

pub(crate) fn sanitized_runtime_scope(value: &str) -> TmuxAdapterResult<String> {
    let mut sanitized = String::new();
    let mut last_was_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            sanitized.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if (ch == '-' || ch == '_' || ch == '.' || ch.is_whitespace()) && !last_was_dash {
            sanitized.push('-');
            last_was_dash = true;
        }
    }
    let trimmed = sanitized.trim_matches('-').to_owned();
    if trimmed.is_empty() {
        Err(TmuxAdapterError::Validation {
            reason: "--runtime-home directory name must contain ascii letters or digits".to_owned(),
        })
    } else {
        Ok(trimmed)
    }
}

fn session_prefix(scope: &str) -> String {
    format!("TA-{scope}-loopback-{}", std::process::id())
}

pub(crate) fn build_sessions(prefix: &str, session_count: usize) -> Vec<ManagedTmuxSession> {
    (1..=session_count)
        .map(|index| {
            let logical_id = format!("agent-{index:02}");
            let session_name = format!("{prefix}-{index:02}");
            let input_marker = format!("AGENTTEAM_LOOPBACK_INPUT:{logical_id}");
            let ready_marker = format!("AGENTTEAM_LOOPBACK_READY:{session_name}");
            let output_marker = format!("AGENTTEAM_LOOPBACK_ECHO:{session_name}:{input_marker}");
            ManagedTmuxSession {
                logical_id,
                session_name,
                ready_marker,
                input_marker,
                output_marker,
            }
        })
        .collect()
}

fn launch_session(session: &ManagedTmuxSession) -> TmuxAdapterResult<()> {
    let args = vec![
        "new-session".to_owned(),
        "-d".to_owned(),
        "-s".to_owned(),
        session.session_name.clone(),
        "sh".to_owned(),
        "-lc".to_owned(),
        session_shell_command(&session.session_name),
    ];
    run_tmux_command("new-session", &args).map(|_| ())
}

fn session_shell_command(session_name: &str) -> String {
    format!(
        "AGENTTEAM_LOOPBACK_NAME={session_name}; \
         printf 'AGENTTEAM_LOOPBACK_READY:%s\\n' \"$AGENTTEAM_LOOPBACK_NAME\"; \
         while IFS= read -r line; do \
         printf 'AGENTTEAM_LOOPBACK_ECHO:%s:%s\\n' \"$AGENTTEAM_LOOPBACK_NAME\" \"$line\"; \
         done"
    )
}

fn wait_for_marker(session: &ManagedTmuxSession, marker: &str) -> TmuxAdapterResult<String> {
    for _ in 0..OBSERVE_ATTEMPTS {
        let captured = capture_session(session)?;
        if captured.contains(marker) {
            return Ok(captured);
        }
        thread::sleep(OBSERVE_INTERVAL);
    }
    Err(TmuxAdapterError::Observation {
        logical_id: session.logical_id.clone(),
        reason: format!(
            "tmux output did not contain expected marker for {}",
            session.logical_id
        ),
    })
}

fn capture_session(session: &ManagedTmuxSession) -> TmuxAdapterResult<String> {
    let args = vec![
        "capture-pane".to_owned(),
        "-p".to_owned(),
        "-J".to_owned(),
        "-t".to_owned(),
        session.session_name.clone(),
    ];
    run_tmux_command("capture-pane", &args).map(|output| output.stdout)
}

fn send_input(session: &ManagedTmuxSession) -> TmuxAdapterResult<()> {
    let literal_args = vec![
        "send-keys".to_owned(),
        "-t".to_owned(),
        session.session_name.clone(),
        "-l".to_owned(),
        session.input_marker.clone(),
    ];
    run_tmux_command("send-keys literal", &literal_args)?;
    let enter_args = vec![
        "send-keys".to_owned(),
        "-t".to_owned(),
        session.session_name.clone(),
        "Enter".to_owned(),
    ];
    run_tmux_command("send-keys enter", &enter_args).map(|_| ())
}

fn cleanup_sessions(sessions: &[ManagedTmuxSession]) -> TmuxAdapterResult<usize> {
    let mut cleaned = 0;
    for session in sessions {
        let args = vec![
            "kill-session".to_owned(),
            "-t".to_owned(),
            session.session_name.clone(),
        ];
        run_tmux_command("kill-session", &args).map_err(|error| TmuxAdapterError::Cleanup {
            logical_id: session.logical_id.clone(),
            reason: error.reason(),
        })?;
        cleaned += 1;
    }
    Ok(cleaned)
}

fn combine_step_and_cleanup(
    requested_count: usize,
    step_result: TmuxAdapterResult<Vec<TmuxLoopbackObservation>>,
    cleanup_result: TmuxAdapterResult<usize>,
) -> TmuxAdapterResult<TmuxLoopbackReport> {
    match (step_result, cleanup_result) {
        (Ok(observations), Ok(cleaned_handle_count)) => Ok(TmuxLoopbackReport {
            requested_count,
            observed_count: observations.len(),
            cleaned_handle_count,
            observations,
        }),
        (Err(error), Ok(_)) => Err(error),
        (Ok(_), Err(cleanup)) => Err(cleanup),
        (Err(primary), Err(cleanup)) => Err(TmuxAdapterError::CleanupAfterFailure {
            primary: primary.reason(),
            cleanup: cleanup.reason(),
        }),
    }
}
