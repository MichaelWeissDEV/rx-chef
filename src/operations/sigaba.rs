/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SIGABA operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, Operation, OperationError};

pub const CR_ROTORS: &[(&str, &str)] = &[
    ("Example 1", "SRGWANHPJZFXVIDQCEUKBYOLMT"),
    ("Example 2", "THQEFSAZVKJYULBODCPXNIMWRG"),
    ("Example 3", "XDTUYLEVFNQZBPOGIRCSMHWKAJ"),
    ("Example 4", "LOHDMCWUPSTNGVXYFJREQIKBZA"),
    ("Example 5", "ERXWNZQIJYLVOFUMSGHTCKPBDA"),
    ("Example 6", "FQECYHJIOUMDZVPSLKRTGWXBAN"),
    ("Example 7", "TBYIUMKZDJSOPEWXVANHLCFQGR"),
    ("Example 8", "QZUPDTFNYIAOMLEBWJXCGHKRSV"),
    ("Example 9", "CZWNHEMPOVXLKRSIDGJFYBTQAU"),
    ("Example 10", "ENPXJVKYQBFZTICAGMOHWRLDUS"),
];

pub const I_ROTORS: &[(&str, &str)] = &[
    ("Example 1", "6201348957"),
    ("Example 2", "6147253089"),
    ("Example 3", "8239647510"),
    ("Example 4", "7194835260"),
    ("Example 5", "4873205916"),
];

const _LETTERS: &[&str] = &[
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];
const _NUMBERS: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

/// SIGABA operation
pub struct SigabaOp;

