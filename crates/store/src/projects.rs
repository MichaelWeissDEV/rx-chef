//! Private, quota-limited portable projects. References are never followed.
use crate::{
    graphs::{checked_name, open_regular},
    paths,
    workbench::{WorkbenchProject, MAX_WORKBENCH_BYTES},
    Scope, StoreError,
};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
#[derive(Clone)]
pub struct ProjectRepository {
    root: PathBuf,
    max_files: usize,
    max_bytes: usize,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProjectMeta {
    pub key: String,
    pub name: String,
    pub graphs: usize,
    pub recipes: usize,
}
fn reject(message: &str) -> StoreError {
    StoreError::Other(message.into())
}
impl ProjectRepository {
    pub fn open(root: &Path, max_files: usize, max_bytes: usize) -> Result<Self, StoreError> {
        if max_files == 0 || max_bytes == 0 {
            return Err(reject("Project storage limits must be positive"));
        }
        if root
            .symlink_metadata()
            .is_ok_and(|m| m.file_type().is_symlink())
        {
            return Err(reject("Project store root must not be a symlink"));
        }
        paths::ensure_dir(root)?;
        if !root.is_dir() {
            return Err(reject("Project store root must be a directory"));
        }
        paths::restrict_dir(root)?;
        Ok(Self {
            root: root.canonicalize()?,
            max_files,
            max_bytes,
        })
    }
    pub fn in_scope(scope: Scope) -> Result<Self, StoreError> {
        Self::open(
            &paths::ensure_scope_dir(scope)?.join("projects"),
            32,
            64 << 20,
        )
    }
    fn lock(&self) -> Result<File, StoreError> {
        if !self.root.symlink_metadata()?.is_dir() || self.root.canonicalize()? != self.root {
            return Err(reject("Project store root was replaced"));
        }
        let lock = open_regular(&self.root.join(".projects.lock"), true)?;
        lock.lock()?;
        Ok(lock)
    }
    fn path(&self, key: &str) -> Result<PathBuf, StoreError> {
        checked_name(key)?;
        Ok(self.root.join(format!("{key}.rxproject.json")))
    }
    fn read(&self, key: &str) -> Result<WorkbenchProject, StoreError> {
        let file = open_regular(&self.path(key)?, false)?;
        let mut bytes = Vec::new();
        file.take(MAX_WORKBENCH_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        WorkbenchProject::from_json(&bytes)
    }
    fn entries(&self) -> Result<Vec<(String, u64)>, StoreError> {
        let mut entries: Vec<(String, u64)> = Vec::new();
        for entry in std::fs::read_dir(&self.root)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let Some(key) = file_name
                .to_str()
                .and_then(|name| name.strip_suffix(".rxproject.json"))
            else {
                continue;
            };
            checked_name(key)?;
            if !entry.file_type()?.is_file() {
                return Err(reject("Project storage entry is not a regular file"));
            }
            entries.push((key.into(), entry.metadata()?.len()));
            if entries.len() > self.max_files {
                return Err(reject("Project store file limit exceeded"));
            }
        }
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(entries)
    }
    fn write(
        &self,
        key: &str,
        project: &WorkbenchProject,
        replace: bool,
    ) -> Result<(), StoreError> {
        let path = self.path(key)?;
        let bytes = project.to_json()?;
        let entries = self.entries()?;
        let existing = entries
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key));
        if existing.is_some_and(|(name, _)| name != key) {
            return Err(reject(
                "Project storage key differs only by case from an existing entry",
            ));
        }
        if existing.is_some() && !replace {
            return Err(reject("Project already exists"));
        }
        if existing.is_none() && entries.len() == self.max_files {
            return Err(reject("Project store file limit exceeded"));
        }
        let total = entries
            .iter()
            .try_fold(0u64, |total, (_, size)| total.checked_add(*size))
            .ok_or_else(|| reject("Project storage byte count overflow"))?;
        let updated = total
            .saturating_sub(existing.map_or(0, |(_, size)| *size))
            .checked_add(bytes.len() as u64)
            .ok_or_else(|| reject("Project storage byte count overflow"))?;
        if updated > self.max_bytes as u64 {
            return Err(reject("Project store byte limit exceeded"));
        }
        paths::atomic_write(&path, &bytes, true)?;
        Ok(())
    }
    pub fn save(
        &self,
        key: &str,
        project: &WorkbenchProject,
        replace: bool,
    ) -> Result<(), StoreError> {
        let _lock = self.lock()?;
        self.write(key, project, replace)
    }
    pub fn load(&self, key: &str) -> Result<WorkbenchProject, StoreError> {
        let _lock = self.lock()?;
        self.read(key)
    }
    pub fn list(&self) -> Result<Vec<ProjectMeta>, StoreError> {
        let _lock = self.lock()?;
        self.entries()?
            .into_iter()
            .map(|(key, _)| {
                let project = self.read(&key)?;
                Ok(ProjectMeta {
                    key,
                    name: project.name,
                    graphs: project.graphs.len(),
                    recipes: project.recipes.len(),
                })
            })
            .collect()
    }
    pub fn delete(&self, key: &str) -> Result<(), StoreError> {
        let _lock = self.lock()?;
        let path = self.path(key)?;
        let _file = open_regular(&path, false)?;
        std::fs::remove_file(path)?;
        Ok(())
    }
    pub fn duplicate(&self, source: &str, target: &str) -> Result<(), StoreError> {
        let _lock = self.lock()?;
        let mut project = self.read(source)?;
        project.name = target.into();
        self.write(target, &project, false)
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
            return Err(reject("Project already exists"));
        }
        std::fs::rename(self.path(source)?, destination)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn private_projects_obey_quotas_versions_and_portable_names() {
        let root = std::env::temp_dir().join(format!("rxchef-projects-{}", uuid::Uuid::new_v4()));
        let store = ProjectRepository::open(&root, 2, 4096).unwrap();
        let project = WorkbenchProject::new("Session");
        store.save("session", &project, false).unwrap();
        assert!(store.save("SESSION", &project, true).is_err());
        assert!(store.save("../escape", &project, false).is_err());
        store.duplicate("session", "copy").unwrap();
        assert!(store.save("third", &project, false).is_err());
        store.rename("copy", "renamed").unwrap();
        assert_eq!(store.list().unwrap().len(), 2);
        let mut future = project;
        future.version = 999;
        assert!(store.save("session", &future, true).is_err());
        assert_eq!(store.load("session").unwrap().name, "Session");
        store.delete("renamed").unwrap();
        assert_eq!(store.list().unwrap().len(), 1);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(
                root.join("session.rxproject.json"),
                root.join("link.rxproject.json"),
            )
            .unwrap();
            assert!(store.load("link").is_err());
            assert!(store.delete("link").is_err());
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}
