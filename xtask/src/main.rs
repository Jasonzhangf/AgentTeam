use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

const REQUIRED_FILES: &[&str] = &[
    "AGENTS.md",
    ".gitignore",
    "README.md",
    "Cargo.toml",
    "Cargo.lock",
    "CACHE.md",
    "MEMORY.md",
    "note.md",
    "docs/architecture/overview.md",
    "docs/architecture/ascii-flows.md",
    "docs/architecture/file-structure.md",
    "docs/architecture/function-map.md",
    "docs/architecture/verification-map.md",
    "docs/architecture/mvp-debug-build.md",
    "docs/architecture/mvp-start-gate.md",
    "docs/architecture/function-map-gate.md",
    "docs/architecture/code-size-policy.md",
    "docs/config/config.toml.example",
    "docs/flows/build-regression-flow.md",
    "docs/modules/00-module-discussion-index.md",
    "docs/modules/01-config-center.md",
    "docs/modules/02-error-center.md",
    "docs/modules/03-communication-center.md",
    "docs/modules/04-input-gateway.md",
    "docs/modules/05-output-gateway.md",
    "docs/modules/06-ui-gateway.md",
    "docs/modules/07-agent-registry-naming-pool.md",
    "docs/modules/08-team-orchestrator.md",
    "docs/modules/09-task-engine.md",
    "docs/modules/10-debug-center.md",
    "docs/modules/11-persistence-event-log.md",
    "docs/modules/12-zterm-tmux-adapter.md",
    "docs/modules/13-cli-agent-skill.md",
    "docs/modules/14-tui-agent-adapter-center.md",
    "docs/modules/15-startup-session-manager.md",
    "docs/modules/16-tanote-collaboration-board.md",
    "docs/modules/17-resource-lifecycle-manager.md",
    "docs/modules/18-daemon-domain-registry.md",
    "docs/red-tests/red-test-plan.md",
    "docs/spec.md",
    "docs/tanote/TANote.md.example",
    ".agents/skills/agentteam/SKILL.md",
    ".agents/skills/agentteam-dev/SKILL.md",
    "crates/agentteam-contracts/Cargo.toml",
    "crates/agentteam-contracts/src/lib.rs",
    "crates/agentteam-config/Cargo.toml",
    "crates/agentteam-config/src/lib.rs",
    "crates/agentteam-error/Cargo.toml",
    "crates/agentteam-error/src/lib.rs",
    "crates/agentteam-comm/Cargo.toml",
    "crates/agentteam-comm/src/lib.rs",
    "crates/agentteam-debug/Cargo.toml",
    "crates/agentteam-debug/src/lib.rs",
    "crates/agentteam-persist/Cargo.toml",
    "crates/agentteam-persist/src/lib.rs",
    "crates/agentteam-runtime/Cargo.toml",
    "crates/agentteam-runtime/src/lib.rs",
    "crates/agentteam-startup/Cargo.toml",
    "crates/agentteam-startup/src/lib.rs",
    "crates/agentteam-tanote/Cargo.toml",
    "crates/agentteam-tanote/src/lib.rs",
    "crates/agentteam-resource/Cargo.toml",
    "crates/agentteam-resource/src/lib.rs",
    "crates/agentteam-tmux/Cargo.toml",
    "crates/agentteam-tmux/src/lib.rs",
    "crates/agentteam-tui-adapter/Cargo.toml",
    "crates/agentteam-tui-adapter/src/lib.rs",
    "crates/agentteam-gateway/Cargo.toml",
    "crates/agentteam-gateway/src/lib.rs",
    "crates/agentteam-cli/Cargo.toml",
    "crates/agentteam-cli/src/main.rs",
    "crates/agentteamd/Cargo.toml",
    "crates/agentteamd/src/main.rs",
    "xtask/Cargo.toml",
    "xtask/src/main.rs",
    ".cargo/config.toml",
];

const MODULE_DOCS: &[&str] = &[
    "docs/modules/01-config-center.md",
    "docs/modules/02-error-center.md",
    "docs/modules/03-communication-center.md",
    "docs/modules/04-input-gateway.md",
    "docs/modules/05-output-gateway.md",
    "docs/modules/06-ui-gateway.md",
    "docs/modules/07-agent-registry-naming-pool.md",
    "docs/modules/08-team-orchestrator.md",
    "docs/modules/09-task-engine.md",
    "docs/modules/10-debug-center.md",
    "docs/modules/11-persistence-event-log.md",
    "docs/modules/12-zterm-tmux-adapter.md",
    "docs/modules/13-cli-agent-skill.md",
    "docs/modules/14-tui-agent-adapter-center.md",
    "docs/modules/15-startup-session-manager.md",
    "docs/modules/16-tanote-collaboration-board.md",
    "docs/modules/17-resource-lifecycle-manager.md",
    "docs/modules/18-daemon-domain-registry.md",
];

