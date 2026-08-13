use crate::models::RecipeStep;
use crate::StoreError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectData {
    File { file: String },
    Inline { inline: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCheckpoint {
    pub name: String,
    pub step_index: usize,
    pub data_hex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectData>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub variables: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pipeline: Vec<RecipeStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoints: Vec<ProjectCheckpoint>,
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        Project {
            name: name.into(),
            description: String::new(),
            data: None,
            variables: HashMap::new(),
            pipeline: Vec::new(),
            checkpoints: Vec::new(),
        }
    }
}

pub fn load_project(path: &Path) -> Result<Project, StoreError> {
    let content = fs::read_to_string(path)?;
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("json");
    if ext == "yaml" || ext == "yml" {
        Ok(serde_yaml::from_str(&content)?)
    } else {
        Ok(serde_json::from_str(&content)?)
    }
}

pub fn save_project(project: &Project, path: &Path) -> Result<(), StoreError> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("json");
    let content = if ext == "yaml" || ext == "yml" {
        serde_yaml::to_string(project)?
    } else {
        serde_json::to_string_pretty(project)?
    };
    crate::paths::atomic_write(path, content.as_bytes(), false)?;
    Ok(())
}