impl Operation for SigabaOp {
    fn name(&self) -> &'static str {
        "SIGABA"
    }

    fn module(&self) -> &'static str {
        "Bletchley"
    }

    fn description(&self) -> &'static str {
        "Encipher/decipher with the WW2 SIGABA machine. <br><br>SIGABA, otherwise known as ECM Mark II, was used by the United States for message encryption during WW2 up to the 1950s. It was developed in the 1930s by the US Army and Navy, and has up to this day never been broken. Consisting of 15 rotors: 5 cipher rotors and 10 rotors (5 control rotors and 5 index rotors) controlling the stepping of the cipher rotors, the rotor stepping for SIGABA is much more complex than other rotor machines of its time, such as Enigma. All example rotor wirings are random example sets.<br><br>To configure rotor wirings, for the cipher and control rotors enter a string of letters which map from A to Z, and for the index rotors enter a sequence of numbers which map from 0 to 9. Note that encryption is not the same as decryption, so first choose the desired mode. <br><br> Note: Whilst this has been tested against other software emulators, it has not been tested against hardware."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            // Cipher Rotors (15 args)
            ArgSchema {
                name: "1st cipher rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "1st cipher rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "1st cipher rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            ArgSchema {
                name: "2nd cipher rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "2nd cipher rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "2nd cipher rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            ArgSchema {
                name: "3rd cipher rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "3rd cipher rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "3rd cipher rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            ArgSchema {
                name: "4th cipher rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "4th cipher rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "4th cipher rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            ArgSchema {
                name: "5th cipher rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "5th cipher rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "5th cipher rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            // Control Rotors (15 args)
            ArgSchema {
                name: "1st control rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "1st control rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "1st control rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            ArgSchema {
                name: "2nd control rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "2nd control rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "2nd control rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            ArgSchema {
                name: "3rd control rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "3rd control rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "3rd control rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            ArgSchema {
                name: "4th control rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "4th control rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "4th control rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            ArgSchema {
                name: "5th control rotor",
                description: "Rotor wiring",
                default_value: "SRGWANHPJZFXVIDQCEUKBYOLMT",
            },
            ArgSchema {
                name: "5th control rotor reversed",
                description: "Reversed orientation",
                default_value: "false",
            },
            ArgSchema {
                name: "5th control rotor initial value",
                description: "Initial value",
                default_value: "A",
            },
            // Index Rotors (10 args)
            ArgSchema {
                name: "1st index rotor",
                description: "Rotor wiring",
                default_value: "6201348957",
            },
            ArgSchema {
                name: "1st index rotor initial value",
                description: "Initial value",
                default_value: "0",
            },
            ArgSchema {
                name: "2nd index rotor",
                description: "Rotor wiring",
                default_value: "6201348957",
            },
            ArgSchema {
                name: "2nd index rotor initial value",
                description: "Initial value",
                default_value: "0",
            },
            ArgSchema {
                name: "3rd index rotor",
                description: "Rotor wiring",
                default_value: "6201348957",
            },
            ArgSchema {
                name: "3rd index rotor initial value",
                description: "Initial value",
                default_value: "0",
            },
            ArgSchema {
                name: "4th index rotor",
                description: "Rotor wiring",
                default_value: "6201348957",
            },
            ArgSchema {
                name: "4th index rotor initial value",
                description: "Initial value",
                default_value: "0",
            },
            ArgSchema {
                name: "5th index rotor",
                description: "Rotor wiring",
                default_value: "6201348957",
            },
            ArgSchema {
                name: "5th index rotor initial value",
                description: "Initial value",
                default_value: "0",
            },
            // Mode
            ArgSchema {
                name: "SIGABA mode",
                description: "Encrypt or Decrypt",
                default_value: "Encrypt",
            },
        ];
        SCHEMA
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        let mut cipher_rotors = Vec::new();
        for i in 0..5 {
            let wiring = args[i * 3].as_str().unwrap_or("SRGWANHPJZFXVIDQCEUKBYOLMT");
            let rev = args[i * 3 + 1].as_bool().unwrap_or(false);
            let key = args[i * 3 + 2]
                .as_str()
                .unwrap_or("A")
                .chars()
                .next()
                .unwrap_or('A');
            cipher_rotors.push(CRRotor::new(wiring, key, rev));
        }

        let mut control_rotors = Vec::new();
        for i in 5..10 {
            let wiring = args[i * 3].as_str().unwrap_or("SRGWANHPJZFXVIDQCEUKBYOLMT");
            let rev = args[i * 3 + 1].as_bool().unwrap_or(false);
            let key = args[i * 3 + 2]
                .as_str()
                .unwrap_or("A")
                .chars()
                .next()
                .unwrap_or('A');
            control_rotors.push(CRRotor::new(wiring, key, rev));
        }

        let mut index_rotors = Vec::new();
        for i in 15..20 {
            let wiring = args[i * 2].as_str().unwrap_or("6201348957");
            let key = args[i * 2 + 1]
                .as_str()
                .unwrap_or("0")
                .chars()
                .next()
                .unwrap_or('0');
            index_rotors.push(IRotor::new(wiring, key));
        }

        let mode = args[40].as_str().unwrap_or("Encrypt");
        let mut machine = SigabaMachine::new(cipher_rotors, control_rotors, index_rotors);

        let result = if mode == "Encrypt" {
            machine.encrypt(&input_str)
        } else {
            machine.decrypt(&input_str)
        };

        Ok(result.into_bytes())
    }
}

// --- Supporting implementation ---

struct Rotor {
    state: usize,
    num_mapping: Vec<usize>,
    pos_mapping: Vec<usize>,
}

impl Rotor {
    fn new(wire_setting: Vec<usize>, key: usize, rev: bool) -> Self {
        let num_mapping = if !rev {
            wire_setting.clone()
        } else {
            let mut temp = vec![0; wire_setting.len()];
            for (i, &val) in wire_setting.iter().enumerate() {
                temp[val] = i;
            }
            temp
        };

        let length = num_mapping.len();
        let mut pos_mapping = Vec::with_capacity(length);
        if !rev {
            for i in 0..length {
                pos_mapping.push((key + i) % length);
            }
        } else {
            for i in 0..length {
                pos_mapping.push((key + length - i) % length);
            }
        }

        Self {
            state: key,
            num_mapping,
            pos_mapping,
        }
    }

