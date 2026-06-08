mod error;
mod load;
mod model;
mod normalize;
mod parse;
mod snapshot;
#[cfg(test)]
mod tests;
mod validate;

pub use error::{ConfigCenterError, ConfigCenterResult};
pub use load::{load_config_file, load_default_config_file};
pub use model::{
    DaemonDomainConfig, MemberConfig, NormalizedConfig, ProjectConfig, RemoteDaemonConfig,
    RuntimeConfig, TeamConfig, TmuxConfig, UserConfig, ZtermConfig,
};
pub use normalize::normalize_config;
pub use parse::parse_config_toml;
pub use snapshot::{snapshot_config, ConfigSnapshot};
pub use validate::validate_config;

pub const FEATURE_ID: &str = "config.center";

pub fn check_config_path(path: impl Into<String>) -> ConfigCenterResult<NormalizedConfig> {
    let raw = load_config_file(path.into())?;
    let parsed = parse_config_toml(raw)?;
    let validated = validate_config(parsed)?;
    normalize_config(validated)
}
