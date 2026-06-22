/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the NT Hash operation.
 * -----------------------------------------------------------------------------
 */

use md4::{Digest, Md4};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// NT Hash operation
pub struct NTHash;

impl Operation for NTHash {
    fn name(&self) -> &'static str {
        "NT Hash"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "An NT Hash, sometimes referred to as an NTLM hash, is a method of storing passwords on Windows systems. It works by running MD4 on UTF-16LE encoded input. NTLM hashes are considered weak because they can be brute-forced very easily with modern hardware."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        // Convert to UTF-16LE
        let utf16_bytes: Vec<u8> = input_str
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect();

        let mut hasher = Md4::new();
        hasher.update(&utf16_bytes);
        let result = hasher.finalize();

        Ok(hex::encode(result).to_uppercase().into_bytes())
    }
}
