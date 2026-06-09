use crate::broadcast::parse_members_list;
use crate::error::{GatewayError, GatewayResult};
use crate::model::{TeamReq02ParsedCommand, TeamReq03ValidatedIntent};

pub(crate) fn validate_intent(
    parsed: TeamReq02ParsedCommand,
) -> GatewayResult<TeamReq03ValidatedIntent> {
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
        TeamReq02ParsedCommand::Startup {
            cwd,
            config_path,
            team_id,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::Startup {
                cwd,
                config_path,
                team_id,
                json,
            })
        }
        TeamReq02ParsedCommand::StartupWorker {
            agent_name,
            cwd,
            config_path,
            team_id,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::StartupWorker {
                agent_name: require_value(agent_name, "--agent")?,
                cwd,
                config_path,
                team_id,
                json,
            })
        }
        TeamReq02ParsedCommand::ReadyReport {
            runtime_home,
            sender,
            team_id,
            agent_name,
            body,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::ReadyReport {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                sender: require_value(sender, "--sender")?,
                team_id: require_value(team_id, "--team")?,
                agent_name: require_value(agent_name, "--agent-name")?,
                body: require_value(body, "--body")?,
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
        TeamReq02ParsedCommand::TaskClaim {
            runtime_home,
            worker_name,
            worker_role,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::TaskClaim {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                worker_name: require_value(worker_name, "--worker-name")?,
                worker_role: require_value(worker_role, "--worker-role")?,
                json,
            })
        }
        TeamReq02ParsedCommand::MsgSend {
            runtime_home,
            from,
            to,
            action,
            body,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::MsgSend {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                from: require_value(from, "--from")?,
                to: require_value(to, "--to")?,
                action: require_value(action, "--action")?,
                body: require_value(body, "--body")?,
                json,
            })
        }
        TeamReq02ParsedCommand::MsgBroadcast {
            runtime_home,
            sender,
            team_id,
            action,
            body,
            members,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::MsgBroadcast {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                sender: require_value(sender, "--sender")?,
                team_id: require_value(team_id, "--team")?,
                action: require_value(action, "--action")?,
                body: require_value(body, "--body")?,
                members: parse_members_list(require_value(members, "--members")?)?,
                json,
            })
        }
        TeamReq02ParsedCommand::Control {
            action,
            agent_name,
            team_id,
            session_name,
            input,
            task_id,
            error_fact_id,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::Control {
                action: require_value(action, "control action")?,
                agent_name: require_value(agent_name, "--agent")?,
                team_id: require_value(team_id, "--team")?,
                session_name: require_value(session_name, "--session")?,
                input,
                task_id,
                error_fact_id,
                json,
            })
        }
        TeamReq02ParsedCommand::TmuxLoopback {
            runtime_home,
            session_count,
            json,
        } => {
            require_json(json)?;
            Ok(TeamReq03ValidatedIntent::TmuxLoopback {
                runtime_home: require_value(runtime_home, "--runtime-home")?,
                session_count: require_value(session_count, "--session-count")?,
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
