use agentteam_config::{
    load_config_file, parse_config_toml, validate_config, validate_config_path, ValidatedConfig,
};

use crate::error::{config_error, StartupError, StartupResult};
use crate::paths::expand_default_config_path;

pub(crate) fn load_validated_config(config_path: Option<String>) -> StartupResult<ValidatedConfig> {
    match config_path {
        Some(path) => validate_config_path(path).map_err(config_error),
        None => {
            let config_path = expand_default_config_path()?;
            let raw = load_config_file(config_path).map_err(config_error)?;
            let parsed = parse_config_toml(raw).map_err(config_error)?;
            validate_config(parsed).map_err(config_error)
        }
    }
}

pub(crate) fn default_config_path_missing_home(error: std::env::VarError) -> StartupError {
    StartupError::Config {
        reason: format!("HOME is required to resolve default config path: {error}"),
    }
}
