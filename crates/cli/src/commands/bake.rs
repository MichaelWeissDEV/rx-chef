//! `rxchef bake` — execute a recipe file in CyberChef's exchange format.

use std::fs;

use crate::cli::BakeArgs;
use crate::{input, output};

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum BakeRecipe {
    Steps(Vec<rxchef::integration::RecipeStep>),
    Document {
        #[serde(default = "current_recipe_version")]
        version: u32,
        #[serde(alias = "pipeline")]
        steps: Vec<rxchef::integration::RecipeStep>,
    },
}

impl BakeRecipe {
    pub(crate) fn into_steps(self) -> Result<Vec<rxchef::integration::RecipeStep>, String> {
        match self {
            Self::Steps(steps) => Ok(steps),
            Self::Document { version, steps } if version == 1 => Ok(steps),
            Self::Document { version, .. } => Err(format!(
                "unsupported recipe version {version}; supported version is 1"
            )),
        }
    }
}

pub(crate) fn current_recipe_version() -> u32 {
    1
}

pub(crate) fn cmd_bake(a: BakeArgs) -> Result<(), String> {
    let output_format = a.output.selected_format()?;
    let (content, is_yaml) = match (a.recipe, a.recipe_json) {
        (Some(path), None) => {
            let content = fs::read_to_string(&path)
                .map_err(|error| format!("cannot read '{}': {error}", path.display()))?;
            let is_yaml = matches!(
                path.extension().and_then(|extension| extension.to_str()),
                Some("yaml" | "yml")
            );
            (content, is_yaml)
        }
        (None, Some(content)) => (content, false),
        _ => return Err("provide exactly one of --recipe or --recipe-json".into()),
    };
    let recipe: BakeRecipe = if is_yaml {
        serde_yaml::from_str(&content).map_err(|error| format!("invalid recipe YAML: {error}"))?
    } else {
        serde_json::from_str(&content).map_err(|error| format!("invalid recipe JSON: {error}"))?
    };
    let input = input::load_input_from(a.input, a.input_file, &[])?;
    let result =
        rxchef::integration::bake_with_input(input.bytes, input.supplied, &recipe.into_steps()?)?;
    output::write_formatted_output(
        &result.into_bytes()?,
        output_format,
        a.output.output_file.as_deref(),
    )
}
