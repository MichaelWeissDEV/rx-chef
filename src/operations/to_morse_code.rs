/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Morse Code operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashMap;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Morse Code operation
pub struct ToMorseCode;

impl Operation for ToMorseCode {
    fn name(&self) -> &'static str {
        "To Morse Code"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Translates alphanumeric characters into International Morse Code.<br><br>Ignores non-Morse characters.<br><br>e.g. <code>SOS</code> becomes <code>... --- ...</code>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Format options",
                description: "The characters to use for dots and dashes",
                default_value: "-/.",
            },
            ArgSchema {
                name: "Letter delimiter",
                description: "The delimiter between letters",
                default_value: "Space",
            },
            ArgSchema {
                name: "Word delimiter",
                description: "The delimiter between words",
                default_value: "Line feed",
            },
        ];
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

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let format_arg = args.first().and_then(|v| v.as_str()).unwrap_or("-/.");
        let letter_delim_arg = args.get(1).and_then(|v| v.as_str()).unwrap_or("Space");
        let word_delim_arg = args.get(2).and_then(|v| v.as_str()).unwrap_or("Line feed");

        let format_parts: Vec<&str> = format_arg.split('/').collect();
        if format_parts.len() != 2 {
            return Err(OperationError::InvalidArgument {
                name: "Format options".to_string(),
                reason: "Format must be in the form 'dash/dot'".to_string(),
            });
        }
        let dash = format_parts[0];
        let dot = format_parts[1];

        let letter_delim = parse_morse_delimiter(letter_delim_arg);
        let word_delim = parse_morse_delimiter(word_delim_arg);

        let char_to_morse = get_char_to_morse();

        let lines: Vec<&str> = input_str.split('\n').collect();
        let mut result_lines = Vec::new();

        for line in lines {
            let line = line.trim_end_matches('\r');
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut result_words = Vec::new();

            for word in words {
                let mut result_letters = Vec::new();
                for c in word.chars() {
                    let upper_c = c.to_ascii_uppercase();
                    if let Some(morse) = char_to_morse.get(&upper_c) {
                        result_letters.push(morse.replace("<dash>", dash).replace("<dot>", dot));
                    }
                }
                result_words.push(result_letters.join(&letter_delim));
            }
            result_lines.push(result_words.join(&word_delim));
        }

        Ok(result_lines.join("\n").into_bytes())
    }
}

fn parse_morse_delimiter(delim: &str) -> String {
    match delim {
        "Space" => " ".to_string(),
        "Comma" => ",".to_string(),
        "Semi-colon" => ";".to_string(),
        "Colon" => ":".to_string(),
        "Line feed" => "\n".to_string(),
        "CRLF" => "\r\n".to_string(),
        "None" => "".to_string(),
        _ => delim.to_string(),
    }
}

fn get_char_to_morse() -> HashMap<char, &'static str> {
    let mut m = HashMap::new();
    m.insert('A', "<dot><dash>");
    m.insert('B', "<dash><dot><dot><dot>");
    m.insert('C', "<dash><dot><dash><dot>");
    m.insert('D', "<dash><dot><dot>");
    m.insert('E', "<dot>");
    m.insert('F', "<dot><dot><dash><dot>");
    m.insert('G', "<dash><dash><dot>");
    m.insert('H', "<dot><dot><dot><dot>");
    m.insert('I', "<dot><dot>");
    m.insert('J', "<dot><dash><dash><dash>");
    m.insert('K', "<dash><dot><dash>");
    m.insert('L', "<dot><dash><dot><dot>");
    m.insert('M', "<dash><dash>");
    m.insert('N', "<dash><dot>");
    m.insert('O', "<dash><dash><dash>");
    m.insert('P', "<dot><dash><dash><dot>");
    m.insert('Q', "<dash><dash><dot><dash>");
    m.insert('R', "<dot><dash><dot>");
    m.insert('S', "<dot><dot><dot>");
    m.insert('T', "<dash>");
    m.insert('U', "<dot><dot><dash>");
    m.insert('V', "<dot><dot><dot><dash>");
    m.insert('W', "<dot><dash><dash>");
    m.insert('X', "<dash><dot><dot><dash>");
    m.insert('Y', "<dash><dot><dash><dash>");
    m.insert('Z', "<dash><dash><dot><dot>");
    m.insert('1', "<dot><dash><dash><dash><dash>");
    m.insert('2', "<dot><dot><dash><dash><dash>");
    m.insert('3', "<dot><dot><dot><dash><dash>");
    m.insert('4', "<dot><dot><dot><dot><dash>");
    m.insert('5', "<dot><dot><dot><dot><dot>");
    m.insert('6', "<dash><dot><dot><dot><dot>");
    m.insert('7', "<dash><dash><dot><dot><dot>");
    m.insert('8', "<dash><dash><dash><dot><dot>");
    m.insert('9', "<dash><dash><dash><dash><dot>");
    m.insert('0', "<dash><dash><dash><dash><dash>");
    m.insert('.', "<dot><dash><dot><dash><dot><dash>");
    m.insert(',', "<dash><dash><dot><dot><dash><dash>");
    m.insert(':', "<dash><dash><dash><dot><dot><dot>");
    m.insert(';', "<dash><dot><dash><dot><dash><dot>");
    m.insert('!', "<dash><dot><dash><dot><dash><dash>");
    m.insert('?', "<dot><dot><dash><dash><dot><dot>");
    m.insert('\'', "<dot><dash><dash><dash><dash><dot>");
    m.insert('"', "<dot><dash><dot><dot><dash><dot>");
    m.insert('/', "<dash><dot><dot><dash><dot>");
    m.insert('-', "<dash><dot><dot><dot><dot><dash>");
    m.insert('+', "<dot><dash><dot><dash><dot>");
    m.insert('(', "<dash><dot><dash><dash><dot>");
    m.insert(')', "<dash><dot><dash><dash><dot><dash>");
    m.insert('@', "<dot><dash><dash><dot><dash><dot>");
    m.insert('=', "<dash><dot><dot><dot><dash>");
    m.insert('&', "<dot><dash><dot><dot><dot>");
    m.insert('_', "<dot><dot><dash><dash><dot><dash>");
    m.insert('$', "<dot><dot><dot><dash><dot><dot><dash>");
    m.insert(' ', "<dot><dot><dot><dot><dot><dot><dot>");
    m
}
