//! `rxchef pipeline` — manage and run saved pipelines.

use std::fs;
use std::io::{self, Write};

use rxchef::runtime;
use rxchef_store::{self as store, Scope};

use crate::cli::{PipelineAction, PipelineArgs};
use crate::commands::var::mutation_scope;
use crate::steps::Step;
use crate::{input, output, steps};

pub(crate) fn cmd_pipeline(a: PipelineArgs) -> Result<(), String> {
    match a.action {
        PipelineAction::List {
            global,
            project,
            json,
        } => {
            let scope = if global {
                Some(Scope::Global)
            } else if project {
                Some(Scope::Project)
            } else {
                None
            };
            let recipes = store::list_recipes(scope);
            if json {
                let v: Vec<_> = recipes
                    .iter()
                    .map(|r| {
                        serde_json::json!({
                            "name": r.name, "description": r.description,
                            "steps": r.step_count,
                            "scope": if r.scope == Scope::Global { "global" } else { "project" },
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
                return Ok(());
            }
            if recipes.is_empty() {
                println!("No saved pipelines. Create one with: rxchef pipeline new <name>");
                return Ok(());
            }
            println!(
                "{:<24} {:>5}  {:<8}  {}",
                "NAME", "STEPS", "SCOPE", "DESCRIPTION"
            );
            println!("{}", "-".repeat(70));
            for r in &recipes {
                let scope_tag = if r.scope == Scope::Global {
                    "global"
                } else {
                    "project"
                };
                println!(
                    "{:<24} {:>5}  {:<8}  {}",
                    r.name, r.step_count, scope_tag, r.description
                );
            }
            eprintln!("\n{} pipeline(s)", recipes.len());
        }

        PipelineAction::Show { name, json, format } => {
            let recipe = store::load_recipe(&name).map_err(|e| e.to_string())?;
            if json {
                println!("{}", serde_json::to_string_pretty(&recipe).unwrap());
            } else {
                let out = store::export_recipe(&recipe, &format).map_err(|e| e.to_string())?;
                println!("{}", out);
            }
        }

        PipelineAction::New {
            name,
            description,
            global,
            project,
        } => {
            let scope = mutation_scope(global, project);
            let mut recipe = store::Recipe::new(&name);
            if let Some(d) = description {
                recipe.description = d;
            }
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!("Created pipeline '{}' ({:?} scope).", name, scope);
        }

        PipelineAction::Add {
            pipeline,
            step,
            args,
            global,
            project,
        } => {
            let scope = mutation_scope(global, project);
            let mut recipe =
                store::load_recipe_in_scope(&pipeline, scope).map_err(|e| e.to_string())?;
            let parsed = steps::parse_step_str(&step)?;
            let mut all_args = parsed.args;
            all_args.extend(args);
            recipe.steps.push(store::RecipeStep {
                op: parsed.op.clone(),
                args: all_args,
            });
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!(
                "Added '{}' as step {} to '{}'.",
                parsed.op,
                recipe.steps.len(),
                pipeline
            );
        }

        PipelineAction::Remove {
            pipeline,
            index,
            global,
            project,
        } => {
            let scope = mutation_scope(global, project);
            let mut recipe =
                store::load_recipe_in_scope(&pipeline, scope).map_err(|e| e.to_string())?;
            if index == 0 || index > recipe.steps.len() {
                return Err(format!(
                    "step index {} out of range (1–{})",
                    index,
                    recipe.steps.len()
                ));
            }
            let removed = recipe.steps.remove(index - 1);
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!("Removed step {}: '{}'.", index, removed.op);
        }

        PipelineAction::Set {
            pipeline,
            step,
            arg,
            value,
            global,
            project,
        } => {
            let scope = mutation_scope(global, project);
            let mut recipe =
                store::load_recipe_in_scope(&pipeline, scope).map_err(|e| e.to_string())?;
            if step == 0 || step > recipe.steps.len() {
                return Err(format!(
                    "step {} out of range (1–{})",
                    step,
                    recipe.steps.len()
                ));
            }
            let s = &mut recipe.steps[step - 1];

            // Resolve arg position: numeric index or arg name
            let arg_idx = if let Ok(n) = arg.parse::<usize>() {
                if n == 0 || n > s.args.len() {
                    // Extend args if needed
                    while s.args.len() < n {
                        s.args.push(String::new());
                    }
                    n - 1
                } else {
                    n - 1
                }
            } else {
                // Look up arg name from schema
                let op_info = runtime::operation_info(&s.op).map_err(|e| e.to_string())?;
                let arg_lower = arg.to_lowercase();
                let idx = op_info
                    .args
                    .iter()
                    .position(|a| a.name.to_lowercase() == arg_lower)
                    .ok_or_else(|| format!("argument '{}' not found in '{}'", arg, s.op))?;
                while s.args.len() <= idx {
                    s.args.push(String::new());
                }
                idx
            };

            let old = s.args.get(arg_idx).cloned().unwrap_or_default();
            s.args[arg_idx] = value.clone();
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!(
                "Step {}, arg {}: '{}' → '{}'.",
                step,
                arg_idx + 1,
                old,
                value
            );
        }

        PipelineAction::Run {
            name,
            input,
            input_file,
            trace,
            hex,
            save,
            set_vars,
        } => {
            let recipe = store::load_recipe(&name).map_err(|e| e.to_string())?;
            let var_overrides = input::parse_set_vars(&set_vars)?;
            let loaded_input = input::load_input_from(input, input_file, &[])?;
            let steps: Vec<_> = recipe
                .steps
                .iter()
                .map(|s| Step {
                    op: s.op.clone(),
                    args: s.args.clone(),
                })
                .collect();
            let result = steps::run_steps(
                &steps,
                loaded_input.bytes.clone(),
                loaded_input.supplied,
                &var_overrides,
                trace,
                hex,
            )?;
            if save {
                steps::save_to_history(&steps, Some(&name), &loaded_input.bytes, &result)?;
            }
            output::write_output(&result.final_output, hex)?;
        }

        PipelineAction::Delete {
            name,
            global,
            project,
            yes,
        } => {
            let scope = mutation_scope(global, project);
            if !yes {
                eprint!("Delete pipeline '{}'? [y/N] ", name);
                io::stderr().flush().ok();
                let mut ans = String::new();
                io::stdin().read_line(&mut ans).ok();
                if !ans.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }
            store::delete_recipe(&name, scope).map_err(|e| e.to_string())?;
            println!("Deleted '{}'.", name);
        }

        PipelineAction::Export {
            name,
            format,
            output,
        } => {
            let recipe = store::load_recipe(&name).map_err(|e| e.to_string())?;
            let out = store::export_recipe(&recipe, &format).map_err(|e| e.to_string())?;
            if let Some(path) = output {
                fs::write(&path, &out).map_err(|e| format!("write error: {e}"))?;
                println!("Exported to '{}'.", path.display());
            } else {
                println!("{}", out);
            }
        }

        PipelineAction::Import {
            file,
            name,
            global,
            project,
        } => {
            let scope = mutation_scope(global, project);
            let recipe =
                store::import_recipe(&file, name.as_deref(), scope).map_err(|e| e.to_string())?;
            println!(
                "Imported pipeline '{}' ({} step(s)).",
                recipe.name,
                recipe.steps.len()
            );
        }

        PipelineAction::Rename {
            old_name,
            new_name,
            global,
            project,
        } => {
            let scope = mutation_scope(global, project);
            let mut recipe =
                store::load_recipe_in_scope(&old_name, scope).map_err(|e| e.to_string())?;
            if old_name == new_name {
                return Ok(());
            }
            recipe.name = new_name.clone();
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            store::delete_recipe(&old_name, scope).map_err(|e| e.to_string())?;
            println!("Renamed '{}' → '{}'.", old_name, new_name);
        }
    }
    Ok(())
}
