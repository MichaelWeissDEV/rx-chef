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
    restrict_dir(&path)?;
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

/// Create `path` and any missing parents, restricting it to the owner.
///
/// Stored recipes, projects, variables, and history can carry keys, tokens,
/// and decoded credentials. The files themselves are written 0600, but
/// `create_dir_all` uses 0777 masked by the umask (commonly 0755), which
/// leaves the *names* of stored recipes and projects world-readable. Narrowing
/// the directory to 0700 closes that.
pub fn ensure_dir(path: &std::path::Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
        restrict_dir(path)?;
    }
    Ok(())
}

/// Restrict a directory to owner-only access on Unix. A no-op elsewhere.
pub(crate) fn restrict_dir(path: &std::path::Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))?;
    }
    #[cfg(not(unix))]
    let _ = path;
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

    #[cfg(unix)]
    #[test]
    fn created_store_directories_are_owner_only() {
        use std::os::unix::fs::PermissionsExt;

        // Stored recipes and projects can be named after the secrets they
        // handle, so the directory itself must not be world-readable.
        let root = std::env::temp_dir().join(format!("rxchef-perm-test-{}", uuid::Uuid::new_v4()));
        super::ensure_dir(&root).unwrap();
        let mode = std::fs::metadata(&root).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o700, "expected 0700, got {mode:o}");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn private_store_files_are_owner_only() {
        use std::os::unix::fs::PermissionsExt;

        let root = std::env::temp_dir().join(format!("rxchef-file-test-{}", uuid::Uuid::new_v4()));
        let file = root.join("vars.json");
        super::atomic_write(&file, b"{}", true).unwrap();
        let mode = std::fs::metadata(&file).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "expected 0600, got {mode:o}");
        std::fs::remove_dir_all(root).unwrap();
    }
}
