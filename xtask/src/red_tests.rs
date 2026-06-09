use std::fs;
use std::path::Path;

const BROAD_KILL_PATTERNS: &[&str] = &["pkill", "killall", "kill $(", "xargs kill"];

const AGENT_FACING_DOCS: &[&str] = &[
    ".agents/skills/agentteam/SKILL.md",
    "docs/modules/13-cli-agent-skill.md",
    "docs/tanote/TANote.md.example",
];

const AGENT_INTERNAL_LEAK_PATTERNS: &[&str] = &[
    "tmux session name",
    "pane id",
    "zterm endpoint",
    "session descriptor path",
];

const RUST_ROOTS: &[&str] = &["crates", "xtask/src"];

pub fn run() -> Result<(), String> {
    require_plan_entries()?;
    scan_broad_kill_patterns()?;
    scan_agent_facing_internal_leaks()?;
    scan_toml_parsing_owner()?;
    scan_state_file_write_owner()?;
    scan_domain_owner_boundaries()?;
    scan_non_adjacent_pipeline_conversions()?;
    scan_contract_feature_ids()?;
    scan_configured_agent_name_concepts()?;
    Ok(())
}

fn require_plan_entries() -> Result<(), String> {
    let plan = read("docs/red-tests/red-test-plan.md")?;
    for required in [
        "red.debug.not_persisted",
        "red.resource.no_lease",
        "red.resource.temp_left_after_shutdown",
        "red.manager.skill_missing_ops",
        "red.required_file_untracked",
        "red.registry.sample_agent_name_as_code_concept",
        "red.domain.remote_fallback_to_local",
        "red.domain.comm_parses_domain_directly",
        "red.config.parse_outside_config_center",
        "red.persist.direct_state_write",
        "red.gateway.non_adjacent_conversion",
        "red.cli.broad_kill_doc",
        "red.agent.exposes_tmux_session",
        "red.adapter.resolves_domain_target",
    ] {
        require_contains("docs/red-tests/red-test-plan.md", &plan, required)?;
    }
    Ok(())
}

fn scan_broad_kill_patterns() -> Result<(), String> {
    let mut violations = Vec::new();
    scan_text_files(Path::new("."), &mut |path, content| {
        if ignored_path(path) {
            return;
        }
        for pattern in BROAD_KILL_PATTERNS {
            if content.contains(pattern) && !allowed_broad_kill_reference(path) {
                violations.push(format!("{} contains {pattern:?}", path.display()));
            }
        }
    })?;
    no_violations("broad kill patterns are forbidden", violations)
}

fn scan_agent_facing_internal_leaks() -> Result<(), String> {
    let mut violations = Vec::new();
    for path in AGENT_FACING_DOCS {
        let content = read(path)?;
        for pattern in AGENT_INTERNAL_LEAK_PATTERNS {
            if content.contains(pattern) && !contains_forbidden_context(&content, pattern) {
                violations.push(format!("{path} exposes {pattern:?} without forbidding it"));
            }
        }
    }
    no_violations(
        "agent-facing docs must hide transport internals",
        violations,
    )
}

fn scan_toml_parsing_owner() -> Result<(), String> {
    let mut violations = Vec::new();
    scan_rust_files(&mut |path, content| {
        if path.starts_with("crates/agentteam-config/") || path.starts_with("xtask/") {
            return;
        }
        for pattern in ["toml::", "from_str::<", "parse_toml"] {
            if content.contains(pattern) {
                violations.push(format!(
                    "{} parses TOML outside Config Center",
                    path.display()
                ));
            }
        }
    })?;
    no_violations("TOML parsing belongs to Config Center", violations)
}

fn scan_state_file_write_owner() -> Result<(), String> {
    let mut violations = Vec::new();
    scan_rust_files(&mut |path, content| {
        if path.starts_with("crates/agentteam-persist/") || path.starts_with("xtask/") {
            return;
        }
        for pattern in ["fs::write", "File::create", "OpenOptions::new"] {
            if content.contains(pattern) {
                violations.push(format!(
                    "{} writes state-like files outside Persistence",
                    path.display()
                ));
            }
        }
    })?;
    no_violations("state file writes belong to Persistence", violations)
}

fn scan_domain_owner_boundaries() -> Result<(), String> {
    let mut violations = Vec::new();
    for forbidden_owner in [
        ("crates/agentteam-comm/", "agent@domain"),
        ("crates/agentteam-tmux/", "agent@domain"),
        ("crates/agentteam-comm/", "split('@')"),
        ("crates/agentteam-tmux/", "split('@')"),
    ] {
        let (root, pattern) = forbidden_owner;
        scan_text_files(Path::new(root), &mut |path, content| {
            if content.contains(pattern) {
                violations.push(format!(
                    "{} contains forbidden domain parsing pattern {pattern:?}",
                    path.display()
                ));
            }
        })?;
    }
    no_violations(
        "domain parsing belongs to Daemon Domain Registry",
        violations,
    )
}

fn scan_non_adjacent_pipeline_conversions() -> Result<(), String> {
    let mut violations = Vec::new();
    scan_rust_files(&mut |path, content| {
        if path.starts_with("xtask/") {
            return;
        }
        for pattern in ["impl From<", "impl TryFrom<"] {
            if content.contains(pattern) {
                violations.push(format!(
                    "{} contains forbidden pipeline conversion pattern {pattern:?}",
                    path.display()
                ));
            }
        }
    })?;
    no_violations(
        "pipeline conversions require explicit adjacent builders",
        violations,
    )
}

