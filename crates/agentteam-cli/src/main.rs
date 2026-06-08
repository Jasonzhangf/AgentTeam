use std::env;
use std::process::ExitCode;

use agentteam_gateway::{parse_cli_args, render_gateway_error_json, render_intent_json};

fn main() -> ExitCode {
    let args = env::args().skip(1).collect();
    match parse_cli_args(args).and_then(|intent| render_intent_json(&intent)) {
        Ok(rendered) => {
            println!("{rendered}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            match render_gateway_error_json(&error) {
                Ok(rendered) => eprintln!("{rendered}"),
                Err(render_error) => eprintln!("{}", render_error.reason),
            }
            ExitCode::from(2)
        }
    }
}
