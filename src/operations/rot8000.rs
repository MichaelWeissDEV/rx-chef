/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ROT8000 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// ROT8000 cipher operation.
///
/// ROT8000 is a Caesar-cipher-style rotation for Unicode text. It builds a
/// list of "valid" BMP codepoints from a transition table and rotates each
/// valid character by half the total count. Characters not in the valid set
/// are passed through unchanged.
///
/// Port of https://rot8000.com based on the CyberChef implementation.
pub struct ROT8000;

/// Build the ROT8000 rotation table.
/// Valid code point transitions from the original JavaScript source:
///   { 33: true, 127: false, 161: true, 5760: false, 5761: true, 8192: false,
///     8203: true, 8232: false, 8234: true, 8239: false, 8240: true, 8287: false,
///     8288: true, 12288: false, 12289: true, 55296: false, 57344: true }
fn build_rot8000_table() -> Vec<(char, char)> {
    // Transition points: (codepoint, is_start_of_valid_range)
    // false means start of invalid range, true means start of valid range
    let transitions: &[(u32, bool)] = &[
        (33, true),
        (127, false),
        (161, true),
        (5760, false),
        (5761, true),
        (8192, false),
        (8203, true),
        (8232, false),
        (8234, true),
        (8239, false),
        (8240, true),
        (8287, false),
        (8288, true),
        (12288, false),
        (12289, true),
        (55296, false),
        (57344, true),
    ];

    // Build sorted list of valid codepoints in BMP (0..0x10000)
    let bmp_size: u32 = 0x10000;
    let mut valid_list: Vec<u32> = Vec::new();
    let mut curr_valid = false;
    let mut trans_iter = transitions.iter().peekable();

    for cp in 0..bmp_size {
        // Check if transition happens at this codepoint
        if let Some(&&(t_cp, t_val)) = trans_iter.peek() {
            if cp == t_cp {
                curr_valid = t_val;
                trans_iter.next();
            }
        }
        if curr_valid {
            valid_list.push(cp);
        }
    }

    let total = valid_list.len();
    let rotate_num = total / 2;

    // Build mapping: valid_list[i] -> valid_list[(i + rotate_num) % total]
    let mut table: Vec<(char, char)> = Vec::with_capacity(total);
    for (i, &cp) in valid_list.iter().enumerate() {
        let target_cp = valid_list[(i + rotate_num) % total];
        if let (Some(from_c), Some(to_c)) = (char::from_u32(cp), char::from_u32(target_cp)) {
            table.push((from_c, to_c));
        }
    }
    table
}

impl Operation for ROT8000 {
    fn name(&self) -> &'static str {
        "ROT8000"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "The simple Caesar-cypher encryption that replaces each Unicode character with the one 0x8000 places forward or back along the alphabet. Valid BMP codepoints are rotated; others pass through unchanged."
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
        let text =
            String::from_utf8(input).map_err(|e| OperationError::InvalidInput(e.to_string()))?;

        let table = build_rot8000_table();

        // Build a lookup from char -> char using a sorted vec + binary search
        // (simple approach: sort by from_c and binary search)
        let mut sorted_table = table;
        sorted_table.sort_by_key(|&(from, _)| from as u32);

        let output: String = text
            .chars()
            .map(|c| {
                match sorted_table.binary_search_by_key(&(c as u32), |&(from, _)| from as u32) {
                    Ok(idx) => sorted_table[idx].1,
                    Err(_) => c,
                }
            })
            .collect();

        Ok(output.into_bytes())
    }
}
