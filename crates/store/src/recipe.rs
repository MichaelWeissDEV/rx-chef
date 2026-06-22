use std::path::PathBuf;

use crate::{
    models::Recipe,
    paths::{ensure_dir, recipes_dir, Scope},
    StoreError,
};

/// Lightweight metadata about a stored recipe (no step data loaded).
#[derive(Debug, Clone)]
pub struct RecipeMeta {
    pub name: String,
    pub description: String,
    pub step_count: usize,
    pub scope: Scope,
    pub path: PathBuf,
}

// ─── List ─────────────────────────────────────────────────────────────────────

pub fn list_recipes(scope: Option<Scope>) -> Vec<RecipeMeta> {
    let scopes: Vec<Scope> = match scope {
        Some(s) => vec![s],
        None => vec![Scope::Project, Scope::Global],
    };

    let mut results = Vec::new();
    for s in scopes {
        let dir = recipes_dir(s);
        if !dir.exists() {
            continue;
        }
        if let Ok(rd) = std::fs::read_dir(&dir) {
            for entry in rd.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e == "json" || e == "yaml" || e == "yml")
                    .unwrap_or(false)
                {
                    if let Ok(recipe) = load_file(&path) {
                        results.push(RecipeMeta {
                            name: recipe.name.clone(),
                            description: recipe.description.clone(),
                            step_count: recipe.steps.len(),
                            scope: s,
                            path,
                        });
                    }
                }
            }
        }
    }

    // Sort by name, project before global for same name
    results.sort_by(|a, b| {
        a.name.cmp(&b.name).then_with(|| match (a.scope, b.scope) {
            (Scope::Project, Scope::Global) => std::cmp::Ordering::Less,
            (Scope::Global, Scope::Project) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        })
    });

    results
}

// ─── Load ─────────────────────────────────────────────────────────────────────

/// Load recipe by name, searching project scope first then global.
pub fn load_recipe(name: &str) -> Result<Recipe, StoreError> {
    for scope in [Scope::Project, Scope::Global] {
        if let Some(path) = find_recipe_file(name, scope) {
            return load_file(&path);
        }
    }
    Err(StoreError::NotFound(format!("recipe '{}'", name)))
}

fn find_recipe_file(name: &str, scope: Scope) -> Option<PathBuf> {
    let dir = recipes_dir(scope);
    for ext in ["json", "yaml", "yml"] {
        let candidate = dir.join(format!("{}.{}", sanitize_name(name), ext));
        if candidate.exists() {
            return Some(candidate);
        }
    }
    // Also search by recipe content name (slower, but handles display-name mismatches)
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "json" || e == "yaml" || e == "yml")
                .unwrap_or(false)
            {
                if let Ok(recipe) = load_file(&path) {
                    if recipe.name.eq_ignore_ascii_case(name) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

fn load_file(path: &std::path::Path) -> Result<Recipe, StoreError> {
    let content = std::fs::read_to_string(path)?;
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("json");

    if ext == "yaml" || ext == "yml" {
        // Try full Recipe first, then CyberChef-style array
        if content.trim_start().starts_with('-') || content.trim_start().starts_with('[') {
            // Array of steps (no name field): wrap it
            let steps: Vec<crate::models::RecipeStep> = serde_yaml::from_str(&content)?;
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unnamed")
                .to_string();
            return Ok(Recipe {
                name,
                description: String::new(),
                steps,
                tags: Vec::new(),
            });
        }
        Ok(serde_yaml::from_str(&content)?)
    } else {
        // Try named Recipe first, then CyberChef-style [{op:...}] array
        if content.trim_start().starts_with('[') {
            let steps: Vec<crate::models::RecipeStep> = serde_json::from_str(&content)?;
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unnamed")
                .to_string();
            return Ok(Recipe {
                name,
                description: String::new(),
                steps,
                tags: Vec::new(),
            });
        }
        Ok(serde_json::from_str(&content)?)
    }
}

// ─── Save ─────────────────────────────────────────────────────────────────────

pub fn save_recipe(recipe: &Recipe, scope: Scope) -> Result<PathBuf, StoreError> {
    let dir = recipes_dir(scope);
    ensure_dir(&dir)?;
    let path = dir.join(format!("{}.json", sanitize_name(&recipe.name)));
    let json = serde_json::to_string_pretty(recipe)?;
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, &json)?;
    std::fs::rename(&tmp, &path)?;
    Ok(path)
}

pub fn delete_recipe(name: &str, scope: Scope) -> Result<(), StoreError> {
    let path = find_recipe_file(name, scope)
        .ok_or_else(|| StoreError::NotFound(format!("recipe '{}' in {:?} scope", name, scope)))?;
    std::fs::remove_file(path)?;
    Ok(())
}

// ─── Import / Export ──────────────────────────────────────────────────────────

/// Import a recipe from a file path (JSON or YAML).
pub fn import_recipe(
    path: &std::path::Path,
    name_override: Option<&str>,
    scope: Scope,
) -> Result<Recipe, StoreError> {
    let mut recipe = load_file(path)?;
    if let Some(n) = name_override {
        recipe.name = n.to_string();
    }
    save_recipe(&recipe, scope)?;
    Ok(recipe)
}

/// Serialize a recipe to a string (JSON or YAML).
pub fn export_recipe(recipe: &Recipe, format: &str) -> Result<String, StoreError> {
    match format {
        "yaml" | "yml" => Ok(serde_yaml::to_string(recipe)?),
        _ => Ok(serde_json::to_string_pretty(recipe)?),
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .to_lowercase()
}
