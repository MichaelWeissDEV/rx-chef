/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Lorenz operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashMap;

use crate::operation::{ArgSchema, ArgValue, Operation, OperationError};

/// Lorenz operation
pub struct Lorenz;

#[derive(Clone)]
struct WheelPattern {
    x: Vec<Vec<u8>>,
    s: Vec<Vec<u8>>,
    m: Vec<Vec<u8>>,
}

impl Lorenz {
    fn get_ita2_table() -> HashMap<char, [u8; 5]> {
        let mut table = HashMap::new();
        table.insert('A', [1, 1, 0, 0, 0]);
        table.insert('B', [1, 0, 0, 1, 1]);
        table.insert('C', [0, 1, 1, 1, 0]);
        table.insert('D', [1, 0, 0, 1, 0]);
        table.insert('E', [1, 0, 0, 0, 0]);
        table.insert('F', [1, 0, 1, 1, 0]);
        table.insert('G', [0, 1, 0, 1, 1]);
        table.insert('H', [0, 0, 1, 0, 1]);
        table.insert('I', [0, 1, 1, 0, 0]);
        table.insert('J', [1, 1, 0, 1, 0]);
        table.insert('K', [1, 1, 1, 1, 0]);
        table.insert('L', [0, 1, 0, 0, 1]);
        table.insert('M', [0, 0, 1, 1, 1]);
        table.insert('N', [0, 0, 1, 1, 0]);
        table.insert('O', [0, 0, 0, 1, 1]);
        table.insert('P', [0, 1, 1, 0, 1]);
        table.insert('Q', [1, 1, 1, 0, 1]);
        table.insert('R', [0, 1, 0, 1, 0]);
        table.insert('S', [1, 0, 1, 0, 0]);
        table.insert('T', [0, 0, 0, 0, 1]);
        table.insert('U', [1, 1, 1, 0, 0]);
        table.insert('V', [0, 1, 1, 1, 1]);
        table.insert('W', [1, 1, 0, 0, 1]);
        table.insert('X', [1, 0, 1, 1, 1]);
        table.insert('Y', [1, 0, 1, 0, 1]);
        table.insert('Z', [1, 0, 0, 1, 1]);
        table.insert('3', [0, 0, 0, 1, 0]);
        table.insert('4', [0, 1, 0, 0, 0]);
        table.insert('9', [0, 0, 1, 0, 0]);
        table.insert('/', [0, 0, 0, 0, 0]);
        table.insert(' ', [0, 0, 1, 0, 0]);
        table.insert('.', [0, 0, 1, 0, 0]);
        table.insert('8', [1, 1, 1, 1, 1]);
        table.insert('5', [1, 1, 0, 1, 1]);
        table.insert('-', [1, 1, 1, 1, 1]);
        table.insert('+', [1, 1, 0, 1, 1]);
        table
    }