fn scan_contract_feature_ids() -> Result<(), String> {
    let features = read("crates/agentteam-contracts/src/feature_map/mod.rs")?;
    let mut violations = Vec::new();
    scan_text_files(
        Path::new("crates/agentteam-contracts/src"),
        &mut |path, content| {
            if path.ends_with("feature_map/mod.rs") {
                return;
            }
            if let Some(feature_id) = content.lines().find_map(extract_declared_feature_id) {
                if !features.contains(feature_id) {
                    violations.push(format!(
                        "{} declares unknown feature_id {feature_id:?}",
                        path.display()
                    ));
                }
            }
        },
    )?;
    no_violations("contract feature ids must be in feature map", violations)
}

fn scan_configured_agent_name_concepts() -> Result<(), String> {
    let mut violations = Vec::new();
    let declaration_prefixes = [
        "fn ",
        "pub fn ",
        "pub(crate) fn ",
        "pub(super) fn ",
        "struct ",
        "pub struct ",
        "enum ",
        "pub enum ",
        "trait ",
        "pub trait ",
        "type ",
        "pub type ",
        "const ",
        "pub const ",
        "static ",
        "pub static ",
        "mod ",
        "pub mod ",
    ];
    scan_rust_files(&mut |path, content| {
        for (line_index, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if !trimmed.to_ascii_lowercase().contains("kevin") {
                continue;
            }
            let field_candidate = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
            let field_name = field_candidate.split(':').next().unwrap_or_default().trim();
            let is_field_name = !field_name.is_empty()
                && field_name
                    .chars()
                    .all(|char| char == '_' || char.is_ascii_alphanumeric());
            if declaration_prefixes
                .iter()
                .any(|prefix| trimmed.starts_with(prefix))
                || (is_field_name && field_name.to_ascii_lowercase().contains("kevin"))
            {
                violations.push(format!(
                    "{}:{} declares sample agent name as a Rust concept",
                    path.display(),
                    line_index + 1
                ));
            }
        }
    })?;

    for path in [
        "docs/architecture/function-map.md",
        "docs/red-tests/red-test-plan.md",
    ] {
        let content = read(path)?;
        for (line_index, line) in content.lines().enumerate() {
            if !line.trim_start().starts_with('|') {
                continue;
            }
            let Some(first_tick) = line.find('`') else {
                continue;
            };
            let rest = &line[first_tick + 1..];
            let Some(second_tick) = rest.find('`') else {
                continue;
            };
            let id = &rest[..second_tick];
            if id.to_ascii_lowercase().contains("kevin") {
                violations.push(format!(
                    "{path}:{} uses sample agent name in a registry id",
                    line_index + 1
                ));
            }
        }
    }

    no_violations(
        "configured sample agent names must not become code concepts",
        violations,
    )
}

fn extract_declared_feature_id(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let marker = "FEATURE_ID";
    if !trimmed.contains(marker) {
        return None;
    }
    let first_quote = trimmed.find('"')?;
    let rest = &trimmed[first_quote + 1..];
    let second_quote = rest.find('"')?;
    Some(&rest[..second_quote])
}

fn scan_rust_files(visitor: &mut dyn FnMut(&Path, &str)) -> Result<(), String> {
    for root in RUST_ROOTS {
        scan_text_files(Path::new(root), &mut |path, content| {
            if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                visitor(path, content);
            }
        })?;
    }
    Ok(())
}

fn scan_text_files(path: &Path, visitor: &mut dyn FnMut(&Path, &str)) -> Result<(), String> {
    if path.is_file() {
        if is_text_candidate(path) {
            let content = fs::read_to_string(path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            visitor(path, &content);
        }
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)
            .map_err(|error| format!("failed to read dir {}: {error}", path.display()))?
        {
            let entry = entry.map_err(|error| format!("failed to read dir entry: {error}"))?;
            scan_text_files(&entry.path(), visitor)?;
        }
    }

    Ok(())
}

fn is_text_candidate(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rs" | "md" | "toml" | "txt" | "yaml" | "yml")
    )
}

fn ignored_path(path: &Path) -> bool {
    path.starts_with("target") || path.starts_with(".git")
}

fn allowed_broad_kill_reference(path: &Path) -> bool {
    let allowed = [
        "AGENTS.md",
        "docs/red-tests/red-test-plan.md",
        "docs/modules/13-cli-agent-skill.md",
        "docs/modules/15-startup-session-manager.md",
        "docs/modules/17-resource-lifecycle-manager.md",
        "docs/modules/12-zterm-tmux-adapter.md",
        ".agents/skills/agentteam/SKILL.md",
        ".agents/skills/agentteam-dev/SKILL.md",
        "xtask/src/red_tests.rs",
    ];
    let normalized = normalize_repo_path(path);
    allowed
        .iter()
        .any(|allowed_path| normalized == Path::new(allowed_path))
}

fn normalize_repo_path(path: &Path) -> &Path {
    path.strip_prefix(".").unwrap_or(path)
}

fn contains_forbidden_context(content: &str, pattern: &str) -> bool {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .all(|line| {
            line.contains("must not")
                || line.contains("must hide")
                || line.contains("Do not")
                || line.contains("not expose")
                || line.contains("hidden")
                || line.contains("Forbidden")
                || line.contains("fails")
        })
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

fn no_violations(title: &str, violations: Vec<String>) -> Result<(), String> {
    if violations.is_empty() {
        Ok(())
    } else {
        Err(format!("{title}:\n{}", violations.join("\n")))
    }
}
