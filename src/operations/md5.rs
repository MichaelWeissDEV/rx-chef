/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the MD5 operation.
 * -----------------------------------------------------------------------------
 */

use md5::{Digest, Md5};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// MD5 operation
///
/// MD5 (Message-Digest 5) is a widely used hash function. It produces a
/// 128-bit hash value. While MD5 is not collision resistant and not
/// suitable for security applications like SSL/TLS or digital signatures,
/// it is still commonly used for checksums and data integrity verification.
pub struct MD5;

impl Operation for MD5 {
    fn name(&self) -> &'static str {
        "MD5"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "MD5 (Message-Digest 5) is a widely used hash function. It has been used in a variety of security applications and is also commonly used to check the integrity of files. However, MD5 is not collision resistant and it isn't suitable for applications like SSL/TLS certificates or digital signatures that rely on this property."
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
        let mut hasher = Md5::new();
        hasher.update(input);
        let digest = hasher.finalize();
        let output = format!("{:x}", digest);
        Ok(output.into_bytes())
    }
}
