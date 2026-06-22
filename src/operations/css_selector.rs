/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CSS selector operation.
 * -----------------------------------------------------------------------------
 */

use scraper::{Html, Selector};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct CssSelector;

impl Operation for CssSelector {
    fn name(&self) -> &'static str {
        "CSS selector"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Extract information from an HTML document with a CSS selector"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[
            ArgSchema {
                name: "CSS selector",
                description: "The CSS selector query.",
                default_value: "",
            },
            ArgSchema {
                name: "Delimiter",
                description: "The character(s) to delimit the output.",
                default_value: "\\n",
            },
        ]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let query = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let delimiter_arg = args.get(1).and_then(|a| a.as_str()).unwrap_or("\\n");
        let delimiter = delimiter_arg
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");

        if query.is_empty() || input.is_empty() {
            return Ok(Vec::new());
        }

        let input_str = String::from_utf8_lossy(&input);
        let document = Html::parse_document(&input_str);

        let selector = Selector::parse(query).map_err(|e| OperationError::InvalidArgument {
            name: "CSS selector".to_string(),
            reason: format!("Invalid CSS Selector. Details:\n{:?}", e),
        })?;

        let results: Vec<String> = document
            .select(&selector)
            .map(|element| element.html())
            .collect();

        Ok(results.join(&delimiter).into_bytes())
    }
}
