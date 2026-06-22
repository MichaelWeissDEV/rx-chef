/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert Leet Speak operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Convert Leet Speak operation
///
/// Converts to and from Leet Speak.
pub struct ConvertLeetSpeak;

impl Operation for ConvertLeetSpeak {
    fn name(&self) -> &'static str {
        "Convert Leet Speak"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts to and from Leet Speak."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Direction",
            description: "Direction: To Leet Speak or From Leet Speak",
            default_value: "To Leet Speak",
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
        let direction = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("To Leet Speak");

        let input_str = String::from_utf8_lossy(&input);
        let result = if direction.to_lowercase().contains("leet") {
            // To Leet Speak
            input_str
                .chars()
                .map(|c| {
                    if c.is_ascii_alphabetic() {
                        let leet = lookup_to_leet(c.to_ascii_lowercase());
                        if c.is_uppercase() {
                            leet.to_ascii_uppercase()
                        } else {
                            leet
                        }
                    } else {
                        c
                    }
                })
                .collect::<String>()
        } else {
            // From Leet Speak
            input_str
                .chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() {
                        lookup_from_leet(c.to_ascii_lowercase()).unwrap_or(c)
                    } else {
                        c
                    }
                })
                .collect::<String>()
        };

        Ok(result.into_bytes())
    }
}

fn lookup_to_leet(c: char) -> char {
    match c {
        'a' => '4',
        'e' => '3',
        'i' => '1',
        'o' => '0',
        's' => '5',
        't' => '7',
        _ => c,
    }
}

fn lookup_from_leet(c: char) -> Option<char> {
    match c {
        '4' => Some('a'),
        '3' => Some('e'),
        '1' => Some('i'),
        '0' => Some('o'),
        '5' => Some('s'),
        '7' => Some('t'),
        'b' => Some('b'),
        'c' => Some('c'),
        'd' => Some('d'),
        'f' => Some('f'),
        'g' => Some('g'),
        'h' => Some('h'),
        'j' => Some('j'),
        'k' => Some('k'),
        'l' => Some('l'),
        'm' => Some('m'),
        'n' => Some('n'),
        'p' => Some('p'),
        'q' => Some('q'),
        'r' => Some('r'),
        'u' => Some('u'),
        'v' => Some('v'),
        'w' => Some('w'),
        'x' => Some('x'),
        'y' => Some('y'),
        'z' => Some('z'),
        _ => None,
    }
}
