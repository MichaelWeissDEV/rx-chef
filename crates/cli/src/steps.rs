//! Pipeline step model and execution.
//!
//! Shared by `run`, `pipe`, `recipe`, `bake`, `pipeline`, and the interactive
//! shell, so every entry point resolves arguments and records history the same
//! way.

use std::collections::HashMap;

use rxchef::{execution, runtime};
use rxchef_store as store;

pub(crate) struct Step {
    pub(crate) op: String,
    pub(crate) args: Vec<String>,
}

pub(crate) struct RunResult {
    pub(crate) final_output: Vec<u8>,
    pub(crate) steps: Vec<store::HistoryStep>,
}

pub(crate) fn run_steps(
    steps: &[Step],
    input: Vec<u8>,
    input_supplied: bool,
    var_overrides: &HashMap<String, String>,
    trace: bool,
    _hex: bool,
) -> Result<RunResult, String> {
    let recipe = steps
        .iter()
        .map(|step| execution::RecipeStep {
            op: step.op.clone(),
            args: step.args.clone(),
        })
        .collect::<Vec<_>>();
    let outcome = execution::execute(execution::ExecutionRequest {
        input,
        input_supplied,
        recipe: recipe.clone().into(),
        variables: execution_variables(var_overrides),
        options: execution::ExecutionOptions {
            // History needs byte counts even when the user does not render a trace.
            trace: true,
            ..execution::ExecutionOptions::default()
        },
    })
    .map_err(|error| error.to_string())?;

    if trace {
        eprintln!("STEP  OPERATION                     INPUT       OUTPUT      TIME");
        for entry in &outcome.trace {
            eprintln!(
                "{:<5} {:<29} {:>8} B  {:>8} B  {:>8.3} ms",
                entry.step_index + 1,
                entry.operation,
                entry.input_bytes,
                entry.output_bytes,
                entry.elapsed.as_secs_f64() * 1_000.0
            );
        }
    }

    let last = recipe.len().saturating_sub(1);
    let history_steps = recipe
        .into_iter()
        .enumerate()
        .map(|(index, step)| {
            let output_bytes = outcome
                .trace
                .iter()
                .rev()
                .find(|entry| entry.step_index == index)
                .map_or(0, |entry| entry.output_bytes);
            let duration_ms = outcome
                .trace
                .iter()
                .rev()
                .find(|entry| entry.step_index == index)
                .map_or(0.0, |entry| entry.elapsed.as_secs_f64() * 1_000.0);
            let arguments = redact_sensitive_args(&step.op, &step.args);
            store::HistoryStep {
                op: step.op,
                // Keep unexpanded arguments so variable values are not copied into history.
                args: arguments,
                output_preview: if index == last {
                    store::bytes_preview(&outcome.output, 300)
                } else {
                    String::new()
                },
                output_bytes,
                duration_ms,
                error: None,
            }
        })
        .collect();
    Ok(RunResult {
        final_output: outcome.output,
        steps: history_steps,
    })
}

pub(crate) fn redact_sensitive_args(operation: &str, arguments: &[String]) -> Vec<String> {
    runtime::redact_sensitive_args(operation, arguments)
}

pub(crate) fn execution_variables(
    overrides: &HashMap<String, String>,
) -> execution::VariableContext {
    let mut context = execution::VariableContext::new(
        store::list_vars(None)
            .into_iter()
            .map(|variable| (variable.name, variable.value)),
    );
    for (name, value) in overrides {
        context.insert(name, value.clone());
    }
    context
}

pub(crate) fn save_to_history(
    _steps: &[Step],
    pipeline_name: Option<&str>,
    input: &[u8],
    result: &RunResult,
) -> Result<(), String> {
    let entry = store::HistoryEntry {
        id: store::new_history_id(),
        timestamp: chrono_now(),
        pipeline_name: pipeline_name.map(|s| s.to_string()),
        input_preview: store::bytes_preview(input, 300),
        input_bytes: input.len(),
        steps: result.steps.clone(),
        output_preview: store::bytes_preview(&result.final_output, 300),
        output_bytes: result.final_output.len(),
        success: true,
    };
    store::append_history(&entry).map_err(|e| e.to_string())
}

pub(crate) fn chrono_now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

// ─── Step parsing ─────────────────────────────────────────────────────────────

pub(crate) fn parse_step_str(s: &str) -> Result<Step, String> {
    let fields = split_step_fields(s)?;
    let op = fields.first().cloned().unwrap_or_default();
    if op.is_empty() {
        return Err(format!("invalid empty operation in step '{s}'"));
    }
    Ok(Step {
        op,
        args: fields.into_iter().skip(1).collect(),
    })
}

/// Split the compact CLI step format while allowing commas in arguments.
///
/// Both single and double quotes group fields. A backslash escapes comma,
/// quote, or backslash; before any other character it is kept literally so
/// regular expressions such as `\d+` survive parsing.
pub(crate) fn split_step_fields(s: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quote: Option<char> = None;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => match chars.peek().copied() {
                Some(next) if next == ',' || next == '\\' || Some(next) == quote => {
                    field.push(chars.next().expect("peeked character"));
                }
                _ => field.push('\\'),
            },
            '\'' | '"' => match quote {
                Some(active) if active == ch => quote = None,
                None => quote = Some(ch),
                Some(_) => field.push(ch),
            },
            ',' if quote.is_none() => {
                fields.push(field.trim().to_string());
                field.clear();
            }
            _ => field.push(ch),
        }
    }

    if let Some(unclosed) = quote {
        return Err(format!("invalid step '{s}': unclosed {unclosed} quote"));
    }
    fields.push(field.trim().to_string());
    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::{parse_step_str, redact_sensitive_args, split_step_fields};

    #[test]
    fn parses_plain_pipeline_step() {
        let step = parse_step_str("to_hex, Space, num:0").unwrap();
        assert_eq!(step.op, "to_hex");
        assert_eq!(step.args, ["Space", "num:0"]);
    }

    #[test]
    fn parses_quoted_and_escaped_commas() {
        assert_eq!(
            split_step_fields(r#"find_replace,"a,b",x\,y"#).unwrap(),
            ["find_replace", "a,b", "x,y"]
        );
    }

    #[test]
    fn preserves_regular_expression_backslashes() {
        let step = parse_step_str(r"regular_expression,User,\\d+").unwrap();
        assert_eq!(step.args[1], r"\d+");
    }

    #[test]
    fn rejects_unclosed_quotes_and_empty_operations() {
        assert!(parse_step_str(r#"op,"unterminated"#).is_err());
        assert!(parse_step_str(" ,arg").is_err());
    }

    #[test]
    fn history_arguments_redact_sensitive_schema_fields() {
        assert_eq!(
            redact_sensitive_args(
                "PGP Decrypt",
                &["private-key-material".into(), "passphrase".into()]
            ),
            ["<redacted>", "<redacted>"]
        );
    }
}
