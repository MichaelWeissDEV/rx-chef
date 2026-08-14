/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Snake case operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ToSnakeCase;

impl Operation for ToSnakeCase {
    fn name(&self) -> &'static str {
        "To Snake case"
    }
    fn module(&self) -> &'static str {
        "Code"
    }
    fn description(&self) -> &'static str {
        "Converts the input string to snake case.\n\nSnake case is all lower case with underscores as word boundaries.\n\ne.g. this_is_snake_case"
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Attempt to be context aware",
            description: "Attempt to nicely transform variable and function names.",
            default_value: "false",
            kind: crate::operation::ArgKind::Boolean,
            required: false,
            choices: &[],
            minimum: None,
            maximum: None,
            sensitive: false,
        }];
        SCHEMA
    }
    fn input_type(&self) -> DataType {
        DataType::String
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&_input);
        let smart = args.first().and_then(|v| v.as_bool()).unwrap_or(false);

        let result = if smart {
            replace_variable_names(&input_str, snake_case)
        } else {
            snake_case(&input_str)
        };

        Ok(result.into_bytes())
    }
}

fn get_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    for character in s.chars() {
        if !character.is_alphanumeric() {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }
        if character.is_uppercase()
            && current
                .chars()
                .last()
                .is_some_and(|previous| previous.is_lowercase() || previous.is_numeric())
        {
            words.push(std::mem::take(&mut current));
        } else if character.is_lowercase()
            && current.chars().count() > 1
            && current.chars().all(|value| value.is_uppercase())
        {
            let last = current.pop().unwrap();
            words.push(std::mem::take(&mut current));
            current.push(last);
        }
        current.push(character);
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

fn snake_case(s: &str) -> String {
    let words = get_words(s);
    words
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

fn replace_variable_names<F>(input: &str, replacer: F) -> String
where
    F: Fn(&str) -> String,
{
    let re = Regex::new(r#"(?i)\\"|"(?:\\"|[^"])*"|(\b[a-z0-9\-_]+\b)"#).unwrap();

    re.replace_all(input, |caps: &regex::Captures| {
        if let Some(m) = caps.get(1) {
            replacer(m.as_str())
        } else {
            caps.get(0).unwrap().as_str().to_string()
        }
    })
    .into_owned()
}
