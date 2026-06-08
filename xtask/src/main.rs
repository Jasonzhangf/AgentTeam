use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

mod function_map;
mod red_tests;

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
    "docs/goals/mvp-runtime-vertical-slice-plan.md",
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
    "crates/agentteam-contracts/src/config/mod.rs",
    "crates/agentteam-contracts/src/debug/mod.rs",
    "crates/agentteam-contracts/src/domain/mod.rs",
    "crates/agentteam-contracts/src/error/mod.rs",
    "crates/agentteam-contracts/src/feature_map/mod.rs",
    "crates/agentteam-contracts/src/persist/mod.rs",
    "crates/agentteam-contracts/src/pipeline/mod.rs",
    "crates/agentteam-contracts/src/resource/mod.rs",
    "crates/agentteam-contracts/src/verification_map/mod.rs",
    "crates/agentteam-config/Cargo.toml",
    "crates/agentteam-config/src/error.rs",
    "crates/agentteam-config/src/lib.rs",
    "crates/agentteam-config/src/load.rs",
    "crates/agentteam-config/src/model.rs",
    "crates/agentteam-config/src/normalize.rs",
    "crates/agentteam-config/src/parse.rs",
    "crates/agentteam-config/src/snapshot.rs",
    "crates/agentteam-config/src/tests.rs",
    "crates/agentteam-config/src/validate.rs",
    "crates/agentteam-error/Cargo.toml",
    "crates/agentteam-error/src/classify.rs",
    "crates/agentteam-error/src/code.rs",
    "crates/agentteam-error/src/error.rs",
    "crates/agentteam-error/src/lib.rs",
    "crates/agentteam-error/src/model.rs",
    "crates/agentteam-error/src/persist.rs",
    "crates/agentteam-error/src/tests.rs",
    "crates/agentteam-comm/Cargo.toml",
    "crates/agentteam-comm/src/lib.rs",
    "crates/agentteam-debug/Cargo.toml",
    "crates/agentteam-debug/src/bundle.rs",
    "crates/agentteam-debug/src/error.rs",
    "crates/agentteam-debug/src/lib.rs",
    "crates/agentteam-debug/src/model.rs",
    "crates/agentteam-debug/src/persist.rs",
    "crates/agentteam-debug/src/tests.rs",
    "crates/agentteam-persist/Cargo.toml",
    "crates/agentteam-persist/src/append.rs",
    "crates/agentteam-persist/src/error.rs",
    "crates/agentteam-persist/src/lib.rs",
    "crates/agentteam-persist/src/materialize.rs",
    "crates/agentteam-persist/src/model.rs",
    "crates/agentteam-persist/src/replay.rs",
    "crates/agentteam-persist/src/tests.rs",
    "crates/agentteam-runtime/Cargo.toml",
    "crates/agentteam-runtime/src/domain/mod.rs",
    "crates/agentteam-runtime/src/domain/model.rs",
    "crates/agentteam-runtime/src/domain/registry.rs",
    "crates/agentteam-runtime/src/domain/resolve.rs",
    "crates/agentteam-runtime/src/domain/tests.rs",
    "crates/agentteam-runtime/src/lib.rs",
    "crates/agentteam-startup/Cargo.toml",
    "crates/agentteam-startup/src/lib.rs",
    "crates/agentteam-tanote/Cargo.toml",
    "crates/agentteam-tanote/src/lib.rs",
    "crates/agentteam-resource/Cargo.toml",
    "crates/agentteam-resource/src/error.rs",
    "crates/agentteam-resource/src/lib.rs",
    "crates/agentteam-resource/src/model.rs",
    "crates/agentteam-resource/src/persist.rs",
    "crates/agentteam-resource/src/registry.rs",
    "crates/agentteam-resource/src/snapshot.rs",
    "crates/agentteam-resource/src/tests.rs",
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
    "xtask/src/function_map.rs",
    "xtask/src/main.rs",
    "xtask/src/red_tests.rs",
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
        eprintln!("usage: cargo xtask <verify|red-tests|verify-required-files|verify-skill-frontmatter|verify-resource-lifecycle|verify-function-map|verify-code-size>");
        return ExitCode::from(2);
    };

    let result = match command.as_str() {
        "verify" => verify_all(),
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

fn verify_all() -> Result<(), String> {
    verify_function_map()?;
    run_command("cargo", &["fmt", "--check"])?;
    run_command(
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_command("cargo", &["test", "--workspace"])?;
    red_tests()?;
    verify_required_files()?;
    verify_skill_frontmatter()?;
    verify_resource_lifecycle()?;
    verify_code_size()
}

fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|error| format!("failed to run {program} {}: {error}", args.join(" ")))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "{program} {} failed with status {status}",
            args.join(" ")
        ))
    }
}

fn red_tests() -> Result<(), String> {
    red_tests::run()
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
    function_map::run(MODULE_DOCS)
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
