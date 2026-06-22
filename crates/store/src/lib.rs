/*!
rxchef_store — persistent store for pipelines, variables, and run history.

Two scopes:
- **Global** (`~/.config/rxchef/`)      — shared across all projects
- **Project** (`./.rxchef/`)             — local to the current directory

Load precedence: project overrides global when names conflict.
*/

pub mod history;
pub mod models;
mod paths;
mod recipe;
mod vars;

pub use history::{append_history, clear_history, get_history, list_history, new_history_id};
pub use models::{bytes_preview, HistoryEntry, HistoryStep, Recipe, RecipeStep, Variable};
pub use paths::{global_dir, project_dir, Scope};
pub use recipe::{
    delete_recipe, export_recipe, import_recipe, list_recipes, load_recipe, save_recipe, RecipeMeta,
};
pub use vars::{expand_vars, get_var, list_vars, set_var, unset_var};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("{0}")]
    Other(String),
}