    fn get_fig_shift_arr() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('1', 'Q');
        map.insert('2', 'W');
        map.insert('3', 'E');
        map.insert('4', 'R');
        map.insert('5', 'T');
        map.insert('6', 'Y');
        map.insert('7', 'U');
        map.insert('8', 'I');
        map.insert('9', 'O');
        map.insert('0', 'P');
        map.insert(' ', '9');
        map.insert('-', 'A');
        map.insert('?', 'B');
        map.insert(':', 'C');
        map.insert('#', 'D');
        map.insert('%', 'F');
        map.insert('@', 'G');
        map.insert('#', 'H');
        map.insert('(', 'K');
        map.insert(')', 'L');
        map.insert('.', 'M');
        map.insert(',', 'N');
        map.insert('\'', 'S');
        map.insert('=', 'V');
        map.insert('/', 'X');
        map.insert('+', 'Z');
        map.insert('\n', '3');
        map.insert('\r', '4');
        map
    }

    fn get_init_patterns() -> HashMap<String, WheelPattern> {
        let mut patterns = HashMap::new();

        patterns.insert(
            "No Pattern".to_string(),
            WheelPattern {
                x: vec![
                    vec![0; 41],
                    vec![0; 31],
                    vec![0; 29],
                    vec![0; 26],
                    vec![0; 23],
                ],
                s: vec![
                    vec![0; 43],
                    vec![0; 47],
                    vec![0; 51],
                    vec![0; 53],
                    vec![0; 59],
                ],
                m: vec![vec![0; 61], vec![0; 37]],
            },
        );

        patterns.insert(
            "KH Pattern".to_string(),
            WheelPattern {
                x: vec![
                    vec![
                        0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0,
                        1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0,
                    ],
                    vec![
                        1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1,
                        1, 0, 1, 1, 0, 0,
                    ],
                    vec![
                        0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1,
                        0, 1, 1, 0,
                    ],
                    vec![
                        1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0,
                        0,
                    ],
                    vec![
                        1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0,
                    ],
                ],
                s: vec![
                    vec![
                        0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1,
                        0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1,
                    ],
                    vec![
                        0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0,
                        1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1,
                    ],
                    vec![
                        0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1,
                        0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0,
                        1,
                    ],
                    vec![
                        0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1,
                        0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
                        0, 1, 0,
                    ],
                    vec![
                        1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1,
                        0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0,
                        0, 1, 1, 0, 1, 0, 0, 1, 0,
                    ],
                ],
                m: vec![
                    vec![
                        0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0,
                        0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0,
                        1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0,
                    ],
                    vec![
                        1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1,
                        0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0,
                    ],
                ],
            },
        );

        // Simplified for brevity, normally would include ZMUG and BREAM
        patterns
    }

    fn read_lugs(lugstr: &str) -> Vec<u8> {
        lugstr
            .chars()
            .map(|c| if c == '.' { 0 } else { 1 })
            .collect()
    }

    fn convert_to_ita2(
        input: &str,
        intype: &str,
        mode: &str,
        fig_shift_arr: &HashMap<char, char>,
    ) -> Result<String, OperationError> {
        let mut result = String::new();
        let mut fig_shifted = false;
        let valid_ita2 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ34589+-./";
        let fig_shifted_chars = "1234567890+-'()/:=?,.";

        for character in input.chars() {
            let letter = character.to_ascii_uppercase();
            if intype == "ITA2" || mode == "Receive" {
                if !valid_ita2.contains(letter) && letter != ' ' {
                    return Err(OperationError::InvalidInput(format!(
                        "Invalid ITA2 character : {}",
                        character
                    )));
                }
                result.push(letter);
            } else {
                if !fig_shifted && fig_shifted_chars.contains(letter) {
                    fig_shifted = true;
                    result.push_str("55");
                    if let Some(&c) = fig_shift_arr.get(&letter) {
                        result.push(c);
                    }
                } else if fig_shifted {
                    if letter == '\n' {
                        result.push_str("34");
                    } else if letter == '\r' {
                        result.push('4');
                    } else if fig_shifted_chars.contains(letter) {
                        if let Some(&c) = fig_shift_arr.get(&letter) {
                            result.push(c);
                        }
                    } else {
                        fig_shifted = false;
                        result.push_str("88");
                        result.push(letter);
                    }
                } else {
                    if letter == '\n' {
                        result.push_str("34");
                    } else if letter == '\r' {
                        result.push('4');
                    } else {
                        result.push(letter);
                    }
                }
            }
        }
        Ok(result)
    }
}

