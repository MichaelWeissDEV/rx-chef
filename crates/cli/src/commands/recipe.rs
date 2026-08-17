//! `rxchef recipe` — execute a stored or on-disk recipe.

use rxchef_store as store;

use crate::cli::{OutputFormat, RecipeArgs};
use crate::steps::Step;
use crate::{input, output, steps};

pub(crate) fn cmd_recipe(a: RecipeArgs) -> Result<(), String> {
    let output_format = a.output.selected_format()?;
    let var_overrides = input::parse_set_vars(&a.set_vars)?;
    let recipe = load_recipe_arg(&a.recipe)?;
    let input = input::load_input_from(a.input, a.input_file, &[])?;
    let steps: Vec<_> = recipe
        .steps
        .iter()
        .map(|s| Step {
            op: s.op.clone(),
            args: s.args.clone(),
        })
        .collect();
    let input_bytes = input.bytes.clone();
    let result = steps::run_steps(
        &steps,
        input.bytes,
        input.supplied,
        &var_overrides,
        a.trace,
        output_format == OutputFormat::Hex,
    )?;
    if a.save {
        steps::save_to_history(&steps, Some(&recipe.name), &input_bytes, &result)?;
    }
    output::write_formatted_output(
        &result.final_output,
        output_format,
        a.output.output_file.as_deref(),
    )
}

pub(crate) fn load_recipe_arg(arg: &str) -> Result<store::Recipe, String> {
    // Loading a file is read-only. Import is an explicit pipeline command.
    let as_path = std::path::Path::new(arg);
    if as_path.exists() {
        return store::load_recipe_file(as_path).map_err(|error| error.to_string());
    }
    // Try named recipe from store
    if !arg.trim_start().starts_with('[') && !arg.trim_start().starts_with('{') {
        if let Ok(r) = store::load_recipe(arg) {
            return Ok(r);
        }
    }
    // Try inline JSON
    if arg.trim_start().starts_with('[') {
        let steps: Vec<store::RecipeStep> =
            serde_json::from_str(arg).map_err(|e| format!("invalid recipe JSON: {e}"))?;
        return Ok(store::Recipe {
            version: store::RECIPE_VERSION,
            name: "inline".into(),
            description: String::new(),
            steps,
            tags: vec![],
        });
    }
    Err(format!("recipe not found: '{}'", arg))
}
