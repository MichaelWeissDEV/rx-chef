/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert to NATO alphabet operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Convert to NATO alphabet operation
///
/// Converts characters to their representation in the NATO phonetic alphabet.
pub struct ConvertToNATOAlphabet;

impl Operation for ConvertToNATOAlphabet {
    fn name(&self) -> &'static str {
        "Convert to NATO alphabet"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts characters to their representation in the NATO phonetic alphabet."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let mut result = String::new();

        for c in input_str.chars() {
            if let Some(nato) = lookup(c.to_ascii_uppercase()) {
                result.push_str(nato);
                result.push(' ');
            } else {
                result.push(c);
            }
        }

        Ok(result.into_bytes())
    }
}

fn lookup(c: char) -> Option<&'static str> {
    match c {
        'A' => Some("Alfa"),
        'B' => Some("Bravo"),
        'C' => Some("Charlie"),
        'D' => Some("Delta"),
        'E' => Some("Echo"),
        'F' => Some("Foxtrot"),
        'G' => Some("Golf"),
        'H' => Some("Hotel"),
        'I' => Some("India"),
        'J' => Some("Juliett"),
        'K' => Some("Kilo"),
        'L' => Some("Lima"),
        'M' => Some("Mike"),
        'N' => Some("November"),
        'O' => Some("Oscar"),
        'P' => Some("Papa"),
        'Q' => Some("Quebec"),
        'R' => Some("Romeo"),
        'S' => Some("Sierra"),
        'T' => Some("Tango"),
        'U' => Some("Uniform"),
        'V' => Some("Victor"),
        'W' => Some("Whiskey"),
        'X' => Some("X-ray"),
        'Y' => Some("Yankee"),
        'Z' => Some("Zulu"),
        '0' => Some("Zero"),
        '1' => Some("One"),
        '2' => Some("Two"),
        '3' => Some("Three"),
        '4' => Some("Four"),
        '5' => Some("Five"),
        '6' => Some("Six"),
        '7' => Some("Seven"),
        '8' => Some("Eight"),
        '9' => Some("Nine"),
        ',' => Some("Comma"),
        '/' => Some("Fraction bar"),
        '.' => Some("Full stop"),
        _ => None,
    }
}
