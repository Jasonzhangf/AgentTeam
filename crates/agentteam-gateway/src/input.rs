use crate::broadcast::parse_msg_broadcast;
use crate::control::parse_control;
use crate::error::{GatewayError, GatewayResult};
use crate::model::{TeamReq01CliRaw, TeamReq02ParsedCommand, TeamReq03ValidatedIntent};
use crate::options::{parse_options, ParsedOptions};
use crate::validate::validate_intent;

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
        [area, rest @ ..] if area == "start" => parse_start(rest),
        [area, action, rest @ ..] if area == "ready" && action == "report" => {
            parse_ready_report(rest)
        }
        [area, action, rest @ ..] if area == "task" && action == "send" => parse_task_send(rest),
        [area, action, rest @ ..] if area == "task" && action == "list" => parse_task_list(rest),
        [area, action, rest @ ..] if area == "task" && action == "status" => {
            parse_task_status(rest)
        }
        [area, action, rest @ ..] if area == "task" && action == "done" => parse_task_done(rest),
        [area, action, rest @ ..] if area == "task" && action == "error" => parse_task_error(rest),
        [area, action, rest @ ..] if area == "task" && action == "claim" => parse_task_claim(rest),
        [area, action, rest @ ..] if area == "msg" && action == "send" => parse_msg_send(rest),
        [area, action, rest @ ..] if area == "msg" && action == "broadcast" => {
            parse_msg_broadcast(rest)
        }
        [area, action, rest @ ..] if area == "control" => parse_control(action, rest),
        [area, action, rest @ ..] if area == "tmux" && action == "loopback" => {
            parse_tmux_loopback(rest)
        }
        [area, action, rest @ ..] if area == "report" && action == "flow" => {
            parse_report_flow(rest)
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

fn parse_tmux_loopback(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(args, &["--runtime-home", "--session-count"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::TmuxLoopback {
        runtime_home: option_value(&options, "--runtime-home"),
        session_count: option_value(&options, "--session-count"),
        json: options.json,
    })
}

fn parse_report_flow(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(args, &["--runtime-home"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::ReportFlow {
        runtime_home: option_value(&options, "--runtime-home"),
        json: options.json,
    })
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

fn parse_start(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    if let Some((first, rest)) = args.split_first() {
        if first == "worker" {
            return parse_start_worker(rest);
        }
    }
    let options = parse_options(args, &["--cwd", "--config", "--team"], &["--json"])?;
    Ok(TeamReq02ParsedCommand::Startup {
        cwd: option_value(&options, "--cwd"),
        config_path: option_value(&options, "--config"),
        team_id: option_value(&options, "--team"),
        json: options.json,
    })
}

fn parse_start_worker(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &["--agent", "--cwd", "--config", "--team"],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::StartupWorker {
        agent_name: option_value(&options, "--agent"),
        cwd: option_value(&options, "--cwd"),
        config_path: option_value(&options, "--config"),
        team_id: option_value(&options, "--team"),
        json: options.json,
    })
}

fn parse_ready_report(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &[
            "--runtime-home",
            "--sender",
            "--team",
            "--agent-name",
            "--body",
        ],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::ReadyReport {
        runtime_home: option_value(&options, "--runtime-home"),
        sender: option_value(&options, "--sender"),
        team_id: option_value(&options, "--team"),
        agent_name: option_value(&options, "--agent-name"),
        body: option_value(&options, "--body"),
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

fn parse_task_claim(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &["--runtime-home", "--worker-name", "--worker-role"],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::TaskClaim {
        runtime_home: option_value(&options, "--runtime-home"),
        worker_name: option_value(&options, "--worker-name"),
        worker_role: option_value(&options, "--worker-role"),
        json: options.json,
    })
}

fn parse_msg_send(args: &[String]) -> GatewayResult<TeamReq02ParsedCommand> {
    let options = parse_options(
        args,
        &["--runtime-home", "--from", "--to", "--action", "--body"],
        &["--json"],
    )?;
    Ok(TeamReq02ParsedCommand::MsgSend {
        runtime_home: option_value(&options, "--runtime-home"),
        from: option_value(&options, "--from"),
        to: option_value(&options, "--to"),
        action: option_value(&options, "--action"),
        body: option_value(&options, "--body"),
        json: options.json,
    })
}

pub(crate) fn option_value(options: &ParsedOptions, flag: &str) -> Option<String> {
    options.values.get(flag).cloned()
}
