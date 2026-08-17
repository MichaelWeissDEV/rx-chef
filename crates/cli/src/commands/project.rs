//! `rxchef project` — CTF-style project workspaces.

use rxchef_store::{self as store};

use crate::cli::{ProjectAction, ProjectArgs};
use crate::output;
use crate::steps::{self, Step};

pub(crate) fn cmd_project(a: ProjectArgs) -> Result<(), String> {
    match a.action {
        ProjectAction::Init => {
            let path = store::init_project().map_err(|error| error.to_string())?;
            println!("Initialized rxchef project at '{}'.", path.display());
            Ok(())
        }
        ProjectAction::Run { file, trace } => {
            let project =
                store::load_project(&file).map_err(|e| format!("Failed to load project: {}", e))?;

            let (input_bytes, input_supplied) = match project.data {
                Some(store::ProjectData::Inline { inline }) => (inline.into_bytes(), true),
                Some(store::ProjectData::File { file: path }) => {
                    let base_dir = file.parent().unwrap_or(std::path::Path::new(""));
                    (
                        std::fs::read(base_dir.join(path)).map_err(|e| e.to_string())?,
                        true,
                    )
                }
                None => (Vec::new(), false),
            };

            let steps: Vec<_> = project
                .pipeline
                .iter()
                .map(|s| Step {
                    op: s.op.clone(),
                    args: s.args.clone(),
                })
                .collect();

            let mut overrides = std::collections::HashMap::new();
            for (k, v) in project.variables.iter() {
                overrides.insert(k.clone(), v.clone());
            }

            let result = steps::run_steps(
                &steps,
                input_bytes.clone(),
                input_supplied,
                &overrides,
                trace,
                false,
            )?;
            output::write_output(&result.final_output, false)?;

            Ok(())
        }
    }
}
