use crate::error::GatewayResult;
use crate::input::option_value;
use crate::model::TeamReq02ParsedCommand;
use crate::options::parse_options;

pub(crate) fn parse_control(
    action: &str,
    args: &[String],
) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &[
            "--agent",
            "--team",
            "--session",
            "--cwd",
            "--project",
            "--input",
            "--task",
            "--error-fact",
        ],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::Control {
        action: Some(action.to_owned()),
        agent_name: option_value(&options, "--agent"),
        team_id: option_value(&options, "--team"),
        session_name: option_value(&options, "--session"),
        cwd: option_value(&options, "--cwd"),
        project_slug: option_value(&options, "--project"),
        input: option_value(&options, "--input"),
        task_id: option_value(&options, "--task"),
        error_fact_id: option_value(&options, "--error-fact"),
        json: options.json,
    })
}
