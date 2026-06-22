/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Disassemble x86 operation.
 * -----------------------------------------------------------------------------
 */

use capstone::prelude::*;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Disassemble x86 operation
pub struct DisassembleX86;

impl Operation for DisassembleX86 {
    fn name(&self) -> &'static str {
        "Disassemble x86"
    }

    fn module(&self) -> &'static str {
        "Shellcode"
    }

    fn description(&self) -> &'static str {
        "Disassembly is the process of translating machine language into assembly language.<br><br>This operation supports 64-bit, 32-bit and 16-bit code written for Intel or AMD x86 processors. It is particularly useful for reverse engineering shellcode.<br><br>Input should be in hexadecimal."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Bit mode",
                description: "The bit mode (64, 32, 16)",
                default_value: "64",
            },
            ArgSchema {
                name: "Compatibility",
                description: "The compatibility mode (not all supported by Capstone)",
                default_value: "Full x86 architecture",
            },
            ArgSchema {
                name: "Code Segment (CS)",
                description: "The code segment (not used by Capstone)",
                default_value: "0",
            },
            ArgSchema {
                name: "Offset (IP)",
                description: "The instruction pointer offset",
                default_value: "0",
            },
            ArgSchema {
                name: "Show instruction hex",
                description: "Whether to show instruction hex",
                default_value: "true",
            },
            ArgSchema {
                name: "Show instruction position",
                description: "Whether to show instruction position",
                default_value: "true",
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
        let bit_mode = args.first().and_then(|v| v.as_str()).unwrap_or("64");
        let offset = args.get(3).and_then(|v| v.as_usize()).unwrap_or(0);
        let show_hex = args.get(4).and_then(|v| v.as_bool()).unwrap_or(true);
        let show_pos = args.get(5).and_then(|v| v.as_bool()).unwrap_or(true);

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;
        let clean_input = input_str.replace(|c: char| c.is_whitespace(), "");
        let code = hex::decode(clean_input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?;

        let mode = match bit_mode {
            "64" => arch::x86::ArchMode::Mode64,
            "32" => arch::x86::ArchMode::Mode32,
            "16" => arch::x86::ArchMode::Mode16,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Bit mode".to_string(),
                    reason: format!("Unsupported bit mode: {}", bit_mode),
                })
            }
        };

        let cs = Capstone::new()
            .x86()
            .mode(mode)
            .syntax(arch::x86::ArchSyntax::Intel)
            .build()
            .map_err(|e| {
                OperationError::ProcessingError(format!("Failed to initialize Capstone: {}", e))
            })?;

        let insns = cs
            .disasm_all(&code, offset as u64)
            .map_err(|e| OperationError::ProcessingError(format!("Disassembly failed: {}", e)))?;

        let mut output = String::new();
        for i in insns.as_ref() {
            if show_pos {
                output.push_str(&format!("{:08x}  ", i.address()));
            }
            if show_hex {
                let bytes = i.bytes();
                let hex_bytes = hex::encode(bytes);
                output.push_str(&format!("{:<20} ", hex_bytes));
            }
            output.push_str(&format!(
                "{} {}\n",
                i.mnemonic().unwrap_or(""),
                i.op_str().unwrap_or("")
            ));
        }

        Ok(output.into_bytes())
    }
}
