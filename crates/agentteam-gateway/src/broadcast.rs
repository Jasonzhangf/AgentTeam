use crate::error::{GatewayError, GatewayResult};
use crate::input::option_value;
use crate::model::TeamReq02ParsedCommand;
use crate::options::parse_options;

pub(crate) fn parse_msg_broadcast(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &[
            "--runtime-home",
            "--sender",
            "--team",
            "--action",
            "--body",
            "--members",
        ],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::MsgBroadcast {
        runtime_home: option_value(&options, "--runtime-home"),
        sender: option_value(&options, "--sender"),
        team_id: option_value(&options, "--team"),
        action: option_value(&options, "--action"),
        body: option_value(&options, "--body"),
        members: option_value(&options, "--members"),
        json: options.json,
    })
}

pub(crate) fn parse_members_list(value: String) -> GatewayResult<Vec<String>> {
    let members: Vec<String> = value
        .split(',')
        .map(|member| member.trim().to_owned())
        .filter(|member| !member.is_empty())
        .collect();
    if members.is_empty() {
        return Err(GatewayError::validation(
            "members must contain at least one entry",
        ));
    }
    Ok(members)
}
