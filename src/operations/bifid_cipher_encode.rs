/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bifid Cipher Encode operation.
 * -----------------------------------------------------------------------------
 */

use itertools::Itertools;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bifid Cipher Encode operation
///
/// The Bifid cipher is a cipher which uses a Polybius square in conjunction
/// with transposition, which can be fairly difficult to decipher without
/// knowing the alphabet keyword.
pub struct BifidCipherEncode;

impl Operation for BifidCipherEncode {
    fn name(&self) -> &'static str {
        "Bifid Cipher Encode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "The Bifid cipher is a cipher which uses a Polybius square in conjunction with transposition, which can be fairly difficult to decipher without knowing the alphabet keyword."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Keyword",
            description: "The keyword to use for the Polybius square",
            default_value: "",
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
        let keyword_str = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("")
            .to_uppercase()
            .replace('J', "I");

        if !keyword_str.is_empty() && !keyword_str.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(OperationError::InvalidInput(
                "The key must consist only of letters in the English alphabet".to_string(),
            ));
        }

        let alpha = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
        let polybius = gen_polybius_square(&keyword_str);

        let input_str = String::from_utf8_lossy(&input);
        let input_upper = input_str.replace('J', "I");

        let mut x_co: Vec<usize> = Vec::new();
        let mut y_co: Vec<usize> = Vec::new();
        let mut structure: Vec<String> = Vec::new();
        let mut output = String::new();
        let mut count = 0;

        for letter in input_upper.chars() {
            let alp_ind = alpha.contains(letter);
            if alp_ind {
                for i in 0..5 {
                    if let Some(col) = polybius[i].iter().position(|&c| c == letter) {
                        x_co.push(col);
                        y_co.push(i);
                        if alpha.contains(letter) {
                            structure.push("true".to_string());
                        } else {
                            structure.push("false".to_string());
                        }
                        break;
                    }
                }
            } else {
                structure.push(letter.to_string());
            }
        }

        let trans = format!(
            "{}{}",
            y_co.iter().map(|&c| c.to_string()).collect::<String>(),
            x_co.iter().map(|&c| c.to_string()).collect::<String>()
        );

        for pos in structure.iter() {
            if pos == "true" || pos == "false" {
                let coords: Vec<char> = trans.chars().skip(2 * count).take(2).collect();
                if coords.len() >= 2 {
                    let row = coords[0].to_digit(10).unwrap_or(0) as usize;
                    let col = coords[1].to_digit(10).unwrap_or(0) as usize;
                    let c = polybius[row][col];
                    output.push(if pos == "true" {
                        c
                    } else {
                        c.to_ascii_lowercase()
                    });
                    count += 1;
                }
            } else {
                output.push_str(pos);
            }
        }

        Ok(output.into_bytes())
    }
}

fn gen_polybius_square(keyword: &str) -> Vec<Vec<char>> {
    let mut key = keyword.to_uppercase();
    key = key.replace('J', "I");
    key = key.chars().unique().collect::<String>();

    let alpha = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
    let mut square_str = key;
    for c in alpha.chars() {
        if !square_str.contains(c) {
            square_str.push(c);
        }
    }

    let mut square = Vec::new();
    for row in square_str.as_bytes().chunks(5) {
        square.push(row.iter().map(|&b| b as char).collect::<Vec<char>>());
    }

    square
}
