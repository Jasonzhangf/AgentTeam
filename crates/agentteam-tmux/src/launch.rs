use std::path::Path;

use crate::error::{TmuxAdapterError, TmuxAdapterResult};
use crate::model::{TmuxLaunchInput, TmuxLaunchReport};
use crate::process::run_tmux_command;

pub fn launch_managed_session(input: TmuxLaunchInput) -> TmuxAdapterResult<TmuxLaunchReport> {
    validate_launch_input(&input)?;
    let command_line = build_shell_command(&input.cwd, &input.command, &input.args, &input.env);
    let args = vec![
        "new-session".to_owned(),
        "-d".to_owned(),
        "-s".to_owned(),
        input.session_name.clone(),
        "-c".to_owned(),
        input.cwd.clone(),
        "sh".to_owned(),
        "-lc".to_owned(),
        command_line.clone(),
    ];
    run_tmux_command("new-session", &args)?;
    Ok(TmuxLaunchReport {
        session_name: input.session_name,
        cwd: input.cwd,
        command_line,
        env_count: input.env.len(),
        arg_count: input.args.len(),
    })
}

fn validate_launch_input(input: &TmuxLaunchInput) -> TmuxAdapterResult<()> {
    for (field, value) in [
        ("session_name", input.session_name.as_str()),
        ("cwd", input.cwd.as_str()),
        ("command", input.command.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(TmuxAdapterError::Validation {
                reason: format!("{field} must not be empty"),
            });
        }
    }
    if !Path::new(&input.cwd).is_absolute() {
        return Err(TmuxAdapterError::Validation {
            reason: "--cwd must be an absolute path".to_owned(),
        });
    }
    Ok(())
}

fn build_shell_command(
    cwd: &str,
    command: &str,
    args: &[String],
    env: &std::collections::BTreeMap<String, String>,
) -> String {
    let mut script = String::new();
    for (key, value) in env {
        script.push_str("export ");
        script.push_str(key);
        script.push('=');
        script.push_str(&quote_shell_word(value));
        script.push_str("; ");
    }
    script.push_str("cd ");
    script.push_str(&quote_shell_word(cwd));
    script.push_str(" && exec ");
    script.push_str(&quote_shell_word(command));
    for arg in args {
        script.push(' ');
        script.push_str(&quote_shell_word(arg));
    }
    script
}

fn quote_shell_word(value: &str) -> String {
    if value.is_empty() {
        return "''".to_owned();
    }
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "-_./:@".contains(ch))
    {
        return value.to_owned();
    }
    let escaped = value.replace('\'', "'\"'\"'");
    format!("'{escaped}'")
}
