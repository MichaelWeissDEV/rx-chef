/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CRC32 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CRC32 operation
///
/// Computes CRC32 (Cyclic Redundancy Check) checksum of the input data.
/// Uses the crc crate with the CRC-32 algorithm (IEEE 802.3).
pub struct CRC32;

impl Operation for CRC32 {
    fn name(&self) -> &'static str {
        "CRC32"
    }

    fn module(&self) -> &'static str {
        "Checksums"
    }

    fn description(&self) -> &'static str {
        "CRC32 (Cyclic Redundancy Check) is a hash function that produces a 32-bit checksum. It is widely used for error detection in digital networks and storage devices."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Polynomial",
                description: "CRC polynomial (default: IEEE)",
                default_value: "IEEE",
            },
            ArgSchema {
                name: "Initial Value",
                description: "Initial CRC value (default: 0xFFFFFFFF)",
                default_value: "0xFFFFFFFF",
            },
            ArgSchema {
                name: "Reflect Input",
                description: "Reflect input bytes (true/false)",
                default_value: "true",
            },
            ArgSchema {
                name: "Reflect Output",
                description: "Reflect output CRC (true/false)",
                default_value: "true",
            },
            ArgSchema {
                name: "XOR Output",
                description: "XOR output with this value (default: 0xFFFFFFFF)",
                default_value: "0xFFFFFFFF",
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
        let polynomial = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("IEEE")
            .to_uppercase();
        let _initial_value = args
            .get(1)
            .and_then(|a| a.as_str())
            .and_then(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).ok())
            .unwrap_or(0xFFFFFFFF);
        let reflect_input = args
            .get(2)
            .and_then(|a| a.as_str())
            .map(|s| s.to_lowercase() == "true")
            .unwrap_or(true);
        let reflect_output = args
            .get(3)
            .and_then(|a| a.as_str())
            .map(|s| s.to_lowercase() == "true")
            .unwrap_or(true);
        let xor_output = args
            .get(4)
            .and_then(|a| a.as_str())
            .and_then(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).ok())
            .unwrap_or(0xFFFFFFFF);

        let crc = match polynomial.as_str() {
            "IEEE" | "CRC32" => {
                // CRC-32 IEEE 802.3 with standard settings
                let crc_calc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
                let mut result = crc_calc.checksum(&input);
                if !reflect_input {
                    result = reflect_u32(result, 32);
                }
                if !reflect_output {
                    result = reflect_u32(result, 32);
                }
                result ^ xor_output
            }
            "CKSUM" | "CRC32C" => {
                // CRC-32 castagnoli (CRC-32C) - polynomial 0x1EDC6F41
                let crc_calc = crc::Crc::<u32>::new(&crc::CRC_32_CKSUM);
                let mut result = crc_calc.checksum(&input);
                if !reflect_input {
                    result = reflect_u32(result, 32);
                }
                if !reflect_output {
                    result = reflect_u32(result, 32);
                }
                result ^ xor_output
            }
            "BZIP2" => {
                // CRC-32 with different initial value (like zlib)
                let crc_calc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
                let mut result = crc_calc.checksum(&input);
                result ^= 0xFFFFFFFF;
                if !reflect_input {
                    result = reflect_u32(result, 32);
                }
                if !reflect_output {
                    result = reflect_u32(result, 32);
                }
                result ^ xor_output
            }
            _ => {
                return Err(OperationError::InvalidInput(format!(
                    "Unsupported polynomial: {}. Supported: IEEE, CKSUM, BZIP2",
                    polynomial
                )));
            }
        };

        let output = format!("{:08X}", crc);
        Ok(output.into_bytes())
    }
}

fn reflect_u32(data: u32, num_bits: u32) -> u32 {
    let mut reflected = 0u32;
    for bit in 0..num_bits {
        if data & (1 << bit) != 0 {
            reflected |= 1 << (num_bits - 1 - bit);
        }
    }
    reflected
}
