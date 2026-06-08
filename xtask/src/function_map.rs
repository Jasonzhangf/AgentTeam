use std::fs;
use std::path::Path;

const FEATURE_IDS: &[&str] = &[
    "architecture.gate",
    "contract.pipeline",
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
];

const FUNCTION_SCAN_ROOTS: &[&str] = &["crates", "xtask/src"];

pub fn run(module_docs: &[&str]) -> Result<(), String> {
    verify_feature_ids()?;
    verify_module_docs(module_docs)?;
    verify_rust_function_registry()
}

fn verify_feature_ids() -> Result<(), String> {
    let function_map = read("docs/architecture/function-map.md")?;
    let verification_map = read("docs/architecture/verification-map.md")?;
    let contracts_map = read("crates/agentteam-contracts/src/feature_map/mod.rs")?;

    for feature_id in FEATURE_IDS {
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

    Ok(())
}

fn verify_module_docs(module_docs: &[&str]) -> Result<(), String> {
    for module in module_docs {
        let doc = read(module)?;
        require_contains(module, &doc, "## Module Function Map")?;
        require_contains(module, &doc, "## Module Help Contract")?;
    }
    Ok(())
}

fn verify_rust_function_registry() -> Result<(), String> {
    let function_map = read("docs/architecture/function-map.md")?;
    let symbols = collect_rust_function_symbols()?;
    let missing: Vec<String> = symbols
        .into_iter()
        .filter(|symbol| !function_map.contains(&format!("`{symbol}`")))
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Rust functions missing from docs/architecture/function-map.md:\n{}",
            missing.join("\n")
        ))
    }
}

fn collect_rust_function_symbols() -> Result<Vec<String>, String> {
    let mut symbols = Vec::new();
    for root in FUNCTION_SCAN_ROOTS {
        collect_rust_function_symbols_in(Path::new(root), &mut symbols)?;
    }
    symbols.sort();
    symbols.dedup();
    Ok(symbols)
}

fn collect_rust_function_symbols_in(path: &Path, symbols: &mut Vec<String>) -> Result<(), String> {
    if path.is_file() {
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            let content = fs::read_to_string(path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            collect_symbols_from_content(path, &content, symbols);
        }
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)
            .map_err(|error| format!("failed to read dir {}: {error}", path.display()))?
        {
            let entry = entry.map_err(|error| format!("failed to read dir entry: {error}"))?;
            collect_rust_function_symbols_in(&entry.path(), symbols)?;
        }
    }

    Ok(())
}

fn collect_symbols_from_content(path: &Path, content: &str, symbols: &mut Vec<String>) {
    let module = module_symbol(path);
    let mut depth = 0usize;
    let mut impl_stack: Vec<(String, usize)> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim_start();
        while impl_stack
            .last()
            .is_some_and(|(_, impl_depth)| *impl_depth > depth)
        {
            impl_stack.pop();
        }

        if let Some(name) = parse_function_name(trimmed) {
            if let Some((impl_type, _)) = impl_stack.last() {
                symbols.push(format!("{module}::{impl_type}::{name}"));
            } else {
                symbols.push(format!("{module}::{name}"));
            }
        }

        let open_count = line.chars().filter(|char| *char == '{').count();
        let close_count = line.chars().filter(|char| *char == '}').count();
        if let Some(impl_type) = parse_impl_type(trimmed) {
            impl_stack.push((impl_type.to_owned(), depth.saturating_add(1)));
        }
        depth = depth.saturating_add(open_count).saturating_sub(close_count);
    }
}

fn module_symbol(path: &Path) -> String {
    let path = path.strip_prefix(".").unwrap_or(path);
    let path = path.with_extension("");
    path.components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>()
        .join("::")
}

fn parse_function_name(line: &str) -> Option<&str> {
    let rest = line
        .strip_prefix("pub fn ")
        .or_else(|| line.strip_prefix("fn "))
        .or_else(|| line.strip_prefix("pub const fn "))
        .or_else(|| line.strip_prefix("const fn "))?;
    let end = rest.find('(')?;
    Some(rest[..end].trim())
}

fn parse_impl_type(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("impl ")?;
    let before_body = rest.split('{').next()?.trim();
    if before_body.is_empty() || before_body.starts_with('<') {
        return None;
    }
    if let Some((_, implemented_for)) = before_body.split_once(" for ") {
        Some(implemented_for.trim())
    } else {
        Some(before_body)
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
