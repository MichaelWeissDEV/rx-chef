/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Template operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Template operation
pub struct Template;

impl Operation for Template {
    fn name(&self) -> &'static str {
        "Template"
    }

    fn module(&self) -> &'static str {
        "Handlebars"
    }

    fn description(&self) -> &'static str {
        "Render a template with Handlebars/Mustache substituting variables using JSON input. Templates will be rendered to plain-text only, to prevent XSS."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Template definition (.handlebars)",
            description: "The template string",
            default_value: "",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let template_str = args.first().and_then(|v| v.as_str()).unwrap_or("");

        let json_input: Value = serde_json::from_slice(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid JSON input: {}", e)))?;

        // Note: CyberChef uses Handlebars. Since it's not available as a dependency,
        // we implement a basic {{variable}} substitution.
        let result = render_template(template_str, &json_input);

        Ok(result.into_bytes())
    }
}

fn render_template(template: &str, data: &Value) -> String {
    let mut result = String::new();
    let mut current = template;

    while let Some(start_index) = current.find("{{") {
        result.push_str(&current[..start_index]);
        let remaining = &current[start_index + 2..];

        if let Some(end_index) = remaining.find("}}") {
            let key = remaining[..end_index].trim();

            // Basic support for nested keys via dot notation
            let mut val = data;
            for part in key.split('.') {
                val = &val[part];
            }

            match val {
                Value::String(s) => result.push_str(s),
                Value::Number(n) => result.push_str(&n.to_string()),
                Value::Bool(b) => result.push_str(&b.to_string()),
                Value::Null => {}
                _ => result.push_str(&val.to_string()),
            }

            current = &remaining[end_index + 2..];
        } else {
            result.push_str("{{");
            current = remaining;
        }
    }
    result.push_str(current);
    result
}
