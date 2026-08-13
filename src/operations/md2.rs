/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the MD2 operation.
 * -----------------------------------------------------------------------------
 */

use digest::Digest;
use md2::Md2;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// MD2 operation
///
/// The MD2 (Message-Digest 2) algorithm is a cryptographic hash function
/// developed by Ronald Rivest in 1989. The algorithm is optimized for 8-bit
/// computers. Although MD2 is no longer considered secure, it remains in use
/// in public key infrastructures as part of certificates generated with MD2
/// and RSA.
pub struct MD2;

impl Operation for MD2 {
    fn name(&self) -> &'static str {
        "MD2"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "MD2 (Message-Digest 2) is a cryptographic hash function developed by Ronald Rivest in 1989. The algorithm is optimized for 8-bit computers. Although MD2 is no longer considered secure, it remains in use in public key infrastructures as part of certificates generated with MD2 and RSA."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let _ = args;
        let mut hasher = Md2::new();
        hasher.update(&input);
        let digest = hasher.finalize();
        let output: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
        Ok(output.into_bytes())
    }
}
