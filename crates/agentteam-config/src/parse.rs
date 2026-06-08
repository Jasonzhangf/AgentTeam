use crate::error::{ConfigCenterError, ConfigCenterResult};
use crate::model::UserConfig;
use agentteam_contracts::config::{ConfigReq02TomlRaw, ConfigReq03ParsedToml};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedConfig {
    pub node: ConfigReq03ParsedToml,
    pub user_config: UserConfig,
}

pub fn parse_config_toml(raw: ConfigReq02TomlRaw) -> ConfigCenterResult<ParsedConfig> {
    let user_config =
        toml::from_str::<UserConfig>(&raw.raw_toml).map_err(|error| ConfigCenterError::Parse {
            path: raw.path.clone(),
            reason: error.to_string(),
        })?;
    let document_id = format!("config-doc-{}", raw.raw_toml.len());
    let node = raw.parse_as_document(document_id);
    Ok(ParsedConfig { node, user_config })
}

#[cfg(test)]
mod tests {
    use super::*;
    use agentteam_contracts::config::ConfigReq01TomlPath;

    #[test]
    fn malformed_toml_is_explicit_parse_error() {
        let raw = ConfigReq01TomlPath::new("inline").read_as_raw("[project");
        let error = parse_config_toml(raw).unwrap_err();
        assert!(matches!(error, ConfigCenterError::Parse { .. }));
    }
}
