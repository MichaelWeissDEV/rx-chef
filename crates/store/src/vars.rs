use std::collections::HashMap;

use crate::{
    models::Variable,
    paths::{atomic_write, ensure_scope_dir, vars_path, Scope},
    StoreError,
};

type VarMap = HashMap<String, Variable>;

fn load_vars(scope: Scope) -> VarMap {
    let path = vars_path(scope);
    if !path.exists() {
        return HashMap::new();
    }
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<VarMap>(&s).ok())
        .unwrap_or_default()
}

fn save_vars(scope: Scope, map: &VarMap) -> Result<(), StoreError> {
    ensure_scope_dir(scope)?;
    let path = vars_path(scope);
    let json = serde_json::to_string_pretty(map)?;
    atomic_write(&path, json.as_bytes(), true)?;
    Ok(())
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Set a variable in the given scope.
pub fn set_var(name: &str, value: &str, description: &str, scope: Scope) -> Result<(), StoreError> {
    set_var_with_options(name, value, description, false, scope)
}

pub fn set_var_with_options(
    name: &str,
    value: &str,
    description: &str,
    secret: bool,
    scope: Scope,
) -> Result<(), StoreError> {
    let mut map = load_vars(scope);
    map.insert(
        name.to_uppercase(),
        Variable {
            name: name.to_uppercase(),
            value: value.to_string(),
            description: description.to_string(),
            secret,
        },
    );
    save_vars(scope, &map)
}

/// Get a variable value, searching project first then global.
pub fn get_var(name: &str) -> Option<String> {
    let key = name.to_uppercase();
    for scope in [Scope::Project, Scope::Global] {
        if let Some(v) = load_vars(scope).remove(&key) {
            return Some(v.value);
        }
    }
    None
}

/// List variables from the given scope (or both if None).
pub fn list_vars(scope: Option<Scope>) -> Vec<Variable> {
    list_vars_with_scope(scope)
        .into_iter()
        .map(|(_, variable)| variable)
        .collect()
}

/// List resolved variables while retaining the scope that supplied each value.
pub fn list_vars_with_scope(scope: Option<Scope>) -> Vec<(Scope, Variable)> {
    let mut result: HashMap<String, (Scope, Variable)> = HashMap::new();
    // Load global first, then project overrides
    for s in [Scope::Global, Scope::Project] {
        if scope.is_none() || scope == Some(s) {
            for (k, v) in load_vars(s) {
                result.insert(k, (s, v));
            }
        }
    }
    let mut vars: Vec<(Scope, Variable)> = result.into_values().collect();
    vars.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    vars
}

/// Remove a variable from the given scope.
pub fn unset_var(name: &str, scope: Scope) -> Result<(), StoreError> {
    let key = name.to_uppercase();
    let mut map = load_vars(scope);
    if map.remove(&key).is_none() {
        return Err(StoreError::NotFound(format!(
            "variable '{}' in {:?} scope",
            name, scope
        )));
    }
    save_vars(scope, &map)
}

/// Expand `$VAR` and `${VAR}` patterns in a string using all stored variables
/// plus an optional override map.
pub fn expand_vars(input: &str, overrides: &HashMap<String, String>) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    // Collect all vars: global < project < overrides
    let mut all_vars: HashMap<String, String> = HashMap::new();
    for v in list_vars(None) {
        all_vars.insert(v.name.to_uppercase(), v.value);
    }
    for (k, v) in overrides {
        all_vars.insert(k.to_uppercase(), v.clone());
    }

    while i < len {
        if chars[i] == '$' && i + 1 < len {
            if chars[i + 1] == '{' {
                // ${VAR_NAME}
                if let Some(end) = chars[i + 2..].iter().position(|&c| c == '}') {
                    let name: String = chars[i + 2..i + 2 + end].iter().collect();
                    let key = name.to_uppercase();
                    if let Some(val) = all_vars.get(&key) {
                        result.push_str(val);
                    } else {
                        // Leave unexpanded
                        result.push_str(&format!("${{{}}}", name));
                    }
                    i += 2 + end + 1;
                    continue;
                }
            } else if chars[i + 1].is_alphabetic() || chars[i + 1] == '_' {
                // $VAR_NAME
                let start = i + 1;
                let mut end = start;
                while end < len && (chars[end].is_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }
                let name: String = chars[start..end].iter().collect();
                let key = name.to_uppercase();
                if let Some(val) = all_vars.get(&key) {
                    result.push_str(val);
                } else {
                    result.push('$');
                    result.push_str(&name);
                }
                i = end;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}