    fn crypt_num(&self, input_pos: usize, direction: &str) -> usize {
        let inp_num = self.pos_mapping[input_pos];
        let out_num = if direction == "leftToRight" {
            self.num_mapping[inp_num]
        } else {
            self.num_mapping.iter().position(|&x| x == inp_num).unwrap()
        };
        self.pos_mapping.iter().position(|&x| x == out_num).unwrap()
    }

    fn step(&mut self) {
        if let Some(last) = self.pos_mapping.pop() {
            self.pos_mapping.insert(0, last);
            self.state = self.pos_mapping[0];
        }
    }
}

struct CRRotor {
    rotor: Rotor,
}

impl CRRotor {
    fn new(wire_setting: &str, key: char, rev: bool) -> Self {
        let wiring: Vec<usize> = wire_setting.chars().map(|c| (c as usize) - 65).collect();
        let key_num = (key as usize) - 65;
        Self {
            rotor: Rotor::new(wiring, key_num, rev),
        }
    }

    fn crypt(&self, input_pos: char, direction: &str) -> char {
        let pos = (input_pos as usize) - 65;
        let out_pos = self.rotor.crypt_num(pos, direction);
        ((out_pos as u8) + 65) as char
    }

    fn step(&mut self) {
        self.rotor.step();
    }
}

struct IRotor {
    rotor: Rotor,
}

impl IRotor {
    fn new(wire_setting: &str, key: char) -> Self {
        let wiring: Vec<usize> = wire_setting.chars().map(|c| (c as usize) - 48).collect();
        let key_num = (key as usize) - 48;
        Self {
            rotor: Rotor::new(wiring, key_num, false),
        }
    }

    fn crypt(&self, input_pos: usize) -> usize {
        self.rotor.crypt_num(input_pos, "leftToRight")
    }
}

struct CipherBank {
    rotors: Vec<CRRotor>,
}

impl CipherBank {
    fn new(rotors: Vec<CRRotor>) -> Self {
        Self { rotors }
    }

    fn encrypt(&self, mut input: char) -> char {
        for rotor in &self.rotors {
            input = rotor.crypt(input, "leftToRight");
        }
        input
    }

    fn decrypt(&self, mut input: char) -> char {
        for rotor in self.rotors.iter().rev() {
            input = rotor.crypt(input, "rightToLeft");
        }
        input
    }

    fn step(&mut self, index_inputs: &[usize]) {
        let logic = [vec![0, 9], vec![7, 8], vec![5, 6], vec![3, 4], vec![1, 2]];
        let mut move_mask = [false; 5];
        for (i, rule) in logic.iter().enumerate() {
            for &idx in index_inputs {
                if rule.contains(&idx) {
                    move_mask[i] = true;
                    break;
                }
            }
        }
        for (i, should_move) in move_mask.iter().enumerate() {
            if *should_move {
                self.rotors[i].step();
            }
        }
    }
}

struct ControlBank {
    rotors: Vec<CRRotor>,
}

impl ControlBank {
    fn new(mut rotors: Vec<CRRotor>) -> Self {
        rotors.reverse();
        Self { rotors }
    }

    fn crypt(&self, mut input: char) -> char {
        for rotor in &self.rotors {
            input = rotor.crypt(input, "rightToLeft");
        }
        input
    }

    fn get_outputs(&self) -> Vec<usize> {
        let outputs = [
            self.crypt('F'),
            self.crypt('G'),
            self.crypt('H'),
            self.crypt('I'),
        ];
        let logic = [
            (1, "B"),
            (2, "C"),
            (3, "DE"),
            (4, "FGH"),
            (5, "IJK"),
            (6, "LMNO"),
            (7, "PQRST"),
            (8, "UVWXYZ"),
            (9, "A"),
        ];
        let mut num_outputs = Vec::new();
        for (val, chars) in logic {
            for &out in &outputs {
                if chars.contains(out) {
                    num_outputs.push(val);
                    break;
                }
            }
        }
        num_outputs
    }

