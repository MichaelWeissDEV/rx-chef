/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Typex operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashSet;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Typex operation
pub struct Typex;

struct TypexRotor {
    map: [u8; 26],
    rev_map: [u8; 26],
    steps: HashSet<u8>,
    pos: u8,
}

impl TypexRotor {
    fn new(
        wiring: &str,
        steps_str: &str,
        reversed: bool,
        ring_setting: char,
        initial_position: char,
    ) -> Result<Self, String> {
        if wiring.len() != 26 {
            return Err("Wiring must be 26 chars".to_string());
        }

        let mut wiring_mod = [0u8; 26];
        if reversed {
            for i in 0..26 {
                let orig_output = (wiring.as_bytes()[i] - b'A') as i8;
                let input = (26 - orig_output).rem_euclid(26) as usize;
                let output = (26 - (i as i8)).rem_euclid(26) as u8;
                wiring_mod[input] = output;
            }
        } else {
            for (i, c) in wiring.chars().enumerate() {
                wiring_mod[i] = (c as u8).wrapping_sub(b'A');
            }
        }

        let mut map = [0u8; 26];
        let mut rev_map = [0u8; 26];
        for (i, &b) in wiring_mod.iter().enumerate() {
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
            steps.insert((s as i8 - rs as i8).rem_euclid(26) as u8);
        }

        let pos =
            ((initial_position as u8).wrapping_sub(b'A') as i8 - rs as i8).rem_euclid(26) as u8;
        Ok(Self {
            map,
            rev_map,
            steps,
            pos,
        })
    }

    fn step(&mut self) {
        self.pos = (self.pos + 1) % 26;
    }

    fn transform(&self, c: u8) -> u8 {
        let input = (c + self.pos) % 26;
        (self.map[input as usize] as i8 - self.pos as i8).rem_euclid(26) as u8
    }

    fn rev_transform(&self, c: u8) -> u8 {
        let input = (c + self.pos) % 26;
        (self.rev_map[input as usize] as i8 - self.pos as i8).rem_euclid(26) as u8
    }
}

struct TypexPairMap {
    map: [u8; 26],
}

impl TypexPairMap {
    fn new(pairs_str: &str) -> Result<Self, String> {
        let mut map: [u8; 26] = [0; 26];
        for i in 0..26 {
            map[i] = i as u8;
        }
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
        Ok(Self { map })
    }

    fn transform(&self, c: u8) -> u8 {
        self.map[c as usize]
    }
}

const KEYBOARD: &[(char, char)] = &[
    ('Q', '1'),
    ('W', '2'),
    ('E', '3'),
    ('R', '4'),
    ('T', '5'),
    ('Y', '6'),
    ('U', '7'),
    ('I', '8'),
    ('O', '9'),
    ('P', '0'),
    ('A', '-'),
    ('S', '/'),
    ('D', 'Z'),
    ('F', '%'),
    ('G', 'X'),
    ('H', '#'),
    ('K', '('),
    ('L', ')'),
    ('C', 'V'),
    ('B', '\''),
    ('N', ','),
    ('M', '.'),
];

