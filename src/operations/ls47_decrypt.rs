/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LS47 Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// LS47 Decrypt operation
pub struct LS47Decrypt;

const ALPHABET: &str = "_abcdefghijklmnopqrstuvwxyz.0123456789,-+*/:?!'()";

impl Operation for LS47Decrypt {
    fn name(&self) -> &'static str {
        "LS47 Decrypt"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "This is a slight improvement of the ElsieFour cipher as described by Alan Kaminsky. We use 7x7 characters instead of original (barely fitting) 6x6, to be able to encrypt some structured information. We also describe a simple key-expansion algorithm, because remembering passwords is popular. Similar security considerations as with ElsieFour hold.<br>The LS47 alphabet consists of following characters: <code>_abcdefghijklmnopqrstuvwxyz.0123456789,-+*/:?!'()</code><br>An LS47 key is a permutation of the alphabet that is then represented in a 7x7 grid used for the encryption or decryption."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Password",
                description: "The password used to derive the LS47 key",
                default_value: "",
            },
            ArgSchema {
                name: "Padding",
                description:
                    "The number of padding characters to remove from the start of the output",
                default_value: "10",
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
        let password = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let padding_size = args.get(1).and_then(|v| v.as_usize()).unwrap_or(10);

        let input_str = String::from_utf8_lossy(&input);
        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let key = self.derive_key(password)?;
        let decrypted = self.decrypt(&key, &input_str)?;

        if decrypted.len() <= padding_size {
            return Ok(Vec::new());
        }

        Ok(decrypted[padding_size..].as_bytes().to_vec())
    }
}

impl LS47Decrypt {
    fn rotate_down(&self, key: &str, col: usize, n: usize) -> String {
        let chars: Vec<char> = key.chars().collect();
        let mut rows: Vec<Vec<char>> = chars.chunks(7).map(|c| c.to_vec()).collect();
        let shift = (7 - n % 7) % 7;

        let mut mid = Vec::with_capacity(7);
        for r in 0..7 {
            mid.push(rows[r][col]);
        }

        let mut new_mid = vec![' '; 7];
        for i in 0..7 {
            new_mid[i] = mid[(i + shift) % 7];
        }

        for r in 0..7 {
            rows[r][col] = new_mid[r];
        }

        rows.into_iter().flatten().collect()
    }

    fn rotate_right(&self, key: &str, row: usize, n: usize) -> String {
        let mut chars: Vec<char> = key.chars().collect();
        let shift = (7 - n % 7) % 7;
        let start = row * 7;
        let mid = chars[start..start + 7].to_vec();

        for i in 0..7 {
            chars[start + i] = mid[(i + shift) % 7];
        }
        chars.into_iter().collect()
    }

    fn find_pos(&self, key: &str, c: char) -> Result<(usize, usize), OperationError> {
        key.find(c)
            .map(|idx| (idx / 7, idx % 7))
            .ok_or_else(|| OperationError::ProcessingError(format!("Character {} not in key", c)))
    }

    fn find_ix(&self, c: char) -> Result<(usize, usize), OperationError> {
        ALPHABET
            .find(c)
            .map(|idx| (idx / 7, idx % 7))
            .ok_or_else(|| OperationError::ProcessingError(format!("Character {} not in LS47", c)))
    }

    fn add_pos(&self, a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
        ((a.0 + b.0) % 7, (a.1 + b.1) % 7)
    }

    fn sub_pos(&self, a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
        ((a.0 + 7 - b.0) % 7, (a.1 + 7 - b.1) % 7)
    }

    fn derive_key(&self, password: &str) -> Result<String, OperationError> {
        let mut k = ALPHABET.to_string();
        let mut i = 0;
        for c in password.chars() {
            let (row, col) = self.find_ix(c)?;
            k = self.rotate_right(&k, i, col);
            k = self.rotate_down(&k, i, row);
            i = (i + 1) % 7;
        }
        Ok(k)
    }

    fn decrypt(&self, key: &str, ciphertext: &str) -> Result<String, OperationError> {
        let mut k = key.to_string();
        let mut mp = (0, 0);
        let mut plaintext = String::new();

        for c in ciphertext.chars() {
            let cp = self.find_pos(&k, c)?;
            let mix_char = k
                .chars()
                .nth(mp.0 * 7 + mp.1)
                .ok_or_else(|| OperationError::ProcessingError("Invalid mix pos".to_string()))?;
            let mix = self.find_ix(mix_char)?;
            let pp = self.sub_pos(cp, mix);
            let p = k
                .chars()
                .nth(pp.0 * 7 + pp.1)
                .ok_or_else(|| OperationError::ProcessingError("Invalid p pos".to_string()))?;
            plaintext.push(p);

            k = self.rotate_right(&k, pp.0, 1);
            let cp_new = self.find_pos(&k, c)?;
            k = self.rotate_down(&k, cp_new.1, 1);

            let ix_c = self.find_ix(c)?;
            mp = self.add_pos(mp, ix_c);
        }
        Ok(plaintext)
    }
}
