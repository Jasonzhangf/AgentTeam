use serde::Serialize;

use crate::error::{GatewayError, GatewayResult};
use crate::model::TeamReq03ValidatedIntent;

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

fn encode_intent_projection(projection: &IntentProjection<'_>) -> GatewayResult<String> {
    serde_json::to_string(projection)
        .map_err(|error| GatewayError::output(format!("failed to render intent JSON: {error}")))
}

fn encode_error_projection(projection: &ErrorProjection<'_>) -> GatewayResult<String> {
    serde_json::to_string(projection)
        .map_err(|error| GatewayError::output(format!("failed to render error JSON: {error}")))
}
