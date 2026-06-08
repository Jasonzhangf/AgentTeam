pub const REQUIRED_GATES: &[&str] = &[
    "cargo fmt --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cargo xtask red-tests",
    "cargo xtask verify-required-files",
    "cargo xtask verify-skill-frontmatter",
    "cargo xtask verify-resource-lifecycle",
    "cargo xtask verify-function-map",
    "cargo xtask verify-code-size",
];