    fn step(&mut self) {
        // Rotors are reversed: [R-hand, 4th, middle, 2nd, L-hand]
        // JS code: MRotor = rotors[1], FRotor = rotors[2], SRotor = rotors[3]
        // In reversed list:
        // index 0: 5th (R-hand)
        // index 1: 4th (M-rotor in JS logic)
        // index 2: 3rd (F-rotor in JS logic)
        // index 3: 2nd (S-rotor in JS logic)
        // index 4: 1st (L-hand)

        // Let's re-verify the JS logic:
        // this.rotors = [...rotors].reverse();
        // const MRotor = this.rotors[1], FRotor = this.rotors[2], SRotor = this.rotors[3];
        // So rotors[1] is the 4th rotor, rotors[2] is 3rd, rotors[3] is 2nd.

        let fr_state = self.rotors[2].rotor.state;
        let mr_state = self.rotors[1].rotor.state;

        if fr_state == 14 {
            // 'O'
            if mr_state == 14 {
                self.rotors[3].step();
            }
            self.rotors[1].step();
        }
        self.rotors[2].step();
    }

    fn go_through_control(&mut self) -> Vec<usize> {
        let outputs = self.get_outputs();
        self.step();
        outputs
    }
}

struct IndexBank {
    rotors: Vec<IRotor>,
}

impl IndexBank {
    fn new(rotors: Vec<IRotor>) -> Self {
        Self { rotors }
    }

    fn crypt(&self, mut input: usize) -> usize {
        for rotor in &self.rotors {
            input = rotor.crypt(input);
        }
        input
    }

    fn go_through_index(&self, control_inputs: Vec<usize>) -> Vec<usize> {
        control_inputs
            .into_iter()
            .map(|inp| self.crypt(inp))
            .collect()
    }
}

struct SigabaMachine {
    cipher_bank: CipherBank,
    control_bank: ControlBank,
    index_bank: IndexBank,
}

impl SigabaMachine {
    fn new(
        cipher_rotors: Vec<CRRotor>,
        control_rotors: Vec<CRRotor>,
        index_rotors: Vec<IRotor>,
    ) -> Self {
        Self {
            cipher_bank: CipherBank::new(cipher_rotors),
            control_bank: ControlBank::new(control_rotors),
            index_bank: IndexBank::new(index_rotors),
        }
    }

    fn step(&mut self) {
        let control_out = self.control_bank.go_through_control();
        let index_out = self.index_bank.go_through_index(control_out);
        self.cipher_bank.step(&index_out);
    }

    fn encrypt_letter(&mut self, mut letter: char) -> char {
        letter = letter.to_ascii_uppercase();
        if letter == ' ' {
            letter = 'Z';
        } else if letter == 'Z' {
            letter = 'X';
        }
        if !letter.is_ascii_alphabetic() {
            return letter;
        }
        let encrypted = self.cipher_bank.encrypt(letter);
        self.step();
        encrypted
    }

    fn decrypt_letter(&mut self, mut letter: char) -> char {
        letter = letter.to_ascii_uppercase();
        if !letter.is_ascii_alphabetic() {
            return letter;
        }
        let mut decrypted = self.cipher_bank.decrypt(letter);
        if decrypted == 'Z' {
            decrypted = ' ';
        }
        self.step();
        decrypted
    }

    fn encrypt(&mut self, msg: &str) -> String {
        msg.chars().map(|c| self.encrypt_letter(c)).collect()
    }

    fn decrypt(&mut self, msg: &str) -> String {
        msg.chars().map(|c| self.decrypt_letter(c)).collect()
    }
}
