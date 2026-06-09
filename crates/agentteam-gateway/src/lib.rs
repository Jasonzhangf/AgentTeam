mod broadcast;
mod control;
mod error;
mod input;
mod model;
mod options;
mod output;
#[cfg(test)]
mod tests;
mod validate;

pub use error::{GatewayError, GatewayResult};
pub use input::parse_cli_args;
pub use model::{TeamReq01CliRaw, TeamReq02ParsedCommand, TeamReq03ValidatedIntent};
pub use output::{
    render_gateway_error_json, render_intent_json, render_local_error_json,
    render_local_result_json,
};

pub const INPUT_FEATURE_ID: &str = "gateway.input";
pub const OUTPUT_FEATURE_ID: &str = "gateway.output";
pub const UI_FEATURE_ID: &str = "gateway.ui";
