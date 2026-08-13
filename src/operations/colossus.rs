/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Colossus operation.
 * -----------------------------------------------------------------------------
 */

use serde::Serialize;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub const VALID_ITA2: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ34589+-./ ";

pub fn get_ita2_bits(c: char) -> Option<&'static str> {
    match c {
        'A' => Some("11000"),
        'B' => Some("10011"),
        'C' => Some("01110"),
        'D' => Some("10010"),
        'E' => Some("10000"),
        'F' => Some("10110"),
        'G' => Some("01011"),
        'H' => Some("00101"),
        'I' => Some("01100"),
        'J' => Some("11010"),
        'K' => Some("11110"),
        'L' => Some("01001"),
        'M' => Some("00111"),
        'N' => Some("00110"),
        'O' => Some("00011"),
        'P' => Some("01101"),
        'Q' => Some("11101"),
        'R' => Some("01010"),
        'S' => Some("10100"),
        'T' => Some("00001"),
        'U' => Some("11100"),
        'V' => Some("01111"),
        'W' => Some("11001"),
        'X' => Some("10111"),
        'Y' => Some("10101"),
        'Z' => Some("10001"),
        '3' => Some("00010"),
        '4' => Some("01000"),
        '9' => Some("00100"),
        '/' => Some("00000"),
        ' ' => Some("00100"),
        '.' => Some("00100"),
        '8' => Some("11111"),
        '5' => Some("11011"),
        '-' => Some("11111"),
        '+' => Some("11011"),
        _ => None,
    }
}

pub struct Colossus;

#[derive(Serialize)]
struct ColossusResult {
    printout: String,
    counters: Vec<usize>,
    runcount: usize,
}

impl Operation for Colossus {
    fn name(&self) -> &'static str {
        "Colossus"
    }

    fn module(&self) -> &'static str {
        "Bletchley"
    }

    fn description(&self) -> &'static str {
        "Analyses an ITA2 teleprinter tape using the five parallel bit channels used by Colossus. The output contains a printable tape transcription, per-channel one-bit counters, and the number of tape characters processed."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input).to_uppercase();
        for c in input_str.chars() {
            if get_ita2_bits(c).is_none() {
                return Err(OperationError::InvalidInput(format!(
                    "Invalid ITA2 character : {}",
                    c
                )));
            }
        }

        let mut counters = vec![0usize; 5];
        let mut printout = String::new();
        for (index, character) in input_str.chars().enumerate() {
            let bits = get_ita2_bits(character).expect("validated ITA2 character");
            for (channel, bit) in bits.bytes().enumerate() {
                counters[channel] += usize::from(bit == b'1');
            }
            printout.push_str(&format!("{:04}  {}  {}\n", index + 1, character, bits));
        }
        let result = ColossusResult {
            printout,
            counters,
            runcount: input_str.chars().count(),
        };

        serde_json::to_vec(&result).map_err(|e| OperationError::ProcessingError(e.to_string()))
    }
}
