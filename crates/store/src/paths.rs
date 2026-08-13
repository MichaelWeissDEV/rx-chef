use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

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
    if let Some(path) = std::env::var_os("RXCHEF_HOME").filter(|value| !value.is_empty()) {
        return PathBuf::from(path);
    }
    let base = dirs::config_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")));
    base.join("rxchef")
}

/// Finds the closest `.rxchef` directory, walking from the current directory
/// towards the filesystem root.
pub fn discover_project_dir() -> Option<PathBuf> {
    let current = std::env::current_dir().ok()?;
    discover_project_dir_from(&current)
}

fn discover_project_dir_from(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .map(|ancestor| ancestor.join(".rxchef"))
        .find(|candidate| candidate.is_dir())
}

/// Returns the discovered project store, or the path that `project init`
/// would create when outside a project.
pub fn project_dir() -> PathBuf {
    discover_project_dir().unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(".rxchef")
    })
}

pub fn default_scope() -> Scope {
    if discover_project_dir().is_some() {
        Scope::Project
    } else {
        Scope::Global
    }
}

pub fn init_project() -> io::Result<PathBuf> {
    let path = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".rxchef");
    std::fs::create_dir_all(&path)?;
    Ok(path)
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

pub(crate) fn ensure_scope_dir(scope: Scope) -> io::Result<PathBuf> {
    let directory = store_dir(scope);
    match scope {
        Scope::Global => ensure_dir(&directory)?,
        Scope::Project if !directory.is_dir() => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no rxchef project found; run `rxchef project init` or use --global",
            ));
        }
        Scope::Project => {}
    }
    Ok(directory)
}

/// Crash-safe replacement for small store files.
pub(crate) fn atomic_write(path: &Path, contents: &[u8], private: bool) -> io::Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    ensure_dir(parent)?;
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("store");
    let temporary = parent.join(format!(
        ".{file_name}.{}.{}.tmp",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));

    let result = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        if private {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&temporary)?;
        file.write_all(contents)?;
        file.flush()?;
        file.sync_all()?;
        std::fs::rename(&temporary, path)?;
        if private {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
            }
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::discover_project_dir_from;

    #[test]
    fn discovers_project_in_parent() {
        let root = std::env::temp_dir().join(format!("rxchef-path-test-{}", uuid::Uuid::new_v4()));
        let nested = root.join("one/two");
        std::fs::create_dir_all(root.join(".rxchef")).unwrap();
        std::fs::create_dir_all(&nested).unwrap();
        assert_eq!(
            discover_project_dir_from(&nested),
            Some(root.join(".rxchef"))
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}
