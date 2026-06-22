/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate UUID operation.
 * -----------------------------------------------------------------------------
 */

use uuid::Uuid;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate UUID operation
///
/// Generates an RFC 4122 compliant Universally Unique Identifier (UUID) v4.
pub struct GenerateUUID;

impl Operation for GenerateUUID {
    fn name(&self) -> &'static str {
        "Generate UUID"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Generates an RFC 4122 compliant Universally Unique Identifier (UUID), also known as a Globally Unique Identifier (GUID). Only UUID v4 (random) is supported."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Version",
            description: "UUID version (only v4 is supported)",
            default_value: "v4",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let version = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("v4")
            .to_lowercase();

        if version != "v4" && version != "4" {
            return Err(OperationError::InvalidArgument {
                name: "Version".to_string(),
                reason: "Only UUID v4 (random) is supported".to_string(),
            });
        }

        let id = Uuid::new_v4();
        Ok(id.to_string().into_bytes())
    }
}
