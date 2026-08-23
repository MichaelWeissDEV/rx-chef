/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Pseudo-Random Number Generator operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigUint;
use rand::RngCore;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Pseudo-Random Number Generator operation
pub struct PseudoRandomNumberGenerator;

impl Operation for PseudoRandomNumberGenerator {
    fn name(&self) -> &'static str {
        "Pseudo-Random Number Generator"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "A cryptographically-secure pseudo-random number generator (PRNG). This operation uses a cryptographically secure RNG."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Number of bytes",
                description: "How many bytes to generate",
                default_value: "32",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Output as",
                description: "Output format (Hex, Integer, Byte array, Raw)",
                default_value: "Hex",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["Hex", "Integer", "Byte array", "Raw"],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn input_requirement(&self) -> crate::operation::InputRequirement {
        crate::operation::InputRequirement::Ignored
    }

    fn output_type(&self) -> DataType {
        // Bytes, not String: the "Raw" output mode emits the cipher's/digest's
        // raw octets, which are not valid UTF-8 in general. Declaring String
        // made the runtime's output contract reject that documented mode for
        // any binary result. `AES Encrypt` and `Blowfish Encrypt` already
        // declare Bytes for the same reason.
        DataType::Bytes
    }

    /// Generates fresh random bytes on every run.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Random]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let num_bytes = args.first().and_then(|a| a.as_usize()).unwrap_or(32);
        let output_as = args.get(1).and_then(|a| a.as_str()).unwrap_or("Hex");

        let mut bytes = vec![0u8; num_bytes];
        rand::thread_rng().fill_bytes(&mut bytes);

        format_bytes(bytes, output_as)
    }
}

fn format_bytes(bytes: Vec<u8>, output_as: &str) -> Result<Vec<u8>, OperationError> {
    match output_as {
        "Hex" => Ok(hex::encode(bytes).into_bytes()),
        "Integer" => Ok(BigUint::from_bytes_le(&bytes).to_string().into_bytes()),
        "Byte array" => serde_json::to_vec(&bytes)
            .map_err(|e| OperationError::ProcessingError(e.to_string())),
        "Raw" => Ok(bytes),
        _ => Err(OperationError::InvalidArgument {
            name: "Output as".into(),
            reason: format!("unsupported random-number output format {output_as:?}"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::format_bytes;

    #[test]
    fn fixed_octets_have_exact_documented_encodings() {
        let octets = vec![0x00, 0x01, 0xff];
        assert_eq!(format_bytes(octets.clone(), "Hex").unwrap(), b"0001ff");
        assert_eq!(format_bytes(octets.clone(), "Integer").unwrap(), b"16711936");
        assert_eq!(format_bytes(octets.clone(), "Byte array").unwrap(), b"[0,1,255]");
        assert_eq!(format_bytes(octets.clone(), "Raw").unwrap(), octets);
    }
}
