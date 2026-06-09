use serde::Serialize;

use crate::error::{GatewayError, GatewayResult};
use crate::model::TeamReq03ValidatedIntent;
use agentteam_runtime::local::{LocalCommandError, LocalCommandResult};

#[derive(Serialize)]
struct IntentProjection<'a> {
    status: &'static str,
    feature_id: &'static str,
    node: &'static str,
    local_parse_only: bool,
    command_name: &'static str,
    intent: &'a TeamReq03ValidatedIntent,
}

#[derive(Serialize)]
struct ErrorProjection<'a> {
    status: &'static str,
    feature_id: &'static str,
    class: &'static str,
    reason: &'a str,
}

#[derive(Serialize)]
struct LocalResultProjection<'a> {
    status: &'static str,
    feature_id: &'static str,
    node: &'static str,
    command_name: String,
    result: &'a LocalCommandResult,
}

pub fn render_intent_json(intent: &TeamReq03ValidatedIntent) -> GatewayResult<String> {
    let projection = IntentProjection {
        status: "ok",
        feature_id: crate::INPUT_FEATURE_ID,
        node: "TeamReq03ValidatedIntent",
        local_parse_only: true,
        command_name: intent.command_name(),
        intent,
    };
    encode_intent_projection(&projection)
}

pub fn render_gateway_error_json(error: &GatewayError) -> GatewayResult<String> {
    let projection = ErrorProjection {
        status: "error",
        feature_id: error.feature_id,
        class: error.class,
        reason: &error.reason,
    };
    encode_error_projection(&projection)
}

pub fn render_local_result_json(result: &LocalCommandResult) -> GatewayResult<String> {
    let projection = LocalResultProjection {
        status: "ok",
        feature_id: crate::OUTPUT_FEATURE_ID,
        node: "TeamResp05DaemonResult",
        command_name: local_result_command_name(result),
        result,
    };
    encode_local_result_projection(&projection)
}

pub fn render_local_error_json(error: &LocalCommandError) -> GatewayResult<String> {
    let reason = local_error_reason(error);
    let projection = ErrorProjection {
        status: "error",
        feature_id: crate::OUTPUT_FEATURE_ID,
        class: local_error_class(error),
        reason: &reason,
    };
    encode_error_projection(&projection)
}

fn encode_intent_projection(projection: &IntentProjection<'_>) -> GatewayResult<String> {
    serde_json::to_string(projection)
        .map_err(|error| GatewayError::output(format!("failed to render intent JSON: {error}")))
}

fn encode_local_result_projection(projection: &LocalResultProjection<'_>) -> GatewayResult<String> {
    serde_json::to_string(projection)
        .map_err(|error| GatewayError::output(format!("failed to render result JSON: {error}")))
}

fn encode_error_projection(projection: &ErrorProjection<'_>) -> GatewayResult<String> {
    serde_json::to_string(projection)
        .map_err(|error| GatewayError::output(format!("failed to render error JSON: {error}")))
}

fn local_result_command_name(result: &LocalCommandResult) -> String {
    match result {
        LocalCommandResult::ConfigCheck { .. } => "config.check".to_owned(),
        LocalCommandResult::DaemonCheck { .. } => "daemon.check".to_owned(),
        LocalCommandResult::DomainResolve { .. } => "domain.resolve".to_owned(),
        LocalCommandResult::DebugSnapshot { .. } => "debug.snapshot".to_owned(),
        LocalCommandResult::StartupStart { .. } => "start".to_owned(),
        LocalCommandResult::StartupWorker { .. } => "start.worker".to_owned(),
        LocalCommandResult::Control { control } => format!("control.{}", control.action),
        LocalCommandResult::TaskSend { .. } => "task.send".to_owned(),
        LocalCommandResult::TaskList { .. } => "task.list".to_owned(),
        LocalCommandResult::TaskStatus { .. } => "task.status".to_owned(),
        LocalCommandResult::TaskDone { .. } => "task.done".to_owned(),
        LocalCommandResult::TaskError { .. } => "task.error".to_owned(),
        LocalCommandResult::TaskClaim { .. } => "task.claim".to_owned(),
        LocalCommandResult::MessageSend { .. } => "msg.send".to_owned(),
        LocalCommandResult::BroadcastSend { .. } => "msg.broadcast".to_owned(),
        LocalCommandResult::ReadyReport { .. } => "ready.report".to_owned(),
        LocalCommandResult::TmuxLoopback { .. } => "tmux.loopback".to_owned(),
        LocalCommandResult::ReportFlow { .. } => "report.flow".to_owned(),
    }
}

fn local_error_class(error: &LocalCommandError) -> &'static str {
    match error {
        LocalCommandError::Config { .. } => "config",
        LocalCommandError::Domain { .. } => "domain",
        LocalCommandError::Debug { .. } => "debug",
        LocalCommandError::Startup { .. } => "startup",
        LocalCommandError::Control { .. } => "control",
        LocalCommandError::Task { .. } => "task",
        LocalCommandError::Comm { .. } => "comm",
        LocalCommandError::Tmux { .. } => "tmux",
        LocalCommandError::Report { .. } => "report",
    }
}

fn local_error_reason(error: &LocalCommandError) -> String {
    match error {
        LocalCommandError::Config { reason }
        | LocalCommandError::Domain { reason }
        | LocalCommandError::Debug { reason }
        | LocalCommandError::Startup { reason }
        | LocalCommandError::Control { reason }
        | LocalCommandError::Task { reason }
        | LocalCommandError::Comm { reason }
        | LocalCommandError::Tmux { reason }
        | LocalCommandError::Report { reason } => reason.clone(),
    }
}
