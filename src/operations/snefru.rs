/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SNEFRU operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SNEFRU operation
///
/// SNEFRU is a cryptographic hash function invented by Ralph Merkle in 1990
/// while working at Xerox PARC. The function supports 128-bit and 256-bit output.
/// It was named after the Egyptian Pharaoh Sneferu, continuing the tradition of
/// the Khufu and Khafre block ciphers.
///
/// The original design of SNEFRU was shown to be insecure by Eli Biham and Adi
/// Shamir who were able to use differential cryptanalysis to find hash collisions.
/// The design was then modified by increasing the number of iterations of the main
/// pass of the algorithm from two to eight.
pub struct SNEFRU;

impl Operation for SNEFRU {
    fn name(&self) -> &'static str {
        "SNEFRU"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "Computes the standardized 256-bit, 8-round SNEFRU hash. SNEFRU was designed by Ralph Merkle in 1990; the original shorter-round design is retained in the argument schema for recipe compatibility but rejected because it is cryptographically broken."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Size",
                description: "Output size in bits (supported: 256)",
                default_value: "256",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of rounds (supported: 8)",
                default_value: "8",
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
        let size = args.first().and_then(|a| a.as_usize()).unwrap_or(256);
        let rounds = args.get(1).and_then(|a| a.as_usize()).unwrap_or(8);

        if size != 256 {
            return Err(OperationError::InvalidArgument {
                name: "Size".to_string(),
                reason: "Only the standardized 256-bit SNEFRU variant is supported".to_string(),
            });
        }

        if rounds != 8 {
            return Err(OperationError::InvalidArgument {
                name: "Rounds".to_string(),
                reason: "Only the strengthened 8-round SNEFRU variant is supported".to_string(),
            });
        }

        let digest = allthehashes::SNEFRU256
            .hash(&input)
            .ok_or_else(|| OperationError::ProcessingError("SNEFRU hashing failed".to_string()))?;
        Ok(hex::encode(digest).into_bytes())
    }
}
