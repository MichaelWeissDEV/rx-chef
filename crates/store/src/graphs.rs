//! Bounded graph definitions in an owner-configured directory. No payloads or tokens.
use crate::{paths, Scope, StoreError};
use rxchef::graph::{Graph, GraphPolicy, NodeKind};
use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy)]
pub struct GraphStoreLimits {
    pub files: usize,
    pub bytes: usize,
}
impl Default for GraphStoreLimits {
    fn default() -> Self {
        Self {
            files: 256,
            bytes: 64 << 20,
        }
    }
}
#[derive(Clone)]
pub struct GraphRepository {
    root: PathBuf,
    limits: GraphStoreLimits,
    policy: GraphPolicy,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphMeta {
    pub name: String,
    pub id: String,
    pub nodes: usize,
    pub edges: usize,
}
fn invalid(message: &str) -> StoreError {
    StoreError::Other(message.into())
}
pub(crate) fn checked_name(name: &str) -> Result<(), StoreError> {
    if name.is_empty()
        || name.len() > 128
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(invalid(
            "Graph storage names require 1–128 ASCII letters, digits, underscores or hyphens",
        ));
    }
    let upper = name.to_ascii_uppercase();
    if matches!(
        upper.as_str(),
        "CON" | "PRN" | "AUX" | "NUL" | "CONIN" | "CONOUT"
    ) || ((upper.starts_with("COM") || upper.starts_with("LPT"))
        && upper.len() == 4
        && upper.as_bytes()[3].is_ascii_digit())
    {
        return Err(invalid("Graph storage name is reserved on Windows"));
    }
    Ok(())
}
pub(crate) fn open_regular(path: &Path, write: bool) -> Result<File, StoreError> {
    let mut options = OpenOptions::new();
    options.read(true);
    if write {
        options.write(true).create(true);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW).mode(0o600);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.custom_flags(0x00200000);
    }
    let file = options.open(path)?;
    if !file.metadata()?.is_file() {
        return Err(invalid("Graph storage requires regular files"));
    }
    Ok(file)
}
/// Shared secret-aware definition export; engine metadata is authoritative.
pub fn check_graph_export(graph: &Graph) -> Result<(), StoreError> {
    for node in &graph.nodes {
        match &node.kind {
            NodeKind::Operation {
                operation, args, ..
            } => {
                let descriptor =
                    rxchef::integration::describe(operation).map_err(StoreError::Other)?;
                if descriptor
                    .args
                    .iter()
                    .any(|arg| arg.sensitive && args.contains_key(&arg.name))
                {
                    return Err(invalid(
                        "Bind sensitive graph arguments through named inputs before saving",
                    ));
                }
            }
            NodeKind::Recipe { steps } => {
                for step in steps {
                    let descriptor =
                        rxchef::integration::describe(&step.op).map_err(StoreError::Other)?;
                    if descriptor
                        .args
                        .iter()
                        .enumerate()
                        .any(|(i, arg)| arg.sensitive && step.args.get(i).is_some())
                    {
                        return Err(invalid(
                            "Bind sensitive recipe arguments through named inputs before saving",
                        ));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
impl GraphRepository {
    /// The host chooses this root, never a request's input binding or output path.
    pub fn open(
        root: &Path,
        limits: GraphStoreLimits,
        policy: GraphPolicy,
    ) -> Result<Self, StoreError> {
        if limits.files == 0 || limits.bytes == 0 {
            return Err(invalid("Graph store limits must be positive"));
        }
        if root
            .symlink_metadata()
            .is_ok_and(|m| m.file_type().is_symlink())
        {
            return Err(invalid("Graph store root must not be a symlink"));
        }
        paths::ensure_dir(root)?;
        if !root.is_dir() {
            return Err(invalid("Graph store root must be a directory"));
        }
        paths::restrict_dir(root)?;
        Ok(Self {
            root: root.canonicalize()?,
            limits,
            policy,
        })
    }
    pub fn in_scope(scope: Scope) -> Result<Self, StoreError> {
        Self::open(
            &paths::ensure_scope_dir(scope)?.join("graphs"),
            GraphStoreLimits::default(),
            GraphPolicy::default(),
        )
    }
    fn lock(&self) -> Result<File, StoreError> {
        if !self.root.symlink_metadata()?.is_dir() || self.root.canonicalize()? != self.root {
            return Err(invalid("Graph store root was replaced"));
        }
        let file = open_regular(&self.root.join(".graphs.lock"), true)?;
        file.lock()?;
        Ok(file)
    }
    fn path(&self, name: &str) -> Result<PathBuf, StoreError> {
        checked_name(name)?;
        Ok(self.root.join(format!("{name}.rxgraph.json")))
    }
    fn read(&self, name: &str) -> Result<Graph, StoreError> {
        let file = open_regular(&self.path(name)?, false)?;
        let mut bytes = Vec::new();
        file.take(self.policy.limits.max_definition_bytes as u64 + 1)
            .read_to_end(&mut bytes)?;
        Graph::from_json(&bytes, &self.policy).map_err(|e| StoreError::Other(e.to_string()))
    }
    fn entries(&self) -> Result<Vec<(String, u64)>, StoreError> {
        let mut entries: Vec<(String, u64)> = Vec::new();
        for entry in std::fs::read_dir(&self.root)? {
            let entry = entry?;
            let name = entry.file_name();
            let Some(name) = name.to_str().and_then(|n| n.strip_suffix(".rxgraph.json")) else {
                continue;
            };
            checked_name(name)?;
            if !entry.file_type()?.is_file() {
                return Err(invalid("Graph entry is not a regular file"));
            }
            entries.push((name.into(), entry.metadata()?.len()));
            if entries.len() > self.limits.files {
                return Err(invalid("Graph store file limit exceeded"));
            }
        }
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(entries)
    }
    fn write(&self, name: &str, graph: &Graph, overwrite: bool) -> Result<(), StoreError> {
        let target = self.path(name)?;
        rxchef::graph::validate(graph, &self.policy)
            .map_err(|e| StoreError::Other(e.to_string()))?;
        check_graph_export(graph)?;
        let contents = serde_json::to_vec_pretty(graph)?;
        if contents.len() > self.policy.limits.max_definition_bytes {
            return Err(invalid("Graph definition byte limit exceeded"));
        }
        let entries = self.entries()?;
        let existing = entries.iter().find(|(n, _)| n.eq_ignore_ascii_case(name));
        if existing.is_some_and(|(actual, _)| actual != name) {
            return Err(invalid(
                "Graph storage name differs only by case from an existing entry",
            ));
        }
        if existing.is_some() && !overwrite {
            return Err(invalid("Graph already exists"));
        }
        if existing.is_none() && entries.len() == self.limits.files {
            return Err(invalid("Graph store file limit exceeded"));
        }
        let bytes = entries
            .iter()
            .try_fold(0u64, |total, (_, size)| total.checked_add(*size))
            .ok_or_else(|| invalid("Graph store byte count overflow"))?;
        let updated = bytes
            .saturating_sub(existing.map_or(0, |(_, size)| *size))
            .checked_add(contents.len() as u64)
            .ok_or_else(|| invalid("Graph store byte count overflow"))?;
        if updated > self.limits.bytes as u64 {
            return Err(invalid("Graph store byte limit exceeded"));
        }
        paths::atomic_write(&target, &contents, true)?;
        Ok(())
    }
    pub fn save(&self, name: &str, graph: &Graph, overwrite: bool) -> Result<(), StoreError> {
        let _lock = self.lock()?;
        self.write(name, graph, overwrite)
    }
    pub fn load(&self, name: &str) -> Result<Graph, StoreError> {
        let _lock = self.lock()?;
        self.read(name)
    }
    pub fn list(&self) -> Result<Vec<GraphMeta>, StoreError> {
        let _lock = self.lock()?;
        self.entries()?
            .into_iter()
            .map(|(name, _)| {
                let graph = self.read(&name)?;
                Ok(GraphMeta {
                    name,
                    id: graph.id,
                    nodes: graph.nodes.len(),
                    edges: graph.edges.len(),
                })
            })
            .collect()
    }
    pub fn delete(&self, name: &str) -> Result<(), StoreError> {
        let _lock = self.lock()?;
        let path = self.path(name)?;
        let _regular = open_regular(&path, false)?;
        std::fs::remove_file(path)?;
        Ok(())
    }
    pub fn duplicate(&self, source: &str, target: &str) -> Result<(), StoreError> {
        let _lock = self.lock()?;
        let mut graph = self.read(source)?;
        graph.id = target.into();
        self.write(target, &graph, false)
    }
    pub fn rename(&self, source: &str, target: &str) -> Result<(), StoreError> {
        let _lock = self.lock()?;
        self.read(source)?;
        let destination = self.path(target)?;
        if self
            .entries()?
            .iter()
            .any(|(key, _)| key.eq_ignore_ascii_case(target))
        {
            return Err(invalid("Graph already exists"));
        }
        // Rename the storage key atomically; execution IDs and references stay stable.
        std::fs::rename(self.path(source)?, destination)?;
        Ok(())
    }
    pub fn import(&self, name: &str, bytes: &[u8]) -> Result<(), StoreError> {
        let graph =
            Graph::from_json(bytes, &self.policy).map_err(|e| StoreError::Other(e.to_string()))?;
        self.save(name, &graph, false)
    }
    pub fn export(&self, name: &str) -> Result<Vec<u8>, StoreError> {
        let graph = self.load(name)?;
        check_graph_export(&graph)?;
        Ok(serde_json::to_vec_pretty(&graph)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Directory(PathBuf);
    impl Directory {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!("rxchef-graphs-{}", uuid::Uuid::new_v4()));
            Self(path)
        }
    }
    impl Drop for Directory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn graph_crud_import_export_and_secret_rejection_are_lossless() {
        let dir = Directory::new();
        let store =
            GraphRepository::open(&dir.0, GraphStoreLimits::default(), GraphPolicy::default())
                .unwrap();
        let graph = Graph::from_recipe("example".into(), Vec::new());
        store.save("example", &graph, false).unwrap();
        assert!(store.save("example", &graph, false).is_err());
        store.duplicate("example", "copy").unwrap();
        store.rename("copy", "renamed").unwrap();
        assert!(store.load("copy").is_err());
        store
            .import("imported", &store.export("example").unwrap())
            .unwrap();
        assert_eq!(store.list().unwrap().len(), 3);
        assert_eq!(
            serde_json::to_value(store.load("imported").unwrap()).unwrap(),
            serde_json::to_value(&graph).unwrap()
        );
        store.delete("renamed").unwrap();
        assert_eq!(store.list().unwrap().len(), 2);
        let secret = Graph::from_recipe(
            "secret".into(),
            vec![rxchef::execution::RecipeStep {
                op: "AES Decrypt".into(),
                args: vec!["SECRET".into()],
            }],
        );
        assert!(check_graph_export(&secret).is_err());
    }
    #[test]
    fn limits_traversal_versions_and_corruption_fail_without_overwriting() {
        let dir = Directory::new();
        let store = GraphRepository::open(
            &dir.0,
            GraphStoreLimits {
                files: 1,
                bytes: 4096,
            },
            GraphPolicy::default(),
        )
        .unwrap();
        let graph = Graph::from_recipe("example".into(), Vec::new());
        store.save("one", &graph, false).unwrap();
        for name in [
            "../escape",
            "/absolute",
            "a/b",
            "a\\b",
            "",
            ".",
            "CON",
            "aux",
            "COM1",
            "LPT9",
        ] {
            assert!(store.save(name, &graph, false).is_err());
        }
        assert!(store.save("two", &graph, false).is_err());
        assert!(store.save("ONE", &graph, true).is_err());
        assert!(store.import("one", b"{bad").is_err());
        assert_eq!(store.load("one").unwrap().id, "example");
        let mut future = graph;
        future.version = 999;
        assert!(store.save("one", &future, true).is_err());
        std::fs::write(dir.0.join("one.rxgraph.json"), b"invalid").unwrap();
        assert!(store.list().is_err());
        assert!(store.load("one").is_err());
    }
    #[test]
    fn concurrent_writers_share_quota_and_byte_limits_are_checked_before_commit() {
        let dir = Directory::new();
        let store = GraphRepository::open(
            &dir.0,
            GraphStoreLimits {
                files: 1,
                bytes: 4096,
            },
            GraphPolicy::default(),
        )
        .unwrap();
        let threads: Vec<_> = (0..8)
            .map(|i| {
                let store = store.clone();
                std::thread::spawn(move || {
                    store
                        .save(
                            &format!("graph-{i}"),
                            &Graph::from_recipe("test".into(), Vec::new()),
                            false,
                        )
                        .is_ok()
                })
            })
            .collect();
        assert_eq!(
            threads
                .into_iter()
                .map(|thread| thread.join().unwrap())
                .filter(|success| *success)
                .count(),
            1
        );
        assert_eq!(store.list().unwrap().len(), 1);
        let tiny = Directory::new();
        let store = GraphRepository::open(
            &tiny.0,
            GraphStoreLimits {
                files: 2,
                bytes: 16,
            },
            GraphPolicy::default(),
        )
        .unwrap();
        assert!(store
            .save(
                "graph",
                &Graph::from_recipe("test".into(), Vec::new()),
                false
            )
            .is_err());
        assert!(store.list().unwrap().is_empty());
    }
    #[cfg(unix)]
    #[test]
    fn symlink_entries_and_lock_files_never_escape_the_owned_root() {
        use std::os::unix::fs::symlink;
        let dir = Directory::new();
        let outside = Directory::new();
        std::fs::create_dir_all(&outside.0).unwrap();
        let target = outside.0.join("secret");
        std::fs::write(&target, b"private").unwrap();
        let store =
            GraphRepository::open(&dir.0, GraphStoreLimits::default(), GraphPolicy::default())
                .unwrap();
        symlink(&target, dir.0.join("escaped.rxgraph.json")).unwrap();
        assert!(store.load("escaped").is_err());
        assert!(store.delete("escaped").is_err());
        assert!(store.list().is_err());
        std::fs::remove_file(dir.0.join("escaped.rxgraph.json")).unwrap();
        std::fs::remove_file(dir.0.join(".graphs.lock")).unwrap();
        symlink(&target, dir.0.join(".graphs.lock")).unwrap();
        assert!(store.load("anything").is_err());
        assert_eq!(std::fs::read(target).unwrap(), b"private");
    }
}