fn main() -> ExitCode {
    let Some(command) = env::args().nth(1) else {
        eprintln!("usage: cargo xtask <red-tests|verify-required-files|verify-skill-frontmatter|verify-resource-lifecycle|verify-function-map|verify-code-size>");
        return ExitCode::from(2);
    };

    let result = match command.as_str() {
        "red-tests" => red_tests(),
        "verify-required-files" => verify_required_files(),
        "verify-skill-frontmatter" => verify_skill_frontmatter(),
        "verify-resource-lifecycle" => verify_resource_lifecycle(),
        "verify-function-map" => verify_function_map(),
        "verify-code-size" => verify_code_size(),
        other => Err(format!("unknown xtask command: {other}")),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

fn red_tests() -> Result<(), String> {
    require_file("docs/red-tests/red-test-plan.md")?;
    let plan = read("docs/red-tests/red-test-plan.md")?;
    for required in [
        "red.debug.not_persisted",
        "red.resource.no_lease",
        "red.resource.temp_left_after_shutdown",
        "red.kevin.skill_missing_ops",
        "red.required_file_untracked",
        "red.domain.remote_fallback_to_local",
        "red.domain.comm_parses_domain_directly",
    ] {
        require_contains("docs/red-tests/red-test-plan.md", &plan, required)?;
    }
    Ok(())
}

fn verify_required_files() -> Result<(), String> {
    for file in REQUIRED_FILES {
        require_file(file)?;
    }

    let output = Command::new("git")
        .args(["ls-files", "--error-unmatch"])
        .args(REQUIRED_FILES)
        .output()
        .map_err(|error| format!("failed to run git ls-files: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "required files must be tracked before build gate passes:\n{stderr}"
        ));
    }

    Ok(())
}

fn verify_skill_frontmatter() -> Result<(), String> {
    let root = Path::new(".agents/skills");
    for entry in fs::read_dir(root).map_err(|error| {
        format!(
            "failed to read local skills directory {}: {error}",
            root.display()
        )
    })? {
        let entry = entry.map_err(|error| format!("failed to read skill dir entry: {error}"))?;
        let path = entry.path().join("SKILL.md");
        if path.is_file() {
            verify_one_skill_frontmatter(&path)?;
        }
    }
    Ok(())
}

fn verify_one_skill_frontmatter(path: &Path) -> Result<(), String> {
    let path_display = path.display().to_string();
    let skill = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {path_display}: {error}"))?;
    let mut lines = skill.lines();
    if lines.next() != Some("---") {
        return Err(format!("{path_display} is missing YAML frontmatter start"));
    }
    let mut closed = false;
    let mut has_name = false;
    let mut has_description = false;
    for line in lines {
        if line == "---" {
            closed = true;
            break;
        }
        has_name |= line.starts_with("name:");
        has_description |= line.starts_with("description:");
    }
    if !closed || !has_name || !has_description {
        return Err(format!(
            "{path_display} frontmatter must include name, description, and closing delimiter"
        ));
    }
    Ok(())
}

fn verify_resource_lifecycle() -> Result<(), String> {
    for module in MODULE_DOCS {
        let doc = read(module)?;
        require_contains(module, &doc, "## Resource Lifecycle")?;
    }

    let resource_doc = read("docs/modules/17-resource-lifecycle-manager.md")?;
    for required in [
        "temporary_file",
        "unbounded growth",
        "scoped shutdown",
        "cleanup tracked temporary files",
    ] {
        require_contains(
            "docs/modules/17-resource-lifecycle-manager.md",
            &resource_doc,
            required,
        )?;
    }

    let debug_doc = read("docs/modules/10-debug-center.md")?;
    require_contains(
        "docs/modules/10-debug-center.md",
        &debug_doc,
        "persistence receipt",
    )?;

    Ok(())
}

fn verify_function_map() -> Result<(), String> {
    let function_map = read("docs/architecture/function-map.md")?;
    let verification_map = read("docs/architecture/verification-map.md")?;
    let contracts_map = read("crates/agentteam-contracts/src/feature_map/mod.rs")?;

    for feature_id in [
        "config.center",
        "error.center",
        "comm.center",
        "domain.registry",
        "gateway.input",
        "gateway.output",
        "gateway.ui",
        "agent.naming_pool",
        "team.orchestration",
        "task.engine",
        "debug.center",
        "persist.event_log",
        "adapter.zterm_tmux",
        "adapter.tui_agent",
        "startup.session",
        "tanote.board",
        "resource.lifecycle",
        "cli.agent_skill",
    ] {
        require_contains(
            "docs/architecture/function-map.md",
            &function_map,
            feature_id,
        )?;
        require_contains(
            "docs/architecture/verification-map.md",
            &verification_map,
            feature_id,
        )?;
        require_contains(
            "crates/agentteam-contracts/src/feature_map/mod.rs",
            &contracts_map,
            feature_id,
        )?;
    }

    for module in MODULE_DOCS {
        let doc = read(module)?;
        require_contains(module, &doc, "## Module Function Map")?;
        require_contains(module, &doc, "## Module Help Contract")?;
    }

    Ok(())
}

fn verify_code_size() -> Result<(), String> {
    let mut oversized = Vec::new();
    for root in ["crates", "xtask/src"] {
        collect_oversized_rust_files(Path::new(root), &mut oversized)?;
    }

    if oversized.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Rust source files exceed 500 lines:\n{}",
            oversized.join("\n")
        ))
    }
}

fn require_file(path: &str) -> Result<(), String> {
    if Path::new(path).is_file() {
        Ok(())
    } else {
        Err(format!("required file is missing: {path}"))
    }
}

fn read(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {path}: {error}"))
}

fn require_contains(path: &str, haystack: &str, needle: &str) -> Result<(), String> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        Err(format!("{path} must contain {needle:?}"))
    }
}

fn collect_oversized_rust_files(path: &Path, oversized: &mut Vec<String>) -> Result<(), String> {
    if path.is_file() {
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            let content = fs::read_to_string(path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let lines = content.lines().count();
            if lines > 500 {
                oversized.push(format!("{}: {lines} lines", path.display()));
            }
        }
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)
            .map_err(|error| format!("failed to read dir {}: {error}", path.display()))?
        {
            let entry = entry.map_err(|error| format!("failed to read dir entry: {error}"))?;
            collect_oversized_rust_files(&entry.path(), oversized)?;
        }
    }

    Ok(())
}
