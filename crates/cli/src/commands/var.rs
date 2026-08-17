//! `rxchef var` — stored variables, including secret values.

use std::io::{self, Read};

use rxchef_store::{self as store, Scope};

use crate::cli::{VarAction, VarArgs};

pub(crate) fn mutation_scope(global: bool, project: bool) -> Scope {
    if global {
        Scope::Global
    } else if project {
        Scope::Project
    } else {
        store::default_scope()
    }
}

// ─── Var ──────────────────────────────────────────────────────────────────────

pub(crate) fn cmd_var(a: VarArgs) -> Result<(), String> {
    match a.action {
        VarAction::Set {
            name,
            value,
            description,
            global,
            project,
            secret,
            stdin,
        } => {
            let scope = mutation_scope(global, project);
            let value = if stdin {
                let mut value = String::new();
                io::stdin()
                    .read_to_string(&mut value)
                    .map_err(|error| format!("cannot read variable from stdin: {error}"))?;
                value
            } else {
                value.expect("clap requires VALUE unless --stdin is used")
            };
            store::set_var_with_options(
                &name,
                &value,
                description.as_deref().unwrap_or(""),
                secret,
                scope,
            )
            .map_err(|e| e.to_string())?;
            println!(
                "Set ${} ({})",
                name.to_uppercase(),
                if scope == Scope::Global {
                    "global"
                } else {
                    "project"
                }
            );
        }
        VarAction::Get { name } => match store::get_var(&name) {
            Some(v) => println!("{}", v),
            None => return Err(format!("variable '{}' not found", name)),
        },
        VarAction::List {
            global,
            project,
            json,
            show_values,
            show_secrets,
        } => {
            let scope = if global {
                Some(Scope::Global)
            } else if project {
                Some(Scope::Project)
            } else {
                None
            };
            let vars = store::list_vars_with_scope(scope);
            if json {
                let v: Vec<_> = vars
                    .iter()
                    .map(|(scope, v)| {
                        let value = if v.secret && !show_secrets {
                            None
                        } else if show_values || show_secrets {
                            Some(v.value.as_str())
                        } else {
                            None
                        };
                        serde_json::json!({
                            "name": v.name,
                            "scope": if *scope == Scope::Global { "global" } else { "project" },
                            "secret": v.secret,
                            "description": v.description,
                            "value": value,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
                return Ok(());
            }
            if vars.is_empty() {
                println!("No variables set. Use: rxchef var set <name> <value>");
                return Ok(());
            }
            println!(
                "{:<20} {:<9} {:<7}  {:<40}  {}",
                "NAME", "SCOPE", "SECRET", "VALUE", "DESCRIPTION"
            );
            println!("{}", "-".repeat(70));
            for (scope, v) in &vars {
                let value = if v.secret && !show_secrets {
                    "<redacted>"
                } else if show_values || show_secrets {
                    &v.value
                } else {
                    "-"
                };
                println!(
                    "{:<20} {:<9} {:<7}  {:<40}  {}",
                    v.name,
                    if *scope == Scope::Global {
                        "global"
                    } else {
                        "project"
                    },
                    if v.secret { "yes" } else { "no" },
                    value,
                    v.description
                );
            }
        }
        VarAction::Unset {
            name,
            global,
            project,
        } => {
            let scope = mutation_scope(global, project);
            store::unset_var(&name, scope).map_err(|e| e.to_string())?;
            println!("Removed ${}.", name.to_uppercase());
        }
    }
    Ok(())
}

// ─── History ──────────────────────────────────────────────────────────────────
