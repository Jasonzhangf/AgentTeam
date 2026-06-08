use std::collections::BTreeMap;

use crate::error::{GatewayError, GatewayResult};
use crate::model::{TeamReq01CliRaw, TeamReq02ParsedCommand, TeamReq03ValidatedIntent};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedOptions {
    values: BTreeMap<String, String>,
    json: bool,
}

pub fn parse_cli_args(args: Vec<String>) -> GatewayResult<TeamReq03ValidatedIntent> {
    let raw = TeamReq01CliRaw::new(args);
    let parsed = parse_cli_raw(raw)?;
    validate_intent(parsed)
}

fn parse_cli_raw(raw: TeamReq01CliRaw) -> GatewayResult<TeamReq02ParsedCommand> {
    match raw.args.as_slice() {
        [area, action, rest @ ..] if area == "config" && action == "check" => {
            parse_config_check(rest)
        }
        [area, action, rest @ ..] if area == "daemon" && action == "check" => {
            parse_daemon_check(rest)
        }
        [area, action, rest @ ..] if area == "domain" && action == "resolve" => {
            parse_domain_resolve(rest)
        }
        [area, action, rest @ ..] if area == "debug" && action == "snapshot" => {
            parse_debug_snapshot(rest)
        }
        [area, action, rest @ ..] if area == "task" && action == "send" => parse_task_send(rest),
        [area, action, rest @ ..] if area == "task" && action == "list" => parse_task_list(rest),
        [area, action, rest @ ..] if area == "task" && action == "status" => {
            parse_task_status(rest)
        }
        [area, action, rest @ ..] if area == "task" && action == "done" => parse_task_done(rest),
        [area, action, rest @ ..] if area == "task" && action == "error" => parse_task_error(rest),
        [] => Err(GatewayError::parse("command is required")),
        [area, action, ..] => Err(GatewayError::parse(format!(
            "unsupported command {area} {action}"
        ))),
        [area] => Err(GatewayError::parse(format!(
            "command action is required for {area}"
        ))),
    }
}

