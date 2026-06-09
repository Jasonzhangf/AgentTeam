use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{ControlError, ControlResult};
use crate::model::ControlSessionInput;

pub const SDK_SRC_ENV: &str = "AGENTTEAM_CODEX_SDK_SRC";
pub const CODEX_BIN_ENV: &str = "AGENTTEAM_CODEX_BIN";

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct HeadlessBridgeResponse {
    pub ok: bool,
    pub operation: String,
    pub session_name: String,
    pub project_slug: String,
    pub thread_id: Option<String>,
    pub turn_id: Option<String>,
    pub state: String,
    pub details: String,
    pub active_flags: Option<Vec<String>>,
    pub final_response: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeadlessBridgeRequest {
    pub operation: String,
    pub sdk_src: String,
    pub codex_bin: String,
    pub cwd: String,
    pub project_slug: String,
    pub state_file: String,
    pub session_name: String,
    pub bridge_host: Option<String>,
    pub bridge_port: Option<u16>,
    pub prompt: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HeadlessBridgePaths {
    pub python: String,
    pub script: PathBuf,
    pub sdk_src: PathBuf,
    pub codex_bin: PathBuf,
    pub cwd: PathBuf,
    pub project_slug: String,
    pub state_file: PathBuf,
    pub session_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HeadlessBridgeState {
    pub bridge_host: Option<String>,
    pub bridge_port: Option<u16>,
    pub bridge_pid: Option<u32>,
    pub bridge_status: Option<String>,
    pub thread_id: Option<String>,
}

impl HeadlessBridgePaths {
    pub fn resolve(input: &ControlSessionInput) -> ControlResult<Self> {
        let python = env::var("PYTHON_BIN").unwrap_or_else(|_| "python3".to_owned());
        let sdk_src = PathBuf::from(env::var(SDK_SRC_ENV).map_err(|_| {
            ControlError::HeadlessUnavailable {
                reason: format!("{SDK_SRC_ENV} is required for headless bridge"),
            }
        })?);
        require_existing_path(&sdk_src, "headless SDK source path")?;
        let codex_bin = PathBuf::from(env::var(CODEX_BIN_ENV).map_err(|_| {
            ControlError::HeadlessUnavailable {
                reason: format!("{CODEX_BIN_ENV} is required for headless bridge"),
            }
        })?);
        require_existing_path(&codex_bin, "codex binary")?;
        let cwd = input
            .cwd
            .as_ref()
            .map(PathBuf::from)
            .map(Ok)
            .unwrap_or_else(|| {
                env::current_dir().map_err(|error| ControlError::HeadlessUnavailable {
                    reason: format!("failed to read current_dir: {error}"),
                })
            })?;
        let project_slug = input
            .project_slug
            .clone()
            .unwrap_or_else(|| project_slug_from_cwd(&cwd));
        let session_dir = headless_session_dir(&project_slug, &input.session_name)?;
        let state_file = session_dir.join("state.json");
        let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("headless_bridge.py");
        require_existing_path(&script, "headless bridge script")?;
        Ok(Self {
            python,
            script,
            sdk_src,
            codex_bin,
            cwd,
            project_slug,
            state_file,
            session_name: input.session_name.clone(),
        })
    }

    pub fn request(&self, operation: &str, prompt: Option<&str>) -> HeadlessBridgeRequest {
        let state = self.read_state().ok();
        let bridge_host = state.as_ref().and_then(|state| state.bridge_host.clone());
        let bridge_port = state.as_ref().and_then(|state| state.bridge_port);
        HeadlessBridgeRequest {
            operation: operation.to_owned(),
            sdk_src: self.sdk_src.to_string_lossy().into_owned(),
            codex_bin: self.codex_bin.to_string_lossy().into_owned(),
            cwd: self.cwd.to_string_lossy().into_owned(),
            project_slug: self.project_slug.clone(),
            state_file: self.state_file.to_string_lossy().into_owned(),
            session_name: self.session_name.clone(),
            bridge_host,
            bridge_port,
            prompt: prompt.map(str::to_owned),
        }
    }

    pub fn read_state(&self) -> ControlResult<HeadlessBridgeState> {
        let content =
            fs::read_to_string(&self.state_file).map_err(|error| ControlError::HeadlessBridge {
                reason: format!(
                    "failed to read headless bridge state {}: {error}",
                    self.state_file.display()
                ),
            })?;
        serde_json::from_str(&content).map_err(|error| ControlError::HeadlessBridge {
            reason: format!("failed to parse headless bridge state: {error}"),
        })
    }
}

pub fn project_slug_from_cwd(cwd: &Path) -> String {
    cwd.file_name()
        .and_then(|value| value.to_str())
        .map(sanitize_session_name)
        .unwrap_or_else(|| "workspace".to_owned())
}

fn headless_session_dir(project_slug: &str, session_name: &str) -> ControlResult<PathBuf> {
    let home = env::var_os("HOME").ok_or_else(|| ControlError::HeadlessUnavailable {
        reason: "HOME is required to store headless session state".to_owned(),
    })?;
    let mut path = PathBuf::from(home);
    path.push(".agentteam");
    path.push("sessions");
    path.push(project_slug);
    path.push("headless");
    path.push(sanitize_session_name(session_name));
    fs::create_dir_all(&path).map_err(|error| ControlError::HeadlessUnavailable {
        reason: format!(
            "failed to create headless state dir {}: {error}",
            path.display()
        ),
    })?;
    Ok(path)
}

pub fn sanitize_session_name(session_name: &str) -> String {
    session_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn require_existing_path(path: &Path, label: &str) -> ControlResult<()> {
    if path.exists() {
        Ok(())
    } else {
        Err(ControlError::HeadlessUnavailable {
            reason: format!("{label} does not exist: {}", path.display()),
        })
    }
}
