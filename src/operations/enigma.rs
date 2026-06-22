/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Enigma operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashSet;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct Enigma;

struct Rotor {
    map: [u8; 26],
    rev_map: [u8; 26],
    steps: HashSet<u8>,
    pos: u8,
}

impl Rotor {
    fn new(
        wiring: &str,
        steps_str: &str,
        ring_setting: char,
        initial_position: char,
    ) -> Result<Self, String> {
        if wiring.len() != 26 {
            return Err("Wiring must be 26 chars".to_string());
        }
        let mut map = [0u8; 26];
        let mut rev_map = [0u8; 26];
        for (i, c) in wiring.chars().enumerate() {
            let b = (c as u8).wrapping_sub(b'A');
            if b >= 26 {
                return Err("Invalid wiring char".to_string());
            }
            map[i] = b;
            rev_map[b as usize] = i as u8;
        }

        let rs = (ring_setting as u8).wrapping_sub(b'A');
        let mut steps = HashSet::new();
        for c in steps_str.chars() {
            let s = (c as u8).wrapping_sub(b'A');
            steps.insert((s + 26 - rs) % 26);
        }

        let pos = ((initial_position as u8).wrapping_sub(b'A') + 26 - rs) % 26;
        Ok(Self {
            map,
            rev_map,
            steps,
            pos,
        })
    }

    fn step(&mut self) -> u8 {
        self.pos = (self.pos + 1) % 26;
        self.pos
    }

    fn transform(&self, c: u8) -> u8 {
        let input = (c + self.pos) % 26;
        (self.map[input as usize] + 26 - self.pos) % 26
    }

    fn rev_transform(&self, c: u8) -> u8 {
        let input = (c + self.pos) % 26;
        (self.rev_map[input as usize] + 26 - self.pos) % 26
    }
}

struct PairMap {
    map: [u8; 26],
}

impl PairMap {
    fn new(pairs_str: &str) -> Result<Self, String> {
        let mut map: Vec<u8> = (0..26).collect();
        for pair in pairs_str.split_whitespace() {
            if pair.len() != 2 {
                continue;
            }
            let a = (pair.chars().nth(0).unwrap() as u8).wrapping_sub(b'A');
            let b = (pair.chars().nth(1).unwrap() as u8).wrapping_sub(b'A');
            if a < 26 && b < 26 {
                map[a as usize] = b;
                map[b as usize] = a;
            }
        }
        let mut arr = [0u8; 26];
        arr.copy_from_slice(&map);
        Ok(Self { map: arr })
    }

    fn transform(&self, c: u8) -> u8 {
        self.map[c as usize]
    }
}

impl Operation for Enigma {
    fn name(&self) -> &'static str {
        "Enigma"
    }

    fn module(&self) -> &'static str {
        "Bletchley"
    }

    fn description(&self) -> &'static str {
        "Encipher/decipher with the WW2 Enigma machine."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Model",
                description: "3-rotor or 4-rotor",
                default_value: "3-rotor",
            },
            ArgSchema {
                name: "4th Rotor",
                description: "Wiring<Steps",
                default_value: "",
            },
            ArgSchema {
                name: "4th Ring",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "4th Pos",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "Left Rotor",
                description: "Wiring<Steps",
                default_value: "EKMFLGDQVZNTOWYHXUSPAIBRCJ<R",
            },
            ArgSchema {
                name: "Left Ring",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "Left Pos",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "Middle Rotor",
                description: "Wiring<Steps",
                default_value: "AJDKSIRUXBLHWTMCQGZNPYFVOE<F",
            },
            ArgSchema {
                name: "Middle Ring",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "Middle Pos",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "Right Rotor",
                description: "Wiring<Steps",
                default_value: "BDFHJLCPRTXVZNYEIWGAKMUSQO<W",
            },
            ArgSchema {
                name: "Right Ring",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "Right Pos",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "Reflector",
                description: "Pairs",
                default_value: "AY BR CU DH EQ FS GL IP JX KN MO TZ VW",
            },
            ArgSchema {
                name: "Plugboard",
                description: "Pairs",
                default_value: "",
            },
            ArgSchema {
                name: "Strict",
                description: "Boolean",
                default_value: "true",
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
        let model = args[0].as_str().unwrap_or("3-rotor");
        let mut rotors = Vec::new();

        let _start_idx = if model == "3-rotor" { 3 } else { 0 };
        for i in 0..4 {
            if i == 0 && model == "3-rotor" {
                continue;
            }
            let rotor_spec = args[i * 3 + 1].as_str().unwrap_or("");
            let (wiring, steps) = if rotor_spec.contains('<') {
                let parts: Vec<&str> = rotor_spec.split('<').collect();
                (parts[0], parts[1])
            } else {
                (rotor_spec, "")
            };
            let ring = args[i * 3 + 2]
                .as_str()
                .unwrap_or("A")
                .chars()
                .next()
                .unwrap_or('A');
            let pos = args[i * 3 + 3]
                .as_str()
                .unwrap_or("A")
                .chars()
                .next()
                .unwrap_or('A');
            rotors
                .push(Rotor::new(wiring, steps, ring, pos).map_err(OperationError::InvalidInput)?);
        }
        rotors.reverse();

        let reflector =
            PairMap::new(args[13].as_str().unwrap_or("")).map_err(OperationError::InvalidInput)?;
        let plugboard =
            PairMap::new(args[14].as_str().unwrap_or("")).map_err(OperationError::InvalidInput)?;
        let strict = args[15].as_str().unwrap_or("true") == "true";

        let mut result = Vec::new();
        for &b in &input {
            let c = b.to_ascii_uppercase();
            if c < b'A' || c > b'Z' {
                if !strict {
                    result.push(b);
                }
                continue;
            }
            let mut val = c - b'A';

            // Step
            let r0_pos = rotors[0].step();
            if rotors[0].steps.contains(&r0_pos)
                || rotors[1].steps.contains(&((rotors[1].pos + 1) % 26))
            {
                let r1_pos = rotors[1].step();
                if rotors[1].steps.contains(&r1_pos) {
                    rotors[2].step();
                }
            }

            val = plugboard.transform(val);
            for r in &rotors {
                val = r.transform(val);
            }
            val = reflector.transform(val);
            for r in rotors.iter().rev() {
                val = r.rev_transform(val);
            }
            val = plugboard.transform(val);

            result.push(val + b'A');
        }

        Ok(result)
    }
}
