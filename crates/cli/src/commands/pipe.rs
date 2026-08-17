//! `rxchef pipe` — execute an inline pipeline.

use crate::cli::{OutputFormat, PipeArgs};
use crate::{input, output, steps};

pub(crate) fn cmd_pipe(a: PipeArgs) -> Result<(), String> {
    if a.steps.is_empty() {
        return Err("no steps — usage: rxchef pipe \"to_hex,Space\" \"sha2,256\" -- Hello".into());
    }
    let output_format = a.output.selected_format()?;
    let var_overrides = input::parse_set_vars(&a.set_vars)?;
    let input = input::load_input_from(a.input, a.input_file, &[])?;
    let steps = a
        .steps
        .iter()
        .map(|s| steps::parse_step_str(s))
        .collect::<Result<Vec<_>, _>>()?;
    let input_bytes = input.bytes.clone();
    let result = steps::run_steps(
        &steps,
        input.bytes,
        input.supplied,
        &var_overrides,
        a.trace && output_format != OutputFormat::Json,
        output_format == OutputFormat::Hex,
    )?;
    if a.save {
        steps::save_to_history(&steps, None, &input_bytes, &result)?;
    }
    if a.output.output_file.is_some() {
        output::write_formatted_output(
            &result.final_output,
            output_format,
            a.output.output_file.as_deref(),
        )
    } else if output_format == OutputFormat::Json {
        output::write_json_pipe_output(&result, if a.trace { Some(&steps) } else { None })
    } else {
        output::write_formatted_output(&result.final_output, output_format, None)
    }
}

// ─── Recipe ───────────────────────────────────────────────────────────────────