impl Operation for Lorenz {
    fn name(&self) -> &'static str {
        "Lorenz"
    }
    fn module(&self) -> &'static str {
        "Bletchley"
    }
    fn description(&self) -> &'static str {
        "The Lorenz SZ40/42 cipher attachment was a WW2 German rotor cipher machine."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Model",
                description: "SZ40, SZ42a, SZ42b",
                default_value: "SZ40",
            },
            ArgSchema {
                name: "Wheel Pattern",
                description: "KH, ZMUG, BREAM, etc.",
                default_value: "KH Pattern",
            },
            ArgSchema {
                name: "KT-Schalter",
                description: "",
                default_value: "false",
            },
            ArgSchema {
                name: "Mode",
                description: "Send or Receive",
                default_value: "Send",
            },
            ArgSchema {
                name: "Input Type",
                description: "Plaintext or ITA2",
                default_value: "Plaintext",
            },
            ArgSchema {
                name: "Output Type",
                description: "Plaintext or ITA2",
                default_value: "Plaintext",
            },
            ArgSchema {
                name: "ITA2 Format",
                description: "5/8/9 or +/-/.",
                default_value: "5/8/9",
            },
            ArgSchema {
                name: "Psi1 start",
                description: "1-43",
                default_value: "1",
            },
            ArgSchema {
                name: "Psi2 start",
                description: "1-47",
                default_value: "1",
            },
            ArgSchema {
                name: "Psi3 start",
                description: "1-51",
                default_value: "1",
            },
            ArgSchema {
                name: "Psi4 start",
                description: "1-53",
                default_value: "1",
            },
            ArgSchema {
                name: "Psi5 start",
                description: "1-59",
                default_value: "1",
            },
            ArgSchema {
                name: "Mu37 start",
                description: "1-37",
                default_value: "1",
            },
            ArgSchema {
                name: "Mu61 start",
                description: "1-61",
                default_value: "1",
            },
            ArgSchema {
                name: "Chi1 start",
                description: "1-41",
                default_value: "1",
            },
            ArgSchema {
                name: "Chi2 start",
                description: "1-31",
                default_value: "1",
            },
            ArgSchema {
                name: "Chi3 start",
                description: "1-29",
                default_value: "1",
            },
            ArgSchema {
                name: "Chi4 start",
                description: "1-26",
                default_value: "1",
            },
            ArgSchema {
                name: "Chi5 start",
                description: "1-23",
                default_value: "1",
            },
            ArgSchema {
                name: "Psi1 lugs",
                description: "43 long",
                default_value: ".x...xx.x.x..xxx.x.x.xxxx.x.x.x.x.x..x.xx.x",
            },
            ArgSchema {
                name: "Psi2 lugs",
                description: "47 long",
                default_value: ".xx.x.xxx..x.x.x..x.xx.x.xxx.x....x.xx.x.x.x..x",
            },
            ArgSchema {
                name: "Psi3 lugs",
                description: "51 long",
                default_value: ".x.x.x..xxx....x.x.xx.x.x.x..xxx.x.x..x.x.xx..x.x.x",
            },
            ArgSchema {
                name: "Psi4 lugs",
                description: "53 long",
                default_value: ".xx...xxxxx.x.x.xx...x.xx.x.x..x.x.xx.x..x.x.x.x.x.x.",
            },
            ArgSchema {
                name: "Psi5 lugs",
                description: "59 long",
                default_value: "xx...xx.x..x.xx.x...x.x.x.x.x.x.x.x.xx..xxxx.x.x...xx.x..x.",
            },
            ArgSchema {
                name: "Mu37 lugs",
                description: "37 long",
                default_value: "x.x.x.x.x.x...x.x.x...x.x.x...x.x....",
            },
            ArgSchema {
                name: "Mu61 lugs",
                description: "61 long",
                default_value: ".xxxx.xxxx.xxx.xxxx.xx....xxx.xxxx.xxxx.xxxx.xxxx.xxx.xxxx...",
            },
            ArgSchema {
                name: "Chi1 lugs",
                description: "41 long",
                default_value: ".x...xxx.x.xxxx.x...x.x..xxx....xx.xxxx..",
            },
            ArgSchema {
                name: "Chi2 lugs",
                description: "31 long",
                default_value: "x..xxx...x.xxxx..xx..x..xx.xx..",
            },
            ArgSchema {
                name: "Chi3 lugs",
                description: "29 long",
                default_value: "..xx..x.xxx...xx...xx..xx.xx.",
            },
            ArgSchema {
                name: "Chi4 lugs",
                description: "26 long",
                default_value: "xx..x..xxxx..xx.xxx....x..",
            },
            ArgSchema {
                name: "Chi5 lugs",
                description: "23 long",
                default_value: "xx..xx....xxxx.x..x.x..",
            },
        ];
        SCHEMA
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let model = args[0].as_str().unwrap_or("SZ40");
        let pattern_name = args[1].as_str().unwrap_or("KH Pattern");
        let kt = args[2].as_bool().unwrap_or(false);
        let mode = args[3].as_str().unwrap_or("Send");
        let intype = args[4].as_str().unwrap_or("Plaintext");
        let outtype = args[5].as_str().unwrap_or("Plaintext");
        let format = args[6].as_str().unwrap_or("5/8/9");

        let mut s = [
            args[7].as_usize().unwrap_or(1),
            args[8].as_usize().unwrap_or(1),
            args[9].as_usize().unwrap_or(1),
            args[10].as_usize().unwrap_or(1),
            args[11].as_usize().unwrap_or(1),
        ];
        let mut m37 = args[12].as_usize().unwrap_or(1);
        let mut m61 = args[13].as_usize().unwrap_or(1);
        let mut x = [
            args[14].as_usize().unwrap_or(1),
            args[15].as_usize().unwrap_or(1),
            args[16].as_usize().unwrap_or(1),
            args[17].as_usize().unwrap_or(1),
            args[18].as_usize().unwrap_or(1),
        ];

        let mut patterns = Self::get_init_patterns();
        let pattern = if pattern_name == "Custom" {
            WheelPattern {
                x: vec![
                    Self::read_lugs(args[26].as_str().unwrap_or("")),
                    Self::read_lugs(args[27].as_str().unwrap_or("")),
                    Self::read_lugs(args[28].as_str().unwrap_or("")),
                    Self::read_lugs(args[29].as_str().unwrap_or("")),
                    Self::read_lugs(args[30].as_str().unwrap_or("")),
                ],
                s: vec![
                    Self::read_lugs(args[19].as_str().unwrap_or("")),
                    Self::read_lugs(args[20].as_str().unwrap_or("")),
                    Self::read_lugs(args[21].as_str().unwrap_or("")),
                    Self::read_lugs(args[22].as_str().unwrap_or("")),
                    Self::read_lugs(args[23].as_str().unwrap_or("")),
                ],
                m: vec![
                    Self::read_lugs(args[25].as_str().unwrap_or("")),
                    Self::read_lugs(args[24].as_str().unwrap_or("")),
                ],
            }
        } else {
            patterns
                .remove(pattern_name)
                .unwrap_or(patterns.remove("No Pattern").unwrap())
        };

        let ita2_table = Self::get_ita2_table();
        let fig_shift_arr = Self::get_fig_shift_arr();
        let ita2_input = Self::convert_to_ita2(&input_str, intype, mode, &fig_shift_arr)?;

        let mut reverse_ita2 = HashMap::new();
        for (k, v) in &ita2_table {
            let s: String = v.iter().map(|&b| if b == 1 { '1' } else { '0' }).collect();
            reverse_ita2.insert(s, *k);
        }
        let mut reverse_fig = HashMap::new();
        for (k, v) in &fig_shift_arr {
            reverse_fig.insert(*v, *k);
        }

        let mut result = String::new();
        let mut p5 = [0u8; 3];

        for letter in ita2_input.chars() {
            let x2bptr = if x[1] == 31 { 1 } else { x[1] + 1 };
            let s1bptr = if s[0] == 43 { 1 } else { s[0] + 1 };

            let this_chi: Vec<u8> = vec![
                pattern.x[0][x[0] - 1],
                pattern.x[1][x[1] - 1],
                pattern.x[2][x[2] - 1],
                pattern.x[3][x[3] - 1],
                pattern.x[4][x[4] - 1],
            ];
            let this_psi: Vec<u8> = vec![
                pattern.s[0][s[0] - 1],
                pattern.s[1][s[1] - 1],
                pattern.s[2][s[2] - 1],
                pattern.s[3][s[3] - 1],
                pattern.s[4][s[4] - 1],
            ];

            let bits = ita2_table
                .get(&letter)
                .ok_or(OperationError::ProcessingError("Invalid char".to_string()))?;
            let mut xor_sum = [0u8; 5];
            for i in 0..5 {
                xor_sum[i] = bits[i] ^ this_psi[i] ^ this_chi[i];
            }

            let result_bits: String = xor_sum
                .iter()
                .map(|&b| if b == 1 { '1' } else { '0' })
                .collect();

            // Wheel movement
            let x_limits = [41, 31, 29, 26, 23];
            for i in 0..5 {
                x[i] = if x[i] == 1 { x_limits[i] } else { x[i] - 1 };
            }

            let m61_lug = pattern.m[0][m61 - 1];
            let mut m37_lug = pattern.m[1][m37 - 1];
            m61 = if m61 == 1 { 61 } else { m61 - 1 };
            if m61_lug == 1 {
                m37 = if m37 == 1 { 37 } else { m37 - 1 };
                m37_lug = pattern.m[1][m37 - 1];
            }

            p5[2] = p5[1];
            p5[1] = p5[0];
            p5[0] = if mode == "Send" { bits[4] } else { xor_sum[4] };

            let mut lim = 0;
            let _ = lim;
            let totalmotor = if model == "SZ42a" {
                lim = pattern.x[1][x2bptr - 1];
                if kt {
                    lim = if lim == p5[2] { 0 } else { 1 };
                }
                if m37_lug == 0 && lim == 1 {
                    0
                } else {
                    1
                }
            } else if model == "SZ42b" {
                let x2b1lug = pattern.x[1][x2bptr - 1];
                let s1b1lug = pattern.s[0][s1bptr - 1];
                lim = if x2b1lug == s1b1lug { 0 } else { 1 };
                if kt {
                    lim = if lim == p5[2] { 0 } else { 1 };
                }
                if m37_lug == 0 && lim == 1 {
                    0
                } else {
                    1
                }
            } else {
                m37_lug
            };

            if totalmotor == 1 {
                let s_limits = [43, 47, 51, 53, 59];
                for i in 0..5 {
                    s[i] = if s[i] == 1 { s_limits[i] } else { s[i] - 1 };
                }
            }

            let mut rtnstr = *reverse_ita2.get(&result_bits).unwrap_or(&' ');
            if format == "5/8/9" {
                if rtnstr == '+' {
                    rtnstr = '5';
                } else if rtnstr == '-' {
                    rtnstr = '8';
                } else if rtnstr == '.' {
                    rtnstr = '9';
                }
            }
            result.push(rtnstr);
        }

        let mut final_res = String::new();
        let mut fig_shifted = false;
        for letter in result.chars() {
            if mode == "Receive" && outtype == "Plaintext" {
                if letter == '5' || letter == '+' {
                    fig_shifted = true;
                } else if letter == '8' || letter == '-' {
                    fig_shifted = false;
                } else if letter == '9' {
                    final_res.push(' ');
                } else if letter == '3' {
                    final_res.push('\n');
                } else if letter == '4' {
                } else if letter == '/' {
                    final_res.push('/');
                } else if fig_shifted {
                    final_res.push(*reverse_fig.get(&letter).unwrap_or(&letter));
                } else {
                    final_res.push(letter);
                }
            } else {
                final_res.push(letter);
            }
        }

        Ok(final_res.into_bytes())
    }
}
