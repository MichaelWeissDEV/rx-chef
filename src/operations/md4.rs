/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the MD4 operation.
 * -----------------------------------------------------------------------------
 */

use digest::Digest;
use md4::Md4;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// MD4 operation
///
/// The MD4 (Message-Digest 4) algorithm is a cryptographic hash function
/// developed by Ronald Rivest in 1990. The digest length is 128 bits.
/// The algorithm has influenced later designs, such as MD5, SHA-1, and
/// RIPEMD. The security of MD4 has been severely compromised.
pub struct MD4;

impl Operation for MD4 {
    fn name(&self) -> &'static str {
        "MD4"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "MD4 (Message-Digest 4) is a cryptographic hash function developed by Ronald Rivest in 1990. The digest length is 128 bits. The algorithm has influenced later designs, such as MD5, SHA-1, and RIPEMD algorithms. The security of MD4 has been severely compromised."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let mut hasher = Md4::new();
        hasher.update(input);
        let digest = hasher.finalize();
        let output = hex::encode(digest);
        Ok(output.into_bytes())
    }
}
