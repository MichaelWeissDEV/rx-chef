/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Morse Code operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashMap;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Morse Code operation
pub struct FromMorseCode;

impl Operation for FromMorseCode {
    fn name(&self) -> &'static str {
        "From Morse Code"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Translates Morse Code into (upper case) alphanumeric characters."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
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

        let letter_delim_arg = args.first().and_then(|v| v.as_str()).unwrap_or("Space");
        let word_delim_arg = args.get(1).and_then(|v| v.as_str()).unwrap_or("Line feed");

        let letter_delim = parse_morse_delimiter(letter_delim_arg);
        let word_delim = parse_morse_delimiter(word_delim_arg);

        // Normalize signals
        let normalized = input_str
            .replace("-", "<dash>")
            .replace("", "<dash>")
            .replace("", "<dash>")
            .replace("_", "<dash>")
            .replace("", "<dash>")
            .replace("", "<dash>")
            .replace("dash", "<dash>")
            .replace("DASH", "<dash>")
            .replace(".", "<dot>")
            .replace("", "<dot>")
            .replace("dot", "<dot>")
            .replace("DOT", "<dot>");

        let morse_to_char = get_morse_table();

        let mut result = Vec::new();
        let words: Vec<&str> = normalized.split(&word_delim).collect();
        for (i, word) in words.iter().enumerate() {
            if i > 0 {
                result.push(b' ');
            }
            let signals: Vec<&str> = word.split(&letter_delim).collect();
            for signal in signals {
                if signal.is_empty() {
                    continue;
                }
                if let Some(&c) = morse_to_char.get(signal) {
                    result.push(c as u8);
                }
            }
        }

        Ok(result)
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

fn get_morse_table() -> HashMap<&'static str, char> {
    let mut m = HashMap::new();
    m.insert("<dot><dash>", 'A');
    m.insert("<dash><dot><dot><dot>", 'B');
    m.insert("<dash><dot><dash><dot>", 'C');
    m.insert("<dash><dot><dot>", 'D');
    m.insert("<dot>", 'E');
    m.insert("<dot><dot><dash><dot>", 'F');
    m.insert("<dash><dash><dot>", 'G');
    m.insert("<dot><dot><dot><dot>", 'H');
    m.insert("<dot><dot>", 'I');
    m.insert("<dot><dash><dash><dash>", 'J');
    m.insert("<dash><dot><dash>", 'K');
    m.insert("<dot><dash><dot><dot>", 'L');
    m.insert("<dash><dash>", 'M');
    m.insert("<dash><dot>", 'N');
    m.insert("<dash><dash><dash>", 'O');
    m.insert("<dot><dash><dash><dot>", 'P');
    m.insert("<dash><dash><dot><dash>", 'Q');
    m.insert("<dot><dash><dot>", 'R');
    m.insert("<dot><dot><dot>", 'S');
    m.insert("<dash>", 'T');
    m.insert("<dot><dot><dash>", 'U');
    m.insert("<dot><dot><dot><dash>", 'V');
    m.insert("<dot><dash><dash>", 'W');
    m.insert("<dash><dot><dot><dash>", 'X');
    m.insert("<dash><dot><dash><dash>", 'Y');
    m.insert("<dash><dash><dot><dot>", 'Z');
    m.insert("<dot><dash><dash><dash><dash>", '1');
    m.insert("<dot><dot><dash><dash><dash>", '2');
    m.insert("<dot><dot><dot><dash><dash>", '3');
    m.insert("<dot><dot><dot><dot><dash>", '4');
    m.insert("<dot><dot><dot><dot><dot>", '5');
    m.insert("<dash><dot><dot><dot><dot>", '6');
    m.insert("<dash><dash><dot><dot><dot>", '7');
    m.insert("<dash><dash><dash><dot><dot>", '8');
    m.insert("<dash><dash><dash><dash><dot>", '9');
    m.insert("<dash><dash><dash><dash><dash>", '0');
    m.insert("<dot><dash><dot><dash><dot><dash>", '.');
    m.insert("<dash><dash><dot><dot><dash><dash>", ',');
    m.insert("<dash><dash><dash><dot><dot><dot>", ':');
    m.insert("<dash><dot><dash><dot><dash><dot>", ';');
    m.insert("<dash><dot><dash><dot><dash><dash>", '!');
    m.insert("<dot><dot><dash><dash><dot><dot>", '?');
    m.insert("<dot><dash><dash><dash><dash><dot>", '\'');
    m.insert("<dot><dash><dot><dot><dash><dot>", '"');
    m.insert("<dash><dot><dot><dash><dot>", '/');
    m.insert("<dash><dot><dot><dot><dot><dash>", '-');
    m.insert("<dot><dash><dot><dash><dot>", '+');
    m.insert("<dash><dot><dash><dash><dot>", '(');
    m.insert("<dash><dot><dash><dash><dot><dash>", ')');
    m.insert("<dot><dash><dash><dot><dash><dot>", '@');
    m.insert("<dash><dot><dot><dot><dash>", '=');
    m.insert("<dot><dash><dot><dot><dot>", '&');
    m.insert("<dot><dot><dash><dash><dot><dash>", '_');
    m.insert("<dot><dot><dot><dash><dot><dot><dash>", '$');
    m.insert("<dot><dot><dot><dot><dot><dot><dot>", ' ');
    m
}
