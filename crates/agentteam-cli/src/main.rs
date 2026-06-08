use std::env;
use std::process::ExitCode;

use agentteam_gateway::{
    parse_cli_args, render_gateway_error_json, render_local_error_json, render_local_result_json,
};
use agentteam_runtime::local::execute_local_intent;

fn main() -> ExitCode {
    let args = env::args().skip(1).collect();
    let intent = match parse_cli_args(args) {
        Ok(intent) => intent,
        Err(error) => return render_gateway_error(&error),
    };
    match execute_local_intent(intent) {
        Ok(result) => render_local_result(&result),
        Err(error) => render_local_error(&error),
    }
}

fn render_local_result(result: &agentteam_runtime::local::LocalCommandResult) -> ExitCode {
    match render_local_result_json(result) {
        Ok(rendered) => {
            println!("{rendered}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}", error.reason);
            ExitCode::from(2)
        }
    }
}

fn render_local_error(error: &agentteam_runtime::local::LocalCommandError) -> ExitCode {
    match render_local_error_json(error) {
        Ok(rendered) => eprintln!("{rendered}"),
        Err(render_error) => eprintln!("{}", render_error.reason),
    }
    ExitCode::from(2)
}

fn render_gateway_error(error: &agentteam_gateway::GatewayError) -> ExitCode {
    match render_gateway_error_json(error) {
        Ok(rendered) => eprintln!("{rendered}"),
        Err(render_error) => eprintln!("{}", render_error.reason),
    }
    ExitCode::from(2)
}
