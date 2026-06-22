/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Argon2 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Argon2 operation
///
/// Argon2 is a key derivation function that was selected as the winner of the
/// Password Hashing Competition in July 2015. It was designed by Alex Biryukov,
/// Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.
pub struct Argon2;

impl Operation for Argon2 {
    fn name(&self) -> &'static str {
        "Argon2"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Salt",
                description: "Salt value",
                default_value: "somesalt",
            },
            ArgSchema {
                name: "Iterations",
                description: "Number of iterations",
                default_value: "3",
            },
            ArgSchema {
                name: "Memory (KiB)",
                description: "Memory usage in KiB",
                default_value: "4096",
            },
            ArgSchema {
                name: "Parallelism",
                description: "Degree of parallelism",
                default_value: "1",
            },
            ArgSchema {
                name: "Hash length (bytes)",
                description: "Length of the hash in bytes",
                default_value: "32",
            },
            ArgSchema {
                name: "Type",
                description: "Argon2 type (Argon2i, Argon2d, Argon2id)",
                default_value: "Argon2i",
            },
            ArgSchema {
                name: "Output format",
                description: "Output format (Encoded hash, Hex hash, Raw hash)",
                default_value: "Encoded hash",
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
        let salt = args.first().and_then(|a| a.as_str()).unwrap_or("somesalt");
        let iterations = args.get(1).and_then(|a| a.as_i64()).unwrap_or(3) as u32;
        let memory = args.get(2).and_then(|a| a.as_i64()).unwrap_or(4096) as u32;
        let parallelism = args.get(3).and_then(|a| a.as_i64()).unwrap_or(1) as u32;
        let hash_len = args.get(4).and_then(|a| a.as_i64()).unwrap_or(32) as usize;
        let argon_type = args.get(5).and_then(|a| a.as_str()).unwrap_or("Argon2i");
        let output_format = args
            .get(6)
            .and_then(|a| a.as_str())
            .unwrap_or("Encoded hash");

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        // Map Argon2 type
        let alg = match argon_type {
            "Argon2d" => argon2::Algorithm::Argon2d,
            "Argon2id" => argon2::Algorithm::Argon2id,
            _ => argon2::Algorithm::Argon2i,
        };

        let version = argon2::Version::V0x13; // v1.3 (latest)
        let params = argon2::Params::new(memory, iterations, parallelism, Some(hash_len))
            .map_err(|e| OperationError::InvalidInput(format!("Invalid params: {}", e)))?;

        let hasher = argon2::Argon2::new(alg, version, params);

        // Hash the password
        let salt_bytes = salt.as_bytes();
        let mut hash = [0u8; 64]; // Max hash length
        let hash_len_actual = hash_len.min(hash.len());

        hasher
            .hash_password_into(
                input_str.as_bytes(),
                salt_bytes,
                &mut hash[..hash_len_actual],
            )
            .map_err(|e| OperationError::InvalidInput(format!("Hashing error: {}", e)))?;

        let output = match output_format {
            "Hex hash" => hex::encode(&hash[..hash_len_actual]),
            "Raw hash" => {
                let mut result = String::new();
                for &b in hash.iter().take(hash_len_actual) {
                    if b.is_ascii_graphic() || b == b' ' {
                        result.push(b as char);
                    } else {
                        result.push('?');
                    }
                }
                result
            }
            "Encoded hash" | _ => {
                // Build PHC string manually
                // Format: $argon2<type>$v=<version>$m=<mem>,t=<iter>,p=<par>$<salt>$<hash>
                let salt_b64 = data_encoding::BASE64.encode(salt_bytes);
                let hash_b64 = data_encoding::BASE64.encode(&hash[..hash_len_actual]);

                let alg_str = match alg {
                    argon2::Algorithm::Argon2i => "i",
                    argon2::Algorithm::Argon2d => "d",
                    argon2::Algorithm::Argon2id => "id",
                };

                format!(
                    "$argon2{}$v={:02}$m={},t={},p={}${}${}",
                    alg_str, version as u8, memory, iterations, parallelism, salt_b64, hash_b64
                )
            }
        };

        Ok(output.into_bytes())
    }
}
