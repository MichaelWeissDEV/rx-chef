/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CipherSaber2 Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CipherSaber2 Decrypt operation
pub struct CipherSaber2Decrypt;

impl Operation for CipherSaber2Decrypt {
    fn name(&self) -> &'static str {
        "CipherSaber2 Decrypt"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "CipherSaber is a simple symmetric encryption protocol based on the RC4 stream \
        cipher. It gives reasonably strong protection of message confidentiality, yet \
        it's designed to be simple enough that even novice programmers can memorize the \
        algorithm and implement it from scratch. The first 10 bytes of the input are \
        the initialisation vector (IV)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Decryption key",
                default_value: "",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of key schedule rounds (default 20)",
                default_value: "20",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.len() < 10 {
            return Err(OperationError::InvalidInput(
                "Input must be at least 10 bytes (IV prepended to ciphertext)".to_string(),
            ));
        }

        let key = self.parse_key(args.first())?;
        let rounds = args.get(1).and_then(|a| a.as_usize()).unwrap_or(20);

        let iv = &input[..10];
        let ciphertext = &input[10..];

        let plaintext = ciphersaber2_crypt(iv, &key, rounds, ciphertext);
        Ok(plaintext)
    }
}

impl CipherSaber2Decrypt {
    fn parse_key(&self, arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
        match arg {
            Some(ArgValue::Str(s)) => {
                if s.is_empty() {
                    Ok(vec![])
                } else if s.starts_with("0x") {
                    hex::decode(&s[2..]).map_err(|e| OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: e.to_string(),
                    })
                } else {
                    // Try to detect if it's hex, otherwise treat as UTF-8
                    // CyberChef's toggleString would be better, but we'll use a heuristic or just UTF-8 for now
                    // consistent with other operations in this project.
                    Ok(s.as_bytes().to_vec())
                }
            }
            Some(ArgValue::Bytes(b)) => Ok(b.clone()),
            _ => Ok(vec![]),
        }
    }
}

/// RC4-based CipherSaber2 encrypt/decrypt (symmetric).
pub(crate) fn ciphersaber2_crypt(iv: &[u8], key: &[u8], rounds: usize, data: &[u8]) -> Vec<u8> {
    let combined: Vec<u8> = key.iter().chain(iv.iter()).copied().collect();
    let combined_len = combined.len();

    let mut state: Vec<u8> = (0u8..=255).collect();
    let mut j: usize = 0;

    for _ in 0..rounds {
        for k in 0usize..256 {
            j = (j + state[k] as usize + combined[k % combined_len] as usize) % 256;
            state.swap(k, j);
        }
    }

    let mut i: usize = 0;
    j = 0;
    let mut output = Vec::with_capacity(data.len());

    for &byte in data {
        i = (i + 1) % 256;
        j = (j + state[i] as usize) % 256;
        state.swap(i, j);
        let n = (state[i] as usize + state[j] as usize) % 256;
        output.push(state[n] ^ byte);
    }

    output
}
