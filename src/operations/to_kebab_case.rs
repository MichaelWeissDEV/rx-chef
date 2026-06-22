/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Kebab case operation.
 * -----------------------------------------------------------------------------
 */

use regex::{Captures, Regex};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Kebab case operation
pub struct ToKebabCase;

impl Operation for ToKebabCase {
    fn name(&self) -> &'static str {
        "To Kebab case"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Converts the input string to kebab case.\n<br><br>\nKebab case is all lower case with dashes as word boundaries.\n<br><br>\ne.g. this-is-kebab-case\n<br><br>\n'Attempt to be context aware' will make the operation attempt to nicely transform variable and function names."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Attempt to be context aware",
            description: "Attempt to nicely transform variable and function names.",
            default_value: "false",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let smart = args.first().and_then(|v| v.as_bool()).unwrap_or(false);

        let result = if smart {
            replace_variable_names(&input_str, kebab_case)
        } else {
            kebab_case(&input_str)
        };

        Ok(result.into_bytes())
    }
}

fn kebab_case(s: &str) -> String {
    let mut words = Vec::new();
    let mut current_word = String::new();

    for c in s.chars() {
        if c.is_alphanumeric() {
            if c.is_uppercase() && !current_word.is_empty() {
                // Split if last char was lowercase, or if next char (if any) is lowercase (e.g. "XMLHttp" -> "xml", "http")
                let last_char_lowercase = current_word.chars().last().unwrap().is_lowercase();
                if last_char_lowercase {
                    words.push(current_word.to_lowercase());
                    current_word = String::new();
                }
            }
            current_word.push(c);
        } else {
            if !current_word.is_empty() {
                words.push(current_word.to_lowercase());
                current_word = String::new();
            }
        }
    }
    if !current_word.is_empty() {
        words.push(current_word.to_lowercase());
    }

    words.join("-")
}

fn replace_variable_names(input: &str, replacer: fn(&str) -> String) -> String {
    let re = Regex::new(r#"(?i)\\"|"(?:\\"|[^"])*"|(\b[a-z0-9\-_]+\b)"#).unwrap();

    re.replace_all(input, |caps: &Captures| {
        if let Some(m) = caps.get(1) {
            replacer(m.as_str())
        } else {
            caps.get(0).unwrap().as_str().to_string()
        }
    })
    .to_string()
}
