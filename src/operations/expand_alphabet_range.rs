/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Expand alphabet range operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, Operation, OperationError};

pub struct ExpandAlphabetRange;

impl Operation for ExpandAlphabetRange {
    fn name(&self) -> &'static str {
        "Expand alphabet range"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Expand an alphabet range string into a list of the characters in that range.<br><br>e.g. <code>a-z</code> becomes <code>abcdefghijklmnopqrstuvwxyz</code>."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description: "The delimiter to use between each character",
            default_value: "",
        }];
        SCHEMA
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let delimiter = args.first().and_then(|v| v.as_str()).unwrap_or("");

        let expanded = expand_alph_range(&input_str);
        Ok(expanded.join(delimiter).into_bytes())
    }
}

pub fn expand_alph_range(alph_str: &str) -> Vec<String> {
    let mut alph_arr = Vec::new();
    let chars: Vec<char> = alph_str.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 2 < chars.len() && chars[i + 1] == '-' && (i == 0 || chars[i - 1] != '\\') {
            let start = chars[i] as u32;
            let end = chars[i + 2] as u32;
            for j in start..=end {
                if let Some(c) = std::char::from_u32(j) {
                    alph_arr.push(c.to_string());
                }
            }
            i += 3;
        } else if i + 1 < chars.len() && chars[i] == '\\' && chars[i + 1] == '-' {
            alph_arr.push("-".to_string());
            i += 2;
        } else {
            alph_arr.push(chars[i].to_string());
            i += 1;
        }
    }
    alph_arr
}
