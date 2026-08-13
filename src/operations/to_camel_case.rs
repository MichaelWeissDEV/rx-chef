/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Camel case operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ToCamelCase;

impl Operation for ToCamelCase {
    fn name(&self) -> &'static str {
        "To Camel case"
    }
    fn module(&self) -> &'static str {
        "Code"
    }
    fn description(&self) -> &'static str {
        "Converts the input string to camel case.\n\nCamel case is all lower case except letters after word boundaries which are uppercase.\n\ne.g. thisIsCamelCase"
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

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&_input);
        let smart = args.first().and_then(|v| v.as_bool()).unwrap_or(false);

        let result = if smart {
            replace_variable_names(&input_str, camel_case)
        } else {
            camel_case(&input_str)
        };

        Ok(result.into_bytes())
    }
}

fn get_words(s: &str) -> Vec<String> {
    split_words(s)
}

fn split_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    for character in input.chars() {
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

fn camel_case(s: &str) -> String {
    let words = get_words(s);
    if words.is_empty() {
        return String::new();
    }

    let mut result = words[0].to_lowercase();
    for word in &words[1..] {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            result.push(first.to_ascii_uppercase());
            result.extend(chars.map(|c| c.to_ascii_lowercase()));
        }
    }
    result
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
