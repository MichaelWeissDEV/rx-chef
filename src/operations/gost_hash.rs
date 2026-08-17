/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Hash operation.
 * -----------------------------------------------------------------------------
 */

use gost94::{digest::Digest as Gost94Digest, Gost94CryptoPro, Gost94Test};
use streebog::{digest::Digest as StreebogDigest, Streebog256, Streebog512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// GOST Hash operation
pub struct GostHash;

impl Operation for GostHash {
    fn name(&self) -> &'static str {
        "GOST Hash"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "The GOST hash function, defined in the standards GOST R 34.11-94 and GOST 34.311-95 is a 256-bit cryptographic hash function. It was initially defined in the Russian national standard GOST R 34.11-94 Information Technology  Cryptographic Information Security  Hash Function. The equivalent standard used by other member-states of the CIS is GOST 34.311-95.\n\nThis function must not be confused with a different Streebog hash function, which is defined in the new revision of the standard GOST R 34.11-2012.\n\nThe GOST hash function is based on the GOST block cipher."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Algorithm",
                description: "The GOST hash algorithm version to use.",
                default_value: "GOST 28147 (1994)",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["GOST 28147 (1994)", "GOST R 34.11 (2012)"],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Digest length",
                description: "The length of the digest to produce (only for Streebog).",
                default_value: "256",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "sBox",
                description: "GOST94 parameter set: E-TEST/D-TEST (test) or CryptoPro/D-A",
                default_value: "E-TEST",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["E-TEST", "D-TEST", "CryptoPro", "D-A"],
                minimum: None,
                maximum: None,
                sensitive: false,
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
        let algorithm = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("GOST 28147 (1994)");
        let digest_length = args.get(1).and_then(|a| a.as_str()).unwrap_or("256");
        let sbox_name = args.get(2).and_then(|a| a.as_str()).unwrap_or("E-TEST");

        // An unrecognised algorithm must be rejected rather than silently
        // falling through to Streebog, which would return a digest from a
        // completely different hash function than the caller named.
        let result = match algorithm {
            "GOST 28147 (1994)" => match sbox_name.to_ascii_uppercase().as_str() {
                "E-TEST" | "D-TEST" | "TEST" => {
                    hex::encode(<Gost94Test as Gost94Digest>::digest(&input))
                }
                "CRYPTOPRO" | "D-A" => {
                    hex::encode(<Gost94CryptoPro as Gost94Digest>::digest(&input))
                }
                _ => {
                    return Err(OperationError::InvalidArgument {
                        name: "sBox".to_string(),
                        reason: format!("unsupported GOST94 parameter set '{sbox_name}'"),
                    })
                }
            },
            "GOST R 34.11 (2012)" => match digest_length {
                "256" => hex::encode(<Streebog256 as StreebogDigest>::digest(&input)),
                "512" => hex::encode(<Streebog512 as StreebogDigest>::digest(&input)),
                _ => {
                    return Err(OperationError::InvalidArgument {
                        name: "Digest length".to_string(),
                        reason: "Invalid digest length for Streebog. Must be 256 or 512."
                            .to_string(),
                    })
                }
            },
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Algorithm".to_string(),
                    reason: format!(
                        "unsupported GOST algorithm '{other}'; expected 'GOST 28147 (1994)' or 'GOST R 34.11 (2012)'"
                    ),
                })
            }
        };

        Ok(result.into_bytes())
    }
}
