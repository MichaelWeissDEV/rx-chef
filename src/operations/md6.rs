/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the MD6 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};
use std::ffi::c_int;

#[link(name = "md6")]
unsafe extern "C" {
    fn md6_default_r(d: c_int, keylen: c_int) -> c_int;
    fn md6_full_hash(
        d: c_int,
        data: *const u8,
        databitlen: u64,
        key: *const u8,
        keylen: c_int,
        levels: c_int,
        rounds: c_int,
        hashval: *mut u8,
    ) -> c_int;
}

/// MD6 operation
pub struct MD6;

impl Operation for MD6 {
    fn name(&self) -> &'static str {
        "MD6"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "The MD6 (Message-Digest 6) algorithm is a cryptographic hash function. It uses a Merkle tree-like structure to allow for immense parallel computation of hashes for very long inputs."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Size",
                description: "Hash size in bits (0-512)",
                default_value: "256",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Levels",
                description: "Number of levels in the Merkle tree",
                default_value: "64",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Key",
                description: "Optional key",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
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
        let size = args.first().and_then(|v| v.as_f64()).unwrap_or(256.0) as usize;
        let levels = args.get(1).and_then(|v| v.as_f64()).unwrap_or(64.0) as usize;
        let key = args.get(2).and_then(|v| v.as_str()).unwrap_or("");

        if !(1..=512).contains(&size) {
            return Err(OperationError::InvalidArgument {
                name: "Size".to_string(),
                reason: "Size must be between 1 and 512".to_string(),
            });
        }
        if levels > 255 {
            return Err(OperationError::InvalidArgument {
                name: "Levels".to_string(),
                reason: "Levels must be between 0 and 255".to_string(),
            });
        }
        if key.len() > 64 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Key must be at most 64 UTF-8 bytes".to_string(),
            });
        }

        let mut digest = vec![0_u8; size.div_ceil(8)];
        let key_bytes = key.as_bytes();
        // SAFETY: all pointers refer to live slices for the duration of the call;
        // lengths and the MD6 parameters have been validated above. The linked C
        // implementation is the bundled MD6 reference implementation.
        let status = unsafe {
            let rounds = md6_default_r(size as c_int, key_bytes.len() as c_int);
            md6_full_hash(
                size as c_int,
                input.as_ptr(),
                (input.len() as u64) * 8,
                key_bytes.as_ptr(),
                key_bytes.len() as c_int,
                levels as c_int,
                rounds,
                digest.as_mut_ptr(),
            )
        };
        if status != 0 {
            return Err(OperationError::ProcessingError(format!(
                "MD6 reference implementation failed with status {status}"
            )));
        }

        Ok(hex::encode(digest).into_bytes())
    }
}
