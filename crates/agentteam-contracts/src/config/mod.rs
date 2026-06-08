use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "config.center";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigReq01TomlPath {
    pub path: String,
}

impl ConfigReq01TomlPath {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Config", "Req", 1, "TomlPath");

    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn read_as_raw(self, raw_toml: impl Into<String>) -> ConfigReq02TomlRaw {
        ConfigReq02TomlRaw {
            path: self.path,
            raw_toml: raw_toml.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigReq02TomlRaw {
    pub path: String,
    pub raw_toml: String,
}

impl ConfigReq02TomlRaw {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Config", "Req", 2, "TomlRaw");

    pub fn parse_as_document(self, document_id: impl Into<String>) -> ConfigReq03ParsedToml {
        ConfigReq03ParsedToml {
            path: self.path,
            document_id: document_id.into(),
            raw_bytes: self.raw_toml.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigReq03ParsedToml {
    pub path: String,
    pub document_id: String,
    pub raw_bytes: usize,
}

impl ConfigReq03ParsedToml {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Config", "Req", 3, "ParsedToml");

    pub fn validate_user_config(
        self,
        project_slug: impl Into<String>,
        local_domain_id: impl Into<String>,
    ) -> ConfigReq04ValidatedUserConfig {
        ConfigReq04ValidatedUserConfig {
            path: self.path,
            document_id: self.document_id,
            project_slug: project_slug.into(),
            local_domain_id: local_domain_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigReq04ValidatedUserConfig {
    pub path: String,
    pub document_id: String,
    pub project_slug: String,
    pub local_domain_id: String,
}

impl ConfigReq04ValidatedUserConfig {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Config", "Req", 4, "ValidatedUserConfig");

    pub fn normalize_runtime(self, runtime_home: impl Into<String>) -> ConfigResp05RuntimeConfig {
        ConfigResp05RuntimeConfig {
            path: self.path,
            project_slug: self.project_slug,
            local_domain_id: self.local_domain_id,
            runtime_home: runtime_home.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigResp05RuntimeConfig {
    pub path: String,
    pub project_slug: String,
    pub local_domain_id: String,
    pub runtime_home: String,
}

impl ConfigResp05RuntimeConfig {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Config", "Resp", 5, "RuntimeConfig");

    pub fn snapshot(self, snapshot_id: impl Into<String>) -> ConfigResp06Snapshot {
        ConfigResp06Snapshot {
            snapshot_id: snapshot_id.into(),
            project_slug: self.project_slug,
            local_domain_id: self.local_domain_id,
            runtime_home: self.runtime_home,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigResp06Snapshot {
    pub snapshot_id: String,
    pub project_slug: String,
    pub local_domain_id: String,
    pub runtime_home: String,
}

impl ConfigResp06Snapshot {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Config", "Resp", 6, "Snapshot");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigErr01Parse {
    pub path: String,
    pub reason: String,
}

impl ConfigErr01Parse {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Config", "Err", 1, "Parse");

    pub fn new(path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigErr02Validation {
    pub path: String,
    pub reason: String,
}

impl ConfigErr02Validation {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Config", "Err", 2, "Validation");

    pub fn new(path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_chain_uses_adjacent_nodes() {
        let snapshot = ConfigReq01TomlPath::new("docs/config/config.toml.example")
            .read_as_raw("[project]\nslug = 'agentteam'")
            .parse_as_document("doc-1")
            .validate_user_config("agentteam", "local")
            .normalize_runtime("/tmp/agentteam")
            .snapshot("snapshot-1");

        assert_eq!(snapshot.snapshot_id, "snapshot-1");
        assert_eq!(snapshot.project_slug, "agentteam");
        assert_eq!(ConfigReq01TomlPath::NODE.number, 1);
        assert_eq!(ConfigResp06Snapshot::NODE.number, 6);
    }

    #[test]
    fn config_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "config.center");
    }
}
