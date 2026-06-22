use std::io::{BufRead, Write};

use crate::{
    models::HistoryEntry,
    paths::{ensure_dir, history_path},
    StoreError,
};

const MAX_HISTORY_ENTRIES: usize = 10000;

/// Append a run entry to the global history log.
pub fn append_history(entry: &HistoryEntry) -> Result<(), StoreError> {
    let path = history_path();
    ensure_dir(path.parent().unwrap())?;
    let line = serde_json::to_string(entry)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    writeln!(file, "{}", line)?;

    // Rotate if needed
    if let Ok(entries) = list_history_all() {
        if entries.len() > MAX_HISTORY_ENTRIES {
            let _ = rotate_history(&entries[entries.len() - MAX_HISTORY_ENTRIES..]);
        }
    }
    Ok(())
}

/// List history entries, newest first. Optional limit.
pub fn list_history(limit: Option<usize>) -> Vec<HistoryEntry> {
    let path = history_path();
    if !path.exists() {
        return Vec::new();
    }

    let file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let reader = std::io::BufReader::new(file);
    let mut entries: Vec<HistoryEntry> = reader
        .lines()
        .filter_map(|line| {
            line.ok()
                .filter(|l| !l.trim().is_empty())
                .and_then(|l| serde_json::from_str(&l).ok())
        })
        .collect();

    // Newest first
    entries.reverse();

    if let Some(n) = limit {
        entries.truncate(n);
    }
    entries
}

/// Find one history entry by ID.
pub fn get_history(id: &str) -> Option<HistoryEntry> {
    let path = history_path();
    if !path.exists() {
        return None;
    }
    let file = std::fs::File::open(&path).ok()?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(entry) = serde_json::from_str::<HistoryEntry>(&line) {
            if entry.id == id {
                return Some(entry);
            }
        }
    }
    None
}

/// Clear all history.
pub fn clear_history() -> Result<(), StoreError> {
    let path = history_path();
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

/// List all history entries in chronological order (oldest first, no limit).
fn list_history_all() -> Result<Vec<HistoryEntry>, StoreError> {
    let path = history_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = std::fs::File::open(&path)?;
    let reader = std::io::BufReader::new(file);
    let entries: Vec<HistoryEntry> = reader
        .lines()
        .filter_map(|line| {
            line.ok()
                .filter(|l| !l.trim().is_empty())
                .and_then(|l| serde_json::from_str(&l).ok())
        })
        .collect();
    Ok(entries)
}

/// Rewrite the history file with only the given entries (oldest first).
/// Uses write-to-temp + atomic rename for safety.
fn rotate_history(entries: &[HistoryEntry]) -> Result<(), StoreError> {
    let path = history_path();
    let tmp = path.with_extension("tmp");
    let mut file = std::fs::File::create(&tmp)?;
    for entry in entries {
        let line = serde_json::to_string(entry)?;
        writeln!(file, "{}", line)?;
    }
    file.flush()?;
    std::fs::rename(&tmp, &path)?;
    Ok(())
}

/// Generate a unique history ID using UUID v4.
pub fn new_history_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
