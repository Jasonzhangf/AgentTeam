use std::env;
use std::path::{Path, PathBuf};

use crate::config::default_config_path_missing_home;
use crate::error::{StartupError, StartupResult};

pub(crate) fn expand_default_config_path() -> StartupResult<String> {
    let home = env::var("HOME").map_err(default_config_path_missing_home)?;
    Ok(format!("{home}/.agentteam/config.toml"))
}

pub(crate) fn resolve_cwd(cwd: Option<String>) -> StartupResult<String> {
    let path = match cwd {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().map_err(|error| StartupError::Config {
            reason: format!("failed to read current directory: {error}"),
        })?,
    };
    path.canonicalize()
        .map_err(|error| StartupError::Config {
            reason: format!("failed to resolve cwd {}: {error}", path.display()),
        })
        .map(|path| path.display().to_string())
}

pub(crate) fn build_session_name(domain_id: &str, project_slug: &str, agent_name: &str) -> String {
    format!("TA_{domain_id}_{project_slug}_{agent_name}")
}

pub(crate) fn session_dir(project_slug: &str) -> StartupResult<String> {
    let home = env::var("HOME").map_err(|error| StartupError::Config {
        reason: format!("HOME is required to resolve session directory: {error}"),
    })?;
    Ok(format!("{home}/.agentteam/sessions/{project_slug}"))
}

pub(crate) fn expand_home_path(path: &str) -> StartupResult<String> {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = env::var("HOME").map_err(|error| StartupError::Config {
            reason: format!("HOME is required to expand {path}: {error}"),
        })?;
        return Ok(format!("{home}/{rest}"));
    }
    if path == "~" {
        return env::var("HOME").map_err(|error| StartupError::Config {
            reason: format!("HOME is required to expand {path}: {error}"),
        });
    }
    Ok(path.to_owned())
}

pub(crate) fn runtime_event_log_path(runtime_home: &str) -> PathBuf {
    Path::new(runtime_home)
        .join("events")
        .join("agentteam.jsonl")
}
