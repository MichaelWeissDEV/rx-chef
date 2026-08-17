//! `rxchef run` — execute a single operation.

use rxchef::{execution, runtime};

use crate::cli::RunArgs;
use crate::error::CliError;
use crate::{input, output, steps};

pub(crate) fn cmd_run(a: RunArgs) -> Result<(), CliError> {
    let output_format = a.output.selected_format().map_err(CliError::InvalidInput)?;
    let input = input::load_input_from(a.input, a.input_file, &[]).map_err(CliError::StoreIo)?;
    let var_overrides = input::parse_set_vars(&a.set_vars).map_err(CliError::InvalidInput)?;
    let resolved = runtime::resolve_named_args(&a.operation, &a.named_args, &a.args)
        .map_err(CliError::Execution)?;
    let input_supplied = input.supplied;
    let output = execution::execute(execution::ExecutionRequest {
        input: input.bytes,
        input_supplied,
        recipe: vec![execution::RecipeStep {
            op: a.operation,
            args: resolved,
        }]
        .into(),
        variables: steps::execution_variables(&var_overrides),
        options: execution::ExecutionOptions::default(),
    })?
    .output;
    output::write_formatted_output(&output, output_format, a.output.output_file.as_deref())
        .map_err(CliError::StoreIo)
}

// ─── Pipe ─────────────────────────────────────────────────────────────────────
