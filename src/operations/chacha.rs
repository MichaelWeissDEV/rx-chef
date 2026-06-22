/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ChaCha operation.
 * -----------------------------------------------------------------------------
 */

use byteorder::{LittleEndian, WriteBytesExt};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// ChaCha operation
pub struct ChaCha;

impl Operation for ChaCha {
    fn name(&self) -> &'static str {
        "ChaCha"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "ChaCha is a stream cipher designed by Daniel J. Bernstein. It is a variant of the Salsa stream cipher. Several parameterizations exist; 'ChaCha' may refer to the original construction, or to the variant as described in RFC-8439. ChaCha is often used with Poly1305, in the ChaCha20-Poly1305 AEAD construction.<br><br><b>Key:</b> ChaCha uses a key of 16 or 32 bytes (128 or 256 bits).<br><br><b>Nonce:</b> ChaCha uses a nonce of 8 or 12 bytes (64 or 96 bits).<br><br><b>Counter:</b> ChaCha uses a counter of 4 or 8 bytes (32 or 64 bits); together, the nonce and counter must add up to 16 bytes. The counter starts at zero at the start of the keystream, and is incremented at every 64 bytes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "The encryption key (16 or 32 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "Nonce",
                description: "The nonce (8 or 12 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "Counter",
                description: "Initial counter value",
                default_value: "0",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of rounds (20, 12, or 8)",
                default_value: "20",
            },
            ArgSchema {
                name: "Input",
                description: "Format of input data",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output",
                description: "Format of output data",
                default_value: "Raw",
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
        let key_str = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let nonce_str = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let counter_val = args.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0) as u32;
        let rounds = args
            .get(3)
            .and_then(|v| v.as_str())
            .unwrap_or("20")
            .parse::<u32>()
            .unwrap_or(20);
        let input_format = args.get(4).and_then(|v| v.as_str()).unwrap_or("Hex");
        let output_format = args.get(5).and_then(|v| v.as_str()).unwrap_or("Raw");

        let key = parse_arg_bytes(key_str)?;
        if key.len() != 16 && key.len() != 32 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes. ChaCha uses a key of 16 or 32 bytes.",
                    key.len()
                ),
            });
        }

        let nonce = parse_arg_bytes(nonce_str)?;
        if nonce.len() != 8 && nonce.len() != 12 {
            return Err(OperationError::InvalidArgument {
                name: "Nonce".to_string(),
                reason: format!(
                    "Invalid nonce length: {} bytes. ChaCha uses a nonce of 8 or 12 bytes.",
                    nonce.len()
                ),
            });
        }

        let input_data = if input_format == "Hex" {
            let input_str = String::from_utf8(input)
                .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;
            hex::decode(input_str.trim().replace(" ", "").replace("0x", ""))
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?
        } else {
            input
        };

        let mut output = Vec::with_capacity(input_data.len());
        let mut counter_int = counter_val;
        let counter_len = 16 - nonce.len();

        for chunk in input_data.chunks(64) {
            let mut counter_bytes = Vec::new();
            counter_bytes
                .write_u32::<LittleEndian>(counter_int)
                .unwrap();
            while counter_bytes.len() < counter_len {
                counter_bytes.push(0);
            }
            if counter_bytes.len() > counter_len {
                counter_bytes.truncate(counter_len);
            }

            let stream = chacha_block(&key, &nonce, &counter_bytes, rounds);
            for (i, &byte) in chunk.iter().enumerate() {
                output.push(byte ^ stream[i]);
            }
            counter_int = counter_int.wrapping_add(1);
        }

        if output_format == "Hex" {
            Ok(hex::encode(output).into_bytes())
        } else {
            Ok(output)
        }
    }
}

fn parse_arg_bytes(s: &str) -> Result<Vec<u8>, OperationError> {
    if s.is_empty() {
        Ok(vec![])
    } else if s.starts_with("0x") {
        hex::decode(&s[2..]).map_err(|e| OperationError::InvalidArgument {
            name: "Argument".to_string(),
            reason: e.to_string(),
        })
    } else {
        Ok(s.as_bytes().to_vec())
    }
}

fn chacha_block(key: &[u8], nonce: &[u8], counter: &[u8], rounds: u32) -> [u8; 64] {
    let tau = b"expand 16-byte k";
    let sigma = b"expand 32-byte k";

    let mut state = [0u32; 16];
    let constants = if key.len() == 16 { tau } else { sigma };

    for (i, chunk) in constants.chunks(4).enumerate() {
        state[i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    if key.len() == 16 {
        for (i, chunk) in key.chunks(4).enumerate() {
            state[i + 4] = u32::from_le_bytes(chunk.try_into().unwrap());
            state[i + 8] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
    } else {
        for (i, chunk) in key.chunks(4).enumerate() {
            state[i + 4] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
    }

    let mut idx = 12;
    for chunk in counter.chunks(4) {
        if idx < 16 {
            let mut b = [0u8; 4];
            b[..chunk.len()].copy_from_slice(chunk);
            state[idx] = u32::from_le_bytes(b);
            idx += 1;
        }
    }
    for chunk in nonce.chunks(4) {
        if idx < 16 {
            let mut b = [0u8; 4];
            b[..chunk.len()].copy_from_slice(chunk);
            state[idx] = u32::from_le_bytes(b);
            idx += 1;
        }
    }

    let initial_state = state;

    for _ in 0..(rounds / 2) {
        quarterround(&mut state, 0, 4, 8, 12);
        quarterround(&mut state, 1, 5, 9, 13);
        quarterround(&mut state, 2, 6, 10, 14);
        quarterround(&mut state, 3, 7, 11, 15);
        quarterround(&mut state, 0, 5, 10, 15);
        quarterround(&mut state, 1, 6, 11, 12);
        quarterround(&mut state, 2, 7, 8, 13);
        quarterround(&mut state, 3, 4, 9, 14);
    }

    let mut output = [0u8; 64];
    for i in 0..16 {
        let val = state[i].wrapping_add(initial_state[i]);
        let bytes = val.to_le_bytes();
        output[i * 4..i * 4 + 4].copy_from_slice(&bytes);
    }

    output
}

fn quarterround(x: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    x[a] = x[a].wrapping_add(x[b]);
    x[d] ^= x[a];
    x[d] = x[d].rotate_left(16);
    x[c] = x[c].wrapping_add(x[d]);
    x[b] ^= x[c];
    x[b] = x[b].rotate_left(12);
    x[a] = x[a].wrapping_add(x[b]);
    x[d] ^= x[a];
    x[d] = x[d].rotate_left(8);
    x[c] = x[c].wrapping_add(x[d]);
    x[b] ^= x[c];
    x[b] = x[b].rotate_left(7);
}
