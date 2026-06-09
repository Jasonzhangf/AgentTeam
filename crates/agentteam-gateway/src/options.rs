use std::collections::BTreeMap;

use crate::error::{GatewayError, GatewayResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedOptions {
    pub values: BTreeMap<String, String>,
    pub json: bool,
}

pub(crate) fn parse_options(
    args: &[String],
    value_flags: &[&str],
    bool_flags: &[&str],
) -> GatewayResult<ParsedOptions> {
    let mut values = BTreeMap::new();
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        if contains_flag(value_flags, token) {
            if values.contains_key(token) {
                return Err(GatewayError::parse(format!("duplicate flag {token}")));
            }
            let Some(value) = args.get(index + 1) else {
                return Err(GatewayError::parse(format!("missing value for {token}")));
            };
            if value.starts_with("--") {
                return Err(GatewayError::parse(format!("missing value for {token}")));
            }
            values.insert(token.clone(), value.clone());
            index += 2;
        } else if contains_flag(bool_flags, token) {
            if token == "--json" && json {
                return Err(GatewayError::parse("duplicate flag --json"));
            }
            json = true;
            index += 1;
        } else if token.starts_with("--") {
            return Err(GatewayError::parse(format!("unknown flag {token}")));
        } else {
            return Err(GatewayError::parse(format!(
                "unexpected positional argument {token}"
            )));
        }
    }
    Ok(ParsedOptions { values, json })
}

pub(crate) fn contains_flag(allowed: &[&str], token: &str) -> bool {
    allowed.contains(&token)
}
