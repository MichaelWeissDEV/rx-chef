use std::path::PathBuf;

/// Which store to read from / write to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// `~/.config/rxchef/`
    Global,
    /// `./.rxchef/` (or the nearest ancestor `.rxchef/`)
    Project,
}

/// Returns the global config directory, creating it if necessary.
pub fn global_dir() -> PathBuf {
    let base = dirs::config_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")));
    base.join("rxchef")
}

/// Returns the project store directory (`.rxchef/` in current dir).
/// Always returns the path even if it doesn't exist yet.
pub fn project_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".rxchef")
}

/// Returns the directory for the given scope, creating it if needed.
pub fn store_dir(scope: Scope) -> PathBuf {
    match scope {
        Scope::Global => global_dir(),
        Scope::Project => project_dir(),
    }
}

pub fn recipes_dir(scope: Scope) -> PathBuf {
    store_dir(scope).join("recipes")
}

pub fn vars_path(scope: Scope) -> PathBuf {
    store_dir(scope).join("vars.json")
}

pub fn history_path() -> PathBuf {
    // History is global-only (one history file, not per project)
    global_dir().join("history.jsonl")
}

pub fn ensure_dir(path: &std::path::Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
