//! `rxchef history` — inspect and clear the run history.

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

use rxchef_store as store;

use crate::cli::{HistoryAction, HistoryArgs};
use crate::output;
use crate::steps::{self, Step};

pub(crate) fn cmd_history(a: HistoryArgs) -> Result<(), String> {
    match a.action {
        HistoryAction::List { limit, json } => {
            let entries = store::list_history(Some(limit));
            if json {
                let v: Vec<_> = entries
                    .iter()
                    .map(|e| {
                        serde_json::json!({
                            "id": e.id, "timestamp": e.timestamp,
                            "pipeline": e.pipeline_name,
                            "steps": e.steps.len(),
                            "input_preview": e.input_preview,
                            "output_preview": e.output_preview,
                            "success": e.success,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
                return Ok(());
            }
            if entries.is_empty() {
                println!("No history. Use --save with pipe/recipe/pipeline run to record runs.");
                return Ok(());
            }
            println!(
                "{:<20} {:<22} {:>5}  {}",
                "ID", "TIMESTAMP", "STEPS", "INPUT PREVIEW"
            );
            println!("{}", "-".repeat(75));
            for e in &entries {
                println!(
                    "{:<20} {:<22} {:>5}  {}",
                    e.id,
                    e.timestamp,
                    e.steps.len(),
                    e.input_preview
                );
            }
        }

        HistoryAction::Show { id } => {
            let entry = store::get_history(&id)
                .ok_or_else(|| format!("history entry '{}' not found", id))?;
            println!("ID:        {}", entry.id);
            println!("Timestamp: {}", entry.timestamp);
            if let Some(n) = &entry.pipeline_name {
                println!("Pipeline:  {}", n);
            }
            println!(
                "Input:     {} ({} bytes)",
                entry.input_preview, entry.input_bytes
            );
            println!("Success:   {}", entry.success);
            println!();
            for (i, step) in entry.steps.iter().enumerate() {
                println!("Step {}  {} [{}]", i + 1, step.op, step.args.join(", "));
                if let Some(e) = &step.error {
                    println!("  ERROR: {}", e);
                } else {
                    println!(
                        "  output ({} bytes, {:.3} ms): {}",
                        step.output_bytes, step.duration_ms, step.output_preview
                    );
                }
            }
            println!(
                "\nFinal output ({} bytes):\n{}",
                entry.output_bytes, entry.output_preview
            );
        }

        HistoryAction::Run {
            id,
            input,
            input_file,
            trace,
        } => {
            let entry = store::get_history(&id)
                .ok_or_else(|| format!("history entry '{}' not found", id))?;
            let input_bytes = if let Some(path) = input_file {
                fs::read(&path)
                    .map_err(|error| format!("cannot read '{}': {error}", path.display()))?
            } else if let Some(text) = input {
                text.into_bytes()
            } else {
                return Err(
                    "original input was not retained; provide --input or --input-file".into(),
                );
            };
            let steps: Vec<_> = entry
                .steps
                .iter()
                .map(|s| Step {
                    op: s.op.clone(),
                    args: s.args.clone(),
                })
                .collect();
            let result =
                steps::run_steps(&steps, input_bytes, true, &HashMap::new(), trace, false)?;
            output::write_output(&result.final_output, false)?;
        }

        HistoryAction::Clear { yes } => {
            if !yes {
                eprint!("Clear all run history? [y/N] ");
                io::stderr().flush().ok();
                let mut ans = String::new();
                io::stdin().read_line(&mut ans).ok();
                if !ans.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }
            store::clear_history().map_err(|e| e.to_string())?;
            println!("History cleared.");
        }
    }
    Ok(())
}

// ─── Magic ────────────────────────────────────────────────────────────────────
