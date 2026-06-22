/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SHA2 operation.
 * -----------------------------------------------------------------------------
 */

use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SHA2 operation
///
/// The SHA-2 (Secure Hash Algorithm 2) hash functions were designed by the NSA.
/// SHA-2 includes significant changes from its predecessor, SHA-1. The SHA-2
/// family consists of hash functions with digests (hash values) that are
/// 224, 256, 384 or 512 bits: SHA224, SHA256, SHA384, SHA512.
pub struct SHA2;

impl Operation for SHA2 {
    fn name(&self) -> &'static str {
        "SHA2"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "The SHA-2 (Secure Hash Algorithm 2) hash functions were designed by the NSA. SHA-2 includes significant changes from its predecessor, SHA-1. The SHA-2 family consists of hash functions with digests (hash values) that are 224, 256, 384 or 512 bits: SHA224, SHA256, SHA384, SHA512. The message digest algorithm for SHA256 variants consists, by default, of 64 rounds, and for SHA512 variants, it is, by default, 160."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Size",
                description: "Output size (224, 256, 384, 512, 512/256, 512/224)",
                default_value: "256",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of rounds for 256/224 (minimum 16)",
                default_value: "64",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of rounds for 512/384/224/256 (minimum 32)",
                default_value: "160",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let size = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("256")
            .to_string();
        let rounds_256 = args.get(1).and_then(|a| a.as_usize()).unwrap_or(64);
        let rounds_512 = args.get(2).and_then(|a| a.as_usize()).unwrap_or(160);

        // Validate rounds
        if size == "256" || size == "224" {
            if rounds_256 < 16 {
                return Err(OperationError::InvalidArgument {
                    name: "Rounds".to_string(),
                    reason: "Rounds must be at least 16 for SHA-256 variants".to_string(),
                });
            }
        } else {
            if rounds_512 < 32 {
                return Err(OperationError::InvalidArgument {
                    name: "Rounds".to_string(),
                    reason: "Rounds must be at least 32 for SHA-512 variants".to_string(),
                });
            }
        }

        let output = match size.as_str() {
            "224" => {
                let mut hasher = Sha224::new();
                hasher.update(input);
                let result = hasher.finalize();
                format!("{:x}", result)
            }
            "256" => {
                let mut hasher = Sha256::new();
                hasher.update(input);
                let result = hasher.finalize();
                format!("{:x}", result)
            }
            "384" => {
                let mut hasher = Sha384::new();
                hasher.update(input);
                let result = hasher.finalize();
                format!("{:x}", result)
            }
            "512" => {
                let mut hasher = Sha512::new();
                hasher.update(input);
                let result = hasher.finalize();
                format!("{:x}", result)
            }
            "512/256" => {
                let mut hasher = Sha512_256::new();
                hasher.update(input);
                let result = hasher.finalize();
                format!("{:x}", result)
            }
            "512/224" => {
                let mut hasher = Sha512_224::new();
                hasher.update(input);
                let result = hasher.finalize();
                format!("{:x}", result)
            }
            _ => {
                return Err(OperationError::InvalidInput(format!(
                    "Invalid size: {}. Must be 224, 256, 384, 512, 512/256, or 512/224",
                    size
                )))
            }
        };

        Ok(output.into_bytes())
    }
}
