use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::{ControlError, ControlResult};
use crate::headless_protocol::{
    HeadlessBridgePaths, HeadlessBridgeRequest, HeadlessBridgeResponse,
};
use crate::model::ControlSessionInput;

pub fn start_session(input: &ControlSessionInput) -> ControlResult<HeadlessBridgeResponse> {
    run_bridge("start", input, None)
}

pub fn seed_agent_session(
    input: &ControlSessionInput,
    prompt: &str,
) -> ControlResult<HeadlessBridgeResponse> {
    run_bridge("seed", input, Some(prompt))
}

#[allow(dead_code)]
pub fn session_status(input: &ControlSessionInput) -> ControlResult<HeadlessBridgeResponse> {
    run_bridge("status", input, None)
}

#[allow(dead_code)]
pub fn run_turn(
    input: &ControlSessionInput,
    prompt: &str,
) -> ControlResult<HeadlessBridgeResponse> {
    run_bridge("run", input, Some(prompt))
}

#[allow(dead_code)]
pub fn interrupt_turn(input: &ControlSessionInput) -> ControlResult<HeadlessBridgeResponse> {
    run_bridge("interrupt", input, None)
}

#[allow(dead_code)]
pub fn stop_session(input: &ControlSessionInput) -> ControlResult<HeadlessBridgeResponse> {
    run_bridge("stop", input, None)
}

pub fn run_bridge(
    operation: &str,
    input: &ControlSessionInput,
    prompt: Option<&str>,
) -> ControlResult<HeadlessBridgeResponse> {
    let paths = HeadlessBridgePaths::resolve(input)?;
    ensure_bridge_running(&paths)?;
    let request = paths.request(operation, prompt);
    let response = send_request(&request)?;
    if !response.ok {
        return Err(ControlError::HeadlessBridge {
            reason: response.details.clone(),
        });
    }
    Ok(response)
}

pub fn ensure_bridge_running(paths: &HeadlessBridgePaths) -> ControlResult<()> {
    if bridge_state_is_running(paths) {
        return Ok(());
    }
    spawn_bridge(paths)?;
    wait_until_running(paths)
}

fn bridge_state_is_running(paths: &HeadlessBridgePaths) -> bool {
    let Ok(state) = paths.read_state() else {
        return false;
    };
    if state.bridge_status.as_deref() != Some("running") || state.bridge_pid.is_none() {
        return false;
    }
    let (Some(host), Some(port)) = (state.bridge_host, state.bridge_port) else {
        return false;
    };
    let mut request = paths.request("ping", None);
    request.bridge_host = Some(host);
    request.bridge_port = Some(port);
    send_request(&request)
        .map(|response| response.ok && response.state == "running")
        .unwrap_or(false)
}

fn spawn_bridge(paths: &HeadlessBridgePaths) -> ControlResult<()> {
    let request = paths.request("daemon", None);
    let request_json =
        serde_json::to_string(&request).map_err(|error| ControlError::HeadlessBridge {
            reason: format!("failed to encode bridge daemon request: {error}"),
        })?;
    let mut command = Command::new(&paths.python);
    #[cfg(unix)]
    {
        command.process_group(0);
    }
    let child = command
        .arg(&paths.script)
        .arg("--daemon")
        .arg("--request-json")
        .arg(request_json)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| ControlError::HeadlessUnavailable {
            reason: format!("failed to spawn headless bridge daemon: {error}"),
        })?;
    drop(child);
    Ok(())
}

fn wait_until_running(paths: &HeadlessBridgePaths) -> ControlResult<()> {
    let deadline = Instant::now() + Duration::from_secs(20);
    while Instant::now() < deadline {
        if bridge_state_is_running(paths) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    Err(ControlError::HeadlessBridge {
        reason: format!(
            "headless bridge did not become ready for {}",
            paths.session_name
        ),
    })
}

fn send_request(request: &HeadlessBridgeRequest) -> ControlResult<HeadlessBridgeResponse> {
    let host = request
        .bridge_host
        .as_deref()
        .ok_or_else(|| ControlError::HeadlessBridge {
            reason: "headless bridge host missing after daemon start".to_owned(),
        })?;
    let port = request
        .bridge_port
        .ok_or_else(|| ControlError::HeadlessBridge {
            reason: "headless bridge port missing after daemon start".to_owned(),
        })?;
    let mut stream =
        TcpStream::connect((host, port)).map_err(|error| ControlError::HeadlessBridge {
            reason: format!("failed to connect to headless bridge {host}:{port}: {error}"),
        })?;
    let payload = serde_json::to_string(request).map_err(|error| ControlError::HeadlessBridge {
        reason: format!("failed to encode headless bridge request: {error}"),
    })?;
    stream
        .write_all(format!("{payload}\n").as_bytes())
        .map_err(|error| ControlError::HeadlessBridge {
            reason: format!("failed to write headless bridge request: {error}"),
        })?;
    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    reader
        .read_line(&mut line)
        .map_err(|error| ControlError::HeadlessBridge {
            reason: format!("failed to read headless bridge response: {error}"),
        })?;
    serde_json::from_str(line.trim()).map_err(|error| ControlError::HeadlessBridge {
        reason: format!("failed to parse headless bridge response: {error}; stdout={line}"),
    })
}