fn parse_config_check(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(args, &["--config"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::ConfigCheck {
        config_path: option_value(&options, "--config"),
        json: options.json,
    })
}

fn parse_daemon_check(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(args, &["--config"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::DaemonCheck {
        config_path: option_value(&options, "--config"),
        json: options.json,
    })
}

fn parse_domain_resolve(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(args, &["--config", "--target"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::DomainResolve {
        target: option_value(&options, "--target"),
        config_path: option_value(&options, "--config"),
        json: options.json,
    })
}

fn parse_debug_snapshot(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(args, &["--config", "--runtime-home"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::DebugSnapshot {
        config_path: option_value(&options, "--config"),
        runtime_home: option_value(&options, "--runtime-home"),
        json: options.json,
    })
}

fn parse_task_send(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &[
            "--runtime-home",
            "--team",
            "--created-by",
            "--target-kind",
            "--target",
            "--title",
            "--body",
        ],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::TaskSend {
        runtime_home: option_value(&options, "--runtime-home"),
        team_id: option_value(&options, "--team"),
        created_by: option_value(&options, "--created-by"),
        target_kind: option_value(&options, "--target-kind"),
        target: option_value(&options, "--target"),
        title: option_value(&options, "--title"),
        body: option_value(&options, "--body"),
        json: options.json,
    })
}

fn parse_task_list(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(args, &["--runtime-home"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::TaskList {
        runtime_home: option_value(&options, "--runtime-home"),
        json: options.json,
    })
}

fn parse_task_status(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(args, &["--runtime-home", "--task"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::TaskStatus {
        runtime_home: option_value(&options, "--runtime-home"),
        task_id: option_value(&options, "--task"),
        json: options.json,
    })
}

fn parse_task_done(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &["--runtime-home", "--task", "--actor", "--detail"],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::TaskDone {
        runtime_home: option_value(&options, "--runtime-home"),
        task_id: option_value(&options, "--task"),
        actor: option_value(&options, "--actor"),
        detail: option_value(&options, "--detail"),
        json: options.json,
    })
}

fn parse_task_error(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &["--runtime-home", "--task", "--actor", "--detail"],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::TaskError {
        runtime_home: option_value(&options, "--runtime-home"),
        task_id: option_value(&options, "--task"),
        actor: option_value(&options, "--actor"),
        detail: option_value(&options, "--detail"),
        json: options.json,
    })
}

fn parse_options(
    args: &[String],
    value_flags: &[&str],
    bool_flags: &[&str],
) -> GatewayResult<ParsedOptions> {
    let mut values = BTreeMap::new();
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        if contains_flag(value_flags, token) {
            if values.contains_key(token) {
                return Err(GatewayError::parse(format!("duplicate flag {token}")));
            }
            let Some(value) = args.get(index + 1) else {
                return Err(GatewayError::parse(format!("missing value for {token}")));
            };
            if value.starts_with("--") {
                return Err(GatewayError::parse(format!("missing value for {token}")));
            }
            values.insert(token.clone(), value.clone());
            index += 2;
        } else if contains_flag(bool_flags, token) {
            if token == "--json" && json {
                return Err(GatewayError::parse("duplicate flag --json"));
            }
            json = true;
            index += 1;
        } else if token.starts_with("--") {
            return Err(GatewayError::parse(format!("unknown flag {token}")));
        } else {
            return Err(GatewayError::parse(format!(
                "unexpected positional argument {token}"
            )));
        }
    }
    Ok(ParsedOptions { values, json })
}

fn validate_intent(parsed: TeamReq02ParsedCommand) -> GatewayResult<TeamReq03ValidatedIntent> {
    match parsed {
        TeamReq02ParsedCommand::ConfigCheck { config_path, json } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::ConfigCheck {
                config_path: require_value(config_path, "--config")?,
                json,
            })
        }
        TeamReq02ParsedCommand::DaemonCheck { config_path, json } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::DaemonCheck {
                config_path: require_value(config_path, "--config")?,
                json,
            })
        }
        TeamReq02ParsedCommand::DomainResolve {
            target,
            config_path,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::DomainResolve {
                target: require_value(target, "--target")?,
                config_path: require_value(config_path, "--config")?,
                json,
            })
        }
        TeamReq02ParsedCommand::DebugSnapshot {
            config_path,
            runtime_home,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::DebugSnapshot {
                config_path: require_value(config_path, "--config")?,
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                json,
            })
        }
        TeamReq02ParsedCommand::TaskSend {
            runtime_home,
            team_id,
            created_by,
            target_kind,
            target,
            title,
            body,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::TaskSend {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                team_id: require_value(team_id, "--team")?,
                created_by: require_value(created_by, "--created-by")?,
                target_kind: require_value(target_kind, "--target-kind")?,
                target: require_value(target, "--target")?,
                title: require_value(title, "--title")?,
                body: require_value(body, "--body")?,
                json,
            })
        }
        TeamReq02ParsedCommand::TaskList { runtime_home, json } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::TaskList {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                json,
            })
        }
        TeamReq02ParsedCommand::TaskStatus {
            runtime_home,
            task_id,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::TaskStatus {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                task_id: require_value(task_id, "--task")?,
                json,
            })
        }
        TeamReq02ParsedCommand::TaskDone {
            runtime_home,
            task_id,
            actor,
            detail,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::TaskDone {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                task_id: require_value(task_id, "--task")?,
                actor: require_value(actor, "--actor")?,
                detail: require_value(detail, "--detail")?,
                json,
            })
        }
        TeamReq02ParsedCommand::TaskError {
            runtime_home,
            task_id,
            actor,
            detail,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::TaskError {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                task_id: require_value(task_id, "--task")?,
                actor: require_value(actor, "--actor")?,
                detail: require_value(detail, "--detail")?,
                json,
            })
        }
    }
}

fn require_json(json: bool) -> GatewayResult<()> {
    if json {
        Ok(())
    } else {
        Err(GatewayError::validation(
            "local parsing MVP requires --json output",
        ))
    }
}

fn require_value(value: Option<String>, flag: &str) -> GatewayResult<String> {
    value.ok_or_else(|| GatewayError::validation(format!("{flag} is required")))
}

fn option_value(options: &ParsedOptions, flag: &str) -> Option<String> {
    options.values.get(flag).cloned()
}

fn contains_flag(allowed: &[&str], token: &str) -> bool {
    allowed.contains(&token)
}
