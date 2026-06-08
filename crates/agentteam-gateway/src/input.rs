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
