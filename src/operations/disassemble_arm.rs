/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Disassemble ARM operation.
 * -----------------------------------------------------------------------------
 */

#[cfg(feature = "disassembly")]
use capstone::prelude::*;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct DisassembleArm;

impl Operation for DisassembleArm {
    fn name(&self) -> &'static str {
        "Disassemble ARM"
    }

    fn module(&self) -> &'static str {
        "Shellcode"
    }

    fn description(&self) -> &'static str {
        "Disassembles ARM machine code into assembly language.<br><br>Supports ARM (32-bit), Thumb, and ARM64 (AArch64) architectures using the Capstone disassembly framework.<br><br>Input should be in hexadecimal."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Architecture",
                description: "The ARM architecture to use.",
                default_value: "ARM (32-bit)",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Mode",
                description: "The instruction set mode.",
                default_value: "ARM",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Endianness",
                description: "The byte order.",
                default_value: "Little Endian",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Starting address (hex)",
                description: "The address to start disassembling from.",
                default_value: "0",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Show instruction hex",
                description: "Whether to show the hex bytes of each instruction.",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Show instruction position",
                description: "Whether to show the address of each instruction.",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
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

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn is_broken(&self) -> bool {
        !cfg!(feature = "disassembly")
    }

    fn feature_requirements(&self) -> &'static [&'static str] {
        &["disassembly"]
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        #[cfg(not(feature = "disassembly"))]
        {
            let _ = (input, args);
            return Err(OperationError::ProcessingError(
                "Disassemble ARM requires --features disassembly".to_string(),
            ));
        }
        #[cfg(feature = "disassembly")]
        {
            let input_str = String::from_utf8_lossy(&input);
            let hex_input: String = input_str.chars().filter(|c| !c.is_whitespace()).collect();

            if hex_input.is_empty() {
                return Ok(Vec::new());
            }

            if hex_input.len() % 2 != 0 {
                return Err(OperationError::InvalidInput(
                    "Invalid hexadecimal input. Length must be even.".to_string(),
                ));
            }

            // Slice the bytes, not the `&str`. Non-ASCII input becomes U+FFFD
            // replacement characters, which are three bytes each: the length
            // stays even while `hex_input[i..i + 2]` lands inside a character
            // and panics. Eight 0xFF bytes were enough to reach it.
            let hex_bytes = hex_input.as_bytes();
            let invalid_hex = || {
                OperationError::InvalidInput(
                    "Invalid hexadecimal input. Please provide valid hex characters only."
                        .to_string(),
                )
            };
            let bytes = (0..hex_bytes.len())
                .step_by(2)
                .map(|i| {
                    let pair =
                        std::str::from_utf8(&hex_bytes[i..i + 2]).map_err(|_| invalid_hex())?;
                    u8::from_str_radix(pair, 16).map_err(|_| invalid_hex())
                })
                .collect::<Result<Vec<u8>, OperationError>>()?;

            let architecture = args
                .get(0)
                .and_then(|a| a.as_str())
                .unwrap_or("ARM (32-bit)");
            let mode_arg = args.get(1).and_then(|a| a.as_str()).unwrap_or("ARM");
            let endianness = args
                .get(2)
                .and_then(|a| a.as_str())
                .unwrap_or("Little Endian");
            let start_address = args.get(3).and_then(|a| a.as_usize()).unwrap_or(0) as u64;
            let show_hex = args.get(4).and_then(|a| a.as_bool()).unwrap_or(true);
            let show_position = args.get(5).and_then(|a| a.as_bool()).unwrap_or(true);

            let endian = if endianness == "Big Endian" {
                capstone::Endian::Big
            } else {
                capstone::Endian::Little
            };

            let cs = if architecture == "ARM64 (AArch64)" {
                Capstone::new()
                    .arm64()
                    .mode(capstone::arch::arm64::ArchMode::Arm)
                    .endian(endian)
                    .build()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to initialise Capstone disassembler: {}",
                            e
                        ))
                    })?
            } else {
                let builder = Capstone::new().arm().endian(endian);
                let arm_mode;
                let mut extra_modes = Vec::new();

                match mode_arg {
                    "ARM" => arm_mode = capstone::arch::arm::ArchMode::Arm,
                    "Thumb" => arm_mode = capstone::arch::arm::ArchMode::Thumb,
                    "Thumb + Cortex-M" => {
                        arm_mode = capstone::arch::arm::ArchMode::Thumb;
                        extra_modes.push(capstone::arch::arm::ArchExtraMode::MClass);
                    }
                    "ARMv8" => {
                        arm_mode = capstone::arch::arm::ArchMode::Arm;
                        extra_modes.push(capstone::arch::arm::ArchExtraMode::V8);
                    }
                    _ => arm_mode = capstone::arch::arm::ArchMode::Arm,
                }

                builder
                    .mode(arm_mode)
                    .extra_mode(extra_modes.into_iter())
                    .build()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to initialise Capstone disassembler: {}",
                            e
                        ))
                    })?
            };

            let instructions = cs.disasm_all(&bytes, start_address).map_err(|e| {
                OperationError::ProcessingError(format!("Disassembly failed: {}", e))
            })?;

            if instructions.is_empty() {
                return Err(OperationError::ProcessingError(format!("No valid {} instructions found in input. The bytes may be for a different architecture or mode.", architecture)));
            }

            let mut output = String::new();
            for insn in instructions.as_ref() {
                if show_position {
                    output.push_str(&format!("0x{:08x}  ", insn.address()));
                }
                if show_hex {
                    let bytes_hex = insn
                        .bytes()
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();
                    output.push_str(&format!("{:16}  ", bytes_hex));
                }

                if let Some(mnemonic) = insn.mnemonic() {
                    output.push_str(mnemonic);
                }

                if let Some(op_str) = insn.op_str() {
                    if !op_str.is_empty() {
                        output.push_str(" ");
                        output.push_str(op_str);
                    }
                }
                output.push('\n');
            }

            if output.ends_with('\n') {
                output.pop();
            }

            Ok(output.into_bytes())
        }
    }
}
