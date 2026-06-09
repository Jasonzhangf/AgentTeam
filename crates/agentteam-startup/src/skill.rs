use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{StartupError, StartupResult};

pub(crate) const INSTALLED_SKILL_PATH: &str = ".agents/skills/agentteam/SKILL.md";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SkillInstallOutcome {
    pub skill_path: String,
    pub cli_path: String,
    pub status: String,
}

pub(crate) fn install_agentteam_skill(cwd: &str) -> StartupResult<SkillInstallOutcome> {
    let source = source_skill_path()?;
    let target = Path::new(cwd).join(INSTALLED_SKILL_PATH);
    let source_content =
        fs::read_to_string(&source).map_err(|error| StartupError::SkillInstall {
            reason: format!("failed to read source skill {}: {error}", source.display()),
        })?;
    let existing = fs::read_to_string(&target).ok();
    let status = if existing.as_deref() == Some(source_content.as_str()) {
        "already_installed".to_owned()
    } else {
        let parent = target.parent().ok_or_else(|| StartupError::SkillInstall {
            reason: format!("skill target has no parent: {}", target.display()),
        })?;
        fs::create_dir_all(parent).map_err(|error| StartupError::SkillInstall {
            reason: format!("failed to create skill dir {}: {error}", parent.display()),
        })?;
        fs::write(&target, source_content).map_err(|error| StartupError::SkillInstall {
            reason: format!("failed to install skill {}: {error}", target.display()),
        })?;
        "installed".to_owned()
    };
    Ok(SkillInstallOutcome {
        skill_path: target.display().to_string(),
        cli_path: current_cli_path()?,
        status,
    })
}

fn source_skill_path() -> StartupResult<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(|repo| repo.join(INSTALLED_SKILL_PATH))
        .ok_or_else(|| StartupError::SkillInstall {
            reason: "failed to resolve repository root for local skill".to_owned(),
        })?;
    if source.exists() {
        Ok(source)
    } else {
        Err(StartupError::SkillInstall {
            reason: format!("source skill does not exist: {}", source.display()),
        })
    }
}

fn current_cli_path() -> StartupResult<String> {
    env::current_exe()
        .map_err(|error| StartupError::SkillInstall {
            reason: format!("failed to resolve current executable: {error}"),
        })
        .map(|path| path.display().to_string())
}
