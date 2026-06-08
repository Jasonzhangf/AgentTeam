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
    command_name: &'static str,
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

fn local_result_command_name(result: &LocalCommandResult) -> &'static str {
    match result {
        LocalCommandResult::ConfigCheck { .. } => "config.check",
        LocalCommandResult::DaemonCheck { .. } => "daemon.check",
        LocalCommandResult::DomainResolve { .. } => "domain.resolve",
        LocalCommandResult::DebugSnapshot { .. } => "debug.snapshot",
        LocalCommandResult::TaskSend { .. } => "task.send",
        LocalCommandResult::TaskList { .. } => "task.list",
        LocalCommandResult::TaskStatus { .. } => "task.status",
        LocalCommandResult::TaskDone { .. } => "task.done",
        LocalCommandResult::TaskError { .. } => "task.error",
        LocalCommandResult::TmuxLoopback { .. } => "tmux.loopback",
    }
}

fn local_error_class(error: &LocalCommandError) -> &'static str {
    match error {
        LocalCommandError::Config { .. } => "config",
        LocalCommandError::Domain { .. } => "domain",
        LocalCommandError::Debug { .. } => "debug",
        LocalCommandError::Task { .. } => "task",
        LocalCommandError::Tmux { .. } => "tmux",
    }
}

fn local_error_reason(error: &LocalCommandError) -> String {
    match error {
        LocalCommandError::Config { reason }
        | LocalCommandError::Domain { reason }
        | LocalCommandError::Debug { reason }
        | LocalCommandError::Task { reason }
        | LocalCommandError::Tmux { reason } => reason.clone(),
    }
}