impl Operation for Typex {
    fn name(&self) -> &'static str {
        "Typex"
    }

    fn module(&self) -> &'static str {
        "Bletchley"
    }

    fn description(&self) -> &'static str {
        "Encipher/decipher with the WW2 Typex machine."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "1st rotor",
                description: "Wiring<Steps",
                default_value: "MCYLPQUVRXGSAOWNBJEZDTFKHI<BFHNQUW",
            },
            ArgSchema {
                name: "1st rotor reversed",
                description: "Boolean",
                default_value: "false",
            },
            ArgSchema {
                name: "1st rotor ring setting",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "1st rotor initial value",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "2nd rotor",
                description: "Wiring<Steps",
                default_value: "KHWENRCBISXJQGOFMAPVYZDLTU<BFHNQUW",
            },
            ArgSchema {
                name: "2nd rotor reversed",
                description: "Boolean",
                default_value: "false",
            },
            ArgSchema {
                name: "2nd rotor ring setting",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "2nd rotor initial value",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "3rd rotor",
                description: "Wiring<Steps",
                default_value: "BYPDZMGIKQCUSATREHOJNLFWXV<BFHNQUW",
            },
            ArgSchema {
                name: "3rd rotor reversed",
                description: "Boolean",
                default_value: "false",
            },
            ArgSchema {
                name: "3rd rotor ring setting",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "3rd rotor initial value",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "4th rotor",
                description: "Wiring<Steps",
                default_value: "ZANJCGDLVHIXOBRPMSWQUKFYET<BFHNQUW",
            },
            ArgSchema {
                name: "4th rotor reversed",
                description: "Boolean",
                default_value: "false",
            },
            ArgSchema {
                name: "4th rotor ring setting",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "4th rotor initial value",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "5th rotor",
                description: "Wiring<Steps",
                default_value: "QXBGUTOVFCZPJIHSWERYNDAMLK<BFHNQUW",
            },
            ArgSchema {
                name: "5th rotor reversed",
                description: "Boolean",
                default_value: "false",
            },
            ArgSchema {
                name: "5th rotor ring setting",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "5th rotor initial value",
                description: "A-Z",
                default_value: "A",
            },
            ArgSchema {
                name: "Reflector",
                description: "Pairs",
                default_value: "AN BC FG IE KD LU MH OR TS VZ WQ XJ YP",
            },
            ArgSchema {
                name: "Plugboard",
                description: "A-Z (26 chars)",
                default_value: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            },
            ArgSchema {
                name: "Typex keyboard emulation",
                description: "None, Encrypt, Decrypt",
                default_value: "None",
            },
            ArgSchema {
                name: "Strict output",
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
        let mut rotors = Vec::new();
        for i in 0..5 {
            let rotor_spec = args[i * 4].as_str().unwrap_or("");
            let (wiring, steps) = if rotor_spec.contains('<') {
                let parts: Vec<&str> = rotor_spec.split('<').collect();
                (parts[0], parts[1])
            } else {
                (rotor_spec, "")
            };
            let reversed = args[i * 4 + 1].as_bool().unwrap_or(false);
            let ring = args[i * 4 + 2]
                .as_str()
                .unwrap_or("A")
                .chars()
                .next()
                .unwrap_or('A');
            let pos = args[i * 4 + 3]
                .as_str()
                .unwrap_or("A")
                .chars()
                .next()
                .unwrap_or('A');
            rotors.push(
                TypexRotor::new(wiring, steps, reversed, ring, pos)
                    .map_err(OperationError::InvalidInput)?,
            );
        }
        // CyberChef: rotors.reverse()
        rotors.reverse();

        let reflector = TypexPairMap::new(args[20].as_str().unwrap_or(""))
            .map_err(OperationError::InvalidInput)?;

        // Plugboard in Typex is a rotor-like map with alphabet mirroring
        let pb_wiring_raw = args[21].as_str().unwrap_or("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        if pb_wiring_raw.len() != 26 {
            return Err(OperationError::InvalidArgument {
                name: "Plugboard".to_string(),
                reason: "Must be 26 chars".to_string(),
            });
        }
        let alphabet_mirrored = "AZYXWVUTSRQPONMLKJIHGFEDCB";
        let mut pb_wiring = String::new();
        for c in pb_wiring_raw.chars() {
            let idx = (c as u8).wrapping_sub(b'A') as usize;
            if idx < 26 {
                pb_wiring.push(alphabet_mirrored.chars().nth(idx).unwrap());
            } else {
                pb_wiring.push(c);
            }
        }
        let plugboard = TypexRotor::new(&pb_wiring, "", false, 'A', 'A')
            .map_err(OperationError::InvalidInput)?;

        let keyboard_mode = args[22].as_str().unwrap_or("None");
        let strict = args[23].as_bool().unwrap_or(true);

        let mut input_str = String::from_utf8_lossy(&input).to_string();

        if strict {
            if keyboard_mode == "Encrypt" {
                input_str = input_str
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric() || " /%()',.-".contains(*c))
                    .collect();
            } else {
                input_str = input_str
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .collect();
            }
        }

        if keyboard_mode == "Encrypt" {
            let mut mod_input = String::new();
            let mut symbol_mode = false;
            for x in input_str.chars() {
                if x == ' ' {
                    mod_input.push('X');
                } else if symbol_mode {
                    if let Some(&(orig, _)) = KEYBOARD.iter().find(|&&(_, sym)| sym == x) {
                        mod_input.push(orig);
                    } else {
                        symbol_mode = false;
                        mod_input.push('V');
                        mod_input.push(x);
                    }
                } else {
                    if let Some(&(orig, _)) = KEYBOARD.iter().find(|&&(_, sym)| sym == x) {
                        symbol_mode = true;
                        mod_input.push('Z');
                        mod_input.push(orig);
                    } else {
                        mod_input.push(x);
                    }
                }
            }
            input_str = mod_input;
        }

        let mut output = String::new();
        for c in input_str.chars() {
            let mut val = (c.to_ascii_uppercase() as u8).wrapping_sub(b'A');
            if val >= 26 {
                output.push(c);
                continue;
            }

            // Step
            // In Typex, only the right-most three rotors step (rotors[0], [1], [2] in our reversed list)
            // But rotors[3] and [4] are static.
            // CyberChef logic:
            // const r0 = this.rotors[2];
            // const r1 = this.rotors[3];
            // Wait, CyberChef rotors are 0..4. r0 is 2, r1 is 3.
            // My rotors list is already reversed. So my rotors[0] is CyberChef's rotors[4] (static), etc.?
            // Let's re-read TypexMachine.step():
            // const r0 = this.rotors[2]; // 3rd rotor
            // const r1 = this.rotors[3]; // 4th rotor
            // r0.step();
            // Wait, CyberChef rotors: [rotor1, rotor2, rotor3, rotor4, rotor5]
            // rotors.reverse() -> [rotor5, rotor4, rotor3, rotor2, rotor1]
            // So r0 = this.rotors[2] is the 3rd rotor (middle).
            // r1 = this.rotors[3] is the 4th rotor.
            // In Typex, rotors 1 and 2 (the left-most ones) are static?
            // "Typex was originally built... using five rotors... first two are static."
            // So rotor 1 and 2 are static. Rotor 3, 4, 5 step.
            // CyberChef's `this.rotors` is [rotor5, rotor4, rotor3, rotor2, rotor1] AFTER reverse.
            // So this.rotors[0] is rotor 5, this.rotors[1] is rotor 4, this.rotors[2] is rotor 3.
            // this.rotors[3] is rotor 2, this.rotors[4] is rotor 1.
            // Wait, the step() function uses this.rotors[2] and this.rotors[3].
            // That means rotor 3 and rotor 2? That doesn't match "first two are static".
            // Let's look again:
            /*
            step() {
                const r0 = this.rotors[2];
                const r1 = this.rotors[3];
                r0.step();
                if (r0.steps.has(r0.pos) || r1.steps.has(Utils.mod(r1.pos + 1, 26))) {
                    r1.step();
                    if (r1.steps.has(r1.pos)) {
                        const r2 = this.rotors[4];
                        r2.step();
                    }
                }
            }
            */
            // If rotors are [R5, R4, R3, R2, R1], then r0=R3, r1=R2, r2=R1.
            // This means R3, R2, R1 step. R5 and R4 are static.
            // This matches "first two are static" if you count from the left (R5, R4).

            let r0 = &mut rotors[2];
            r0.step();
            let r0_pos = r0.pos;
            let r0_stepped = r0.steps.contains(&r0_pos);

            let mut r1_stepped = false;
            let r1_pos_plus_1 = (rotors[3].pos + 1) % 26;
            if r0_stepped || rotors[3].steps.contains(&r1_pos_plus_1) {
                rotors[3].step();
                r1_stepped = true;
            }

            if r1_stepped && rotors[3].steps.contains(&rotors[3].pos) {
                rotors[4].step();
            }

            // Crypt
            val = plugboard.transform(val);
            for r in &rotors {
                val = r.transform(val);
            }
            val = reflector.transform(val);
            for r in rotors.iter().rev() {
                val = r.rev_transform(val);
            }
            val = plugboard.rev_transform(val);

            output.push((val + b'A') as char);
        }

        if keyboard_mode == "Decrypt" {
            let mut mod_output = String::new();
            let mut symbol_mode = false;
            for x in output.chars() {
                if x == 'X' {
                    mod_output.push(' ');
                } else if x == 'V' {
                    symbol_mode = false;
                } else if x == 'Z' {
                    symbol_mode = true;
                } else if symbol_mode {
                    if let Some(&(_, sym)) = KEYBOARD.iter().find(|&&(orig, _)| orig == x) {
                        mod_output.push(sym);
                    } else {
                        mod_output.push(x);
                    }
                } else {
                    mod_output.push(x);
                }
            }
            output = mod_output;
        }

        if strict && keyboard_mode != "Decrypt" {
            // Group into 5 characters
            let mut grouped = String::new();
            for (i, c) in output.chars().enumerate() {
                if i > 0 && i % 5 == 0 {
                    grouped.push(' ');
                }
                grouped.push(c);
            }
            output = grouped;
        }

        Ok(output.into_bytes())
    }
}
