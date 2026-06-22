/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LS47 Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use rand::Rng;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

const LETTERS: &str = "_abcdefghijklmnopqrstuvwxyz.0123456789,-+*/:?!'()";

pub struct LS47Encrypt;

impl LS47Encrypt {
    fn find_ix(letter: char) -> Result<(usize, usize), OperationError> {
        LETTERS.find(letter).map(|i| (i / 7, i % 7)).ok_or_else(|| {
            OperationError::ProcessingError(format!("Letter {} is not included in LS47", letter))
        })
    }

    fn rotate_down(key: &str, col: usize, n: usize) -> String {
        let mut rows: Vec<Vec<char>> = key
            .chars()
            .collect::<Vec<_>>()
            .chunks(7)
            .map(|c| c.to_vec())
            .collect();
        let mut column: Vec<char> = rows.iter().map(|r| r[col]).collect();

        let n = (7 - n % 7) % 7;
        column.rotate_left(n);

        for (i, r) in rows.iter_mut().enumerate() {
            r[col] = column[i];
        }

        rows.into_iter().flatten().collect()
    }

    fn rotate_right(key: &str, row: usize, n: usize) -> String {
        let mut rows: Vec<Vec<char>> = key
            .chars()
            .collect::<Vec<_>>()
            .chunks(7)
            .map(|c| c.to_vec())
            .collect();
        let n = (7 - n % 7) % 7;
        rows[row].rotate_left(n);
        rows.into_iter().flatten().collect()
    }

    fn derive_key(password: &str) -> Result<String, OperationError> {
        let mut i = 0;
        let mut k = LETTERS.to_string();
        for c in password.chars() {
            let (row, col) = Self::find_ix(c)?;
            k = Self::rotate_down(&Self::rotate_right(&k, i, col), i, row);
            i = (i + 1) % 7;
        }
        Ok(k)
    }

    fn find_pos(key: &str, letter: char) -> Result<(usize, usize), OperationError> {
        key.find(letter).map(|i| (i / 7, i % 7)).ok_or_else(|| {
            OperationError::ProcessingError(format!("Letter {} is not in the key", letter))
        })
    }

    fn find_at_pos(key: &str, coord: (usize, usize)) -> char {
        key.chars().nth(coord.1 + (coord.0 * 7)).unwrap()
    }

    fn add_pos(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
        ((a.0 + b.0) % 7, (a.1 + b.1) % 7)
    }

    fn encrypt(mut key: String, plaintext: &str) -> Result<String, OperationError> {
        let mut mp = (0, 0);
        let mut ciphertext = String::new();
        for p in plaintext.chars() {
            let pp = Self::find_pos(&key, p)?;
            let mix = Self::find_ix(Self::find_at_pos(&key, mp))?;
            let mut cp = Self::add_pos(pp, mix);
            let c = Self::find_at_pos(&key, cp);
            ciphertext.push(c);

            key = Self::rotate_right(&key, pp.0, 1);
            cp = Self::find_pos(&key, c)?;
            key = Self::rotate_down(&key, cp.1, 1);

            mp = Self::add_pos(mp, Self::find_ix(c)?);
        }
        Ok(ciphertext)
    }
}

impl Operation for LS47Encrypt {
    fn name(&self) -> &'static str {
        "LS47 Encrypt"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "This is a slight improvement of the ElsieFour cipher as described by Alan Kaminsky. We use 7x7 characters instead of original (barely fitting) 6x6, to be able to encrypt some structured information. We also describe a simple key-expansion algorithm, because remembering passwords is popular. Similar security considerations as with ElsieFour hold.<br>The LS47 alphabet consists of following characters: <code>_abcdefghijklmnopqrstuvwxyz.0123456789,-+*/:?!'()</code><br>A LS47 key is a permutation of the alphabet that is then represented in a 7x7 grid used for the encryption or decryption."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Password",
                description: "Password used to derive the key",
                default_value: "",
            },
            ArgSchema {
                name: "Padding",
                description: "Amount of random padding to add",
                default_value: "10",
            },
            ArgSchema {
                name: "Signature",
                description: "Signature to append to the end of the plaintext",
                default_value: "",
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
        let signature = args.get(2).and_then(|v| v.as_str()).unwrap_or("");

        let input_str =
            String::from_utf8(input).map_err(|e| OperationError::InvalidInput(e.to_string()))?;
        let key = Self::derive_key(password)?;

        let mut rng = rand::thread_rng();
        let mut padding = String::new();
        for _ in 0..padding_size {
            let idx = rng.gen_range(0..LETTERS.len());
            padding.push(LETTERS.chars().nth(idx).unwrap());
        }

        let plaintext = format!("{}{}{}{}", padding, input_str, "---", signature);
        let ciphertext = Self::encrypt(key, &plaintext)?;

        Ok(ciphertext.into_bytes())
    }
}
