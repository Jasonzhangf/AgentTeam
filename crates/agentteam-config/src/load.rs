use crate::error::{ConfigCenterError, ConfigCenterResult};
use agentteam_contracts::config::{ConfigReq01TomlPath, ConfigReq02TomlRaw};
use std::fs;

pub const DEFAULT_CONFIG_PATH: &str = "~/.agentteam/config.toml";

pub fn load_default_config_file() -> ConfigCenterResult<ConfigReq02TomlRaw> {
    load_config_file(DEFAULT_CONFIG_PATH)
}

pub fn load_config_file(path: impl Into<String>) -> ConfigCenterResult<ConfigReq02TomlRaw> {
    let path = path.into();
    let raw_toml = fs::read_to_string(&path).map_err(|error| ConfigCenterError::Load {
        path: path.clone(),
        reason: error.to_string(),
    })?;
    Ok(ConfigReq01TomlPath::new(path).read_as_raw(raw_toml))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_is_explicit_load_error() {
        let error = load_config_file("/tmp/agentteam-missing-config.toml").unwrap_err();
        assert_eq!(error.path(), "/tmp/agentteam-missing-config.toml");
        assert!(matches!(error, ConfigCenterError::Load { .. }));
    }
}
