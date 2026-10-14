//! Portable sessions contain definitions and metadata, never bound bytes or credentials.
use crate::{check_graph_export, paths::atomic_write, StoreError};
use rxchef::graph::{Graph, GraphPolicy, NodeKind};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::Path,
};
pub const WORKBENCH_VERSION: u32 = 1;
pub const MAX_WORKBENCH_BYTES: usize = 16 << 20;
fn enabled() -> bool {
    true
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WorkbenchStep {
    pub op: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default = "enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub breakpoint: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WorkbenchRecipe {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub steps: Vec<WorkbenchStep>,
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum WorkbenchTheme {
    #[default]
    Dark,
    Light,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WorkbenchSettings {
    #[serde(default)]
    pub theme: WorkbenchTheme,
    #[serde(default)]
    pub selected_graph: Option<String>,
    #[serde(default)]
    pub selected_recipe: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct InputFileReference {
    pub graph_id: String,
    pub input_name: String,
    pub path: String,
    /// Must be true: the file is a reference, not a bundled project payload.
    pub external: bool,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum WorkbenchRunStatus {
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    Discarded,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum WorkbenchErrorCode {
    Validation,
    Operation,
    Transport,
    Limit,
    Timeout,
    Cancelled,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WorkbenchRun {
    pub id: String,
    pub run_id: String,
    pub graph_id: String,
    pub started_at: String,
    pub status: WorkbenchRunStatus,
    pub elapsed_ms: f64,
    #[serde(default)]
    pub output_sizes: BTreeMap<String, usize>,
    #[serde(default)]
    pub error_code: Option<WorkbenchErrorCode>,
}
impl WorkbenchRun {
    /// Reuse the existing bounded history store without retaining arguments or previews.
    pub fn append_history(&self, input_bytes: usize) -> Result<(), StoreError> {
        let output_bytes = self
            .output_sizes
            .values()
            .try_fold(0usize, |total, bytes| total.checked_add(*bytes))
            .ok_or_else(|| reject("History output byte count overflow"))?;
        let success = matches!(self.status, WorkbenchRunStatus::Succeeded);
        crate::append_history(&crate::HistoryEntry {
            id: self.id.clone(),
            timestamp: self.started_at.clone(),
            pipeline_name: Some(format!("graph:{}", self.graph_id)),
            input_preview: String::new(),
            input_bytes,
            output_preview: String::new(),
            output_bytes,
            success,
            steps: vec![crate::HistoryStep {
                op: format!("graph:{}", self.graph_id),
                args: Vec::new(),
                output_preview: String::new(),
                output_bytes,
                duration_ms: self.elapsed_ms,
                error: if success {
                    None
                } else {
                    Some(format!("{:?}", self.status))
                },
            }],
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WorkbenchProject {
    pub version: u32,
    pub name: String,
    #[serde(default)]
    pub recipes: Vec<WorkbenchRecipe>,
    #[serde(default)]
    pub graphs: Vec<Graph>,
    #[serde(default)]
    pub settings: WorkbenchSettings,
    #[serde(default)]
    pub input_files: Vec<InputFileReference>,
    #[serde(default)]
    pub history: Vec<WorkbenchRun>,
}
fn label(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256 && !value.chars().any(char::is_control)
}
fn reject(message: &str) -> StoreError {
    StoreError::Other(message.into())
}
impl WorkbenchProject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            version: WORKBENCH_VERSION,
            name: name.into(),
            recipes: Vec::new(),
            graphs: Vec::new(),
            settings: WorkbenchSettings::default(),
            input_files: Vec::new(),
            history: Vec::new(),
        }
    }
    pub fn validate(&self) -> Result<(), StoreError> {
        if self.version != WORKBENCH_VERSION {
            return Err(reject("Unsupported workbench project version"));
        }
        if !label(&self.name)
            || self.graphs.len() > 100
            || self.recipes.len() > 100
            || self.input_files.len() > 512
            || self.history.len() > 1000
        {
            return Err(reject("Workbench project count/name limit exceeded"));
        }
        let mut graph_ids = BTreeSet::new();
        let mut recipe_names = BTreeSet::new();
        for graph in &self.graphs {
            if !graph_ids.insert(graph.id.as_str()) {
                return Err(reject("Duplicate project graph ID"));
            }
            rxchef::graph::validate(graph, &GraphPolicy::default())
                .map_err(|e| StoreError::Other(e.to_string()))?;
            check_graph_export(graph)?;
        }
        for recipe in &self.recipes {
            if !label(&recipe.name)
                || recipe.description.len() > 4096
                || !recipe_names.insert(recipe.name.as_str())
                || recipe.steps.len() > 512
            {
                return Err(reject("Invalid or duplicate project recipe"));
            }
            for step in &recipe.steps {
                if !step.enabled
                    && rxchef::graph::operation_requires_recipe(
                        &rxchef::integration::describe(&step.op)
                            .map_err(StoreError::Other)?
                            .id,
                    )
                {
                    return Err(reject("Disabled recipe control-flow steps are unsupported"));
                }
            }
            let all_steps = recipe
                .steps
                .iter()
                .map(|s| rxchef::execution::RecipeStep {
                    op: s.op.clone(),
                    args: s.args.clone(),
                })
                .collect();
            check_graph_export(&Graph::from_recipe("project-recipe".into(), all_steps))?;
            let active_steps = recipe
                .steps
                .iter()
                .filter(|s| s.enabled)
                .map(|s| rxchef::execution::RecipeStep {
                    op: s.op.clone(),
                    args: s.args.clone(),
                })
                .collect();
            rxchef::graph::validate(
                &Graph::from_recipe("project-recipe".into(), active_steps),
                &GraphPolicy::default(),
            )
            .map_err(|e| StoreError::Other(e.to_string()))?;
        }
        if self
            .settings
            .selected_graph
            .as_ref()
            .is_some_and(|id| !graph_ids.contains(id.as_str()))
            || self
                .settings
                .selected_recipe
                .as_ref()
                .is_some_and(|name| !recipe_names.contains(name.as_str()))
        {
            return Err(reject("Selected project definition does not exist"));
        }
        let mut references = BTreeSet::new();
        for reference in &self.input_files {
            if !reference.external
                || reference.path.is_empty()
                || reference.path.len() > 4096
                || reference.path.chars().any(char::is_control)
                || !references.insert((&reference.graph_id, &reference.input_name))
            {
                return Err(reject("Invalid project file reference"));
            }
            if !self.graphs.iter().any(|g| {
                g.id == reference.graph_id
                    && g.nodes.iter().any(
                        |n| matches!(&n.kind,NodeKind::Input{name} if name==&reference.input_name),
                    )
            }) {
                return Err(reject("Project file reference has no matching named input"));
            }
        }
        let mut run_ids = BTreeSet::new();
        for run in &self.history {
            if !label(&run.id)
                || !label(&run.run_id)
                || !run_ids.insert(&run.id)
                || !label(&run.graph_id)
                || run.started_at.len() > 64
                || chrono::DateTime::parse_from_rfc3339(&run.started_at).is_err()
                || !run.elapsed_ms.is_finite()
                || !(0.0..=604800000.0).contains(&run.elapsed_ms)
                || run.output_sizes.len() > 64
                || run.output_sizes.keys().any(|name| !label(name))
            {
                return Err(reject("Invalid project run metadata"));
            }
        }
        if serde_json::to_vec(self)?.len() > MAX_WORKBENCH_BYTES {
            return Err(reject("Workbench project byte limit exceeded"));
        }
        Ok(())
    }
    pub fn from_json(bytes: &[u8]) -> Result<Self, StoreError> {
        if bytes.len() > MAX_WORKBENCH_BYTES {
            return Err(reject("Workbench project byte limit exceeded"));
        }
        let project: Self = serde_json::from_slice(bytes)?;
        project.validate()?;
        Ok(project)
    }
    pub fn to_json(&self) -> Result<Vec<u8>, StoreError> {
        self.validate()?;
        let bytes = serde_json::to_vec_pretty(self)?;
        if bytes.len() > MAX_WORKBENCH_BYTES {
            return Err(reject("Workbench project byte limit exceeded"));
        }
        Ok(bytes)
    }
    pub fn load(path: &Path) -> Result<Self, StoreError> {
        let file = std::fs::File::open(path)?;
        if !file.metadata()?.is_file() {
            return Err(reject("Project source must be a regular file"));
        }
        let mut bytes = Vec::new();
        file.take(MAX_WORKBENCH_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        Self::from_json(&bytes)
    }
    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        atomic_write(path, &self.to_json()?, true)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn project_roundtrip_restores_multiple_definitions_settings_and_external_references() {
        let mut project = WorkbenchProject::new("Session");
        project
            .graphs
            .push(Graph::from_recipe("graph".into(), Vec::new()));
        project.recipes.push(WorkbenchRecipe {
            name: "recipe".into(),
            description: String::new(),
            steps: vec![WorkbenchStep {
                op: "to_base64".into(),
                args: Vec::new(),
                enabled: false,
                breakpoint: true,
            }],
        });
        project.settings.selected_graph = Some("graph".into());
        project.settings.selected_recipe = Some("recipe".into());
        project.input_files.push(InputFileReference {
            graph_id: "graph".into(),
            input_name: "input".into(),
            path: "missing-external-file.bin".into(),
            external: true,
        });
        let bytes = project.to_json().unwrap();
        let loaded = WorkbenchProject::from_json(&bytes).unwrap();
        assert!(!loaded.recipes[0].steps[0].enabled);
        assert!(loaded.recipes[0].steps[0].breakpoint);
        assert_eq!(loaded.input_files[0].path, "missing-external-file.bin");
        assert!(!String::from_utf8(bytes).unwrap().contains("input_base64"));
    }
    #[test]
    fn unknown_fields_future_versions_secrets_and_invalid_references_are_rejected() {
        assert!(
            WorkbenchProject::from_json(br#"{"version":1,"name":"test","token":"SECRET"}"#)
                .is_err()
        );
        let mut project = WorkbenchProject::new("test");
        project.version = 999;
        assert!(project.to_json().is_err());
        project.version = 1;
        project.recipes.push(WorkbenchRecipe {
            name: "secret".into(),
            description: String::new(),
            steps: vec![WorkbenchStep {
                op: "AES Decrypt".into(),
                args: vec!["SECRET".into()],
                enabled: false,
                breakpoint: false,
            }],
        });
        assert!(project.to_json().is_err());
        project.recipes.clear();
        project.settings.selected_graph = Some("missing".into());
        assert!(project.to_json().is_err());
    }
}
