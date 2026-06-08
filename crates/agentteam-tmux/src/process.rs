use std::process::Command;

use crate::error::{TmuxAdapterError, TmuxAdapterResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TmuxCommandOutput {
    pub stdout: String,
    pub stderr: String,
}

pub(crate) fn run_tmux_command(
    operation: &str,
    args: &[String],
) -> TmuxAdapterResult<TmuxCommandOutput> {
    let output = Command::new("tmux").args(args).output().map_err(|error| {
        TmuxAdapterError::ProcessLaunch {
            reason: format!("failed to run tmux for {operation}: {error}"),
        }
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if output.status.success() {
        Ok(TmuxCommandOutput { stdout, stderr })
    } else {
        Err(TmuxAdapterError::CommandFailed {
            operation: operation.to_owned(),
            reason: format!(
                "tmux {operation} failed with status {}; stderr: {}",
                output.status,
                stderr.trim()
            ),
        })
    }
}
