/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LZNT1 Decompress operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// LZNT1 Decompress operation
pub struct LZNT1Decompress;

impl LZNT1Decompress {
    fn decompress(compressed: &[u8]) -> Result<Vec<u8>, OperationError> {
        let mut decompressed = Vec::new();
        let mut coffset = 0;

        while coffset + 2 <= compressed.len() {
            let header = u16::from_le_bytes([compressed[coffset], compressed[coffset + 1]]);
            coffset += 2;

            // Bits 0-11: Size - 3
            // Bits 12-14: Signature (3)
            // Bit 15: Compressed flag
            let size = (header & 0x0FFF) as usize + 1;
            let compressed_block = (header & 0x8000) != 0;
            let block_end = (coffset + size).min(compressed.len());

            if !compressed_block {
                let to_copy = (block_end - coffset).min(size);
                decompressed.extend_from_slice(&compressed[coffset..coffset + to_copy]);
                coffset += to_copy;
            } else {
                let block_start_offset = decompressed.len();
                while coffset < block_end {
                    let flags = compressed[coffset];
                    coffset += 1;

                    for i in 0..8 {
                        if coffset >= block_end {
                            break;
                        }

                        if (flags >> i) & 1 == 0 {
                            decompressed.push(compressed[coffset]);
                            coffset += 1;
                        } else {
                            if coffset + 2 > block_end {
                                break;
                            }
                            let meta =
                                u16::from_le_bytes([compressed[coffset], compressed[coffset + 1]]);
                            coffset += 2;

                            let current_offset_in_block = decompressed.len() - block_start_offset;
                            if current_offset_in_block == 0 {
                                return Err(OperationError::ProcessingError(
                                    "Invalid LZNT1 stream: compression token at start of block"
                                        .to_string(),
                                ));
                            }

                            let mut len_bits = 0;
                            let mut temp = current_offset_in_block - 1;
                            while temp >= 0x10 {
                                temp >>= 1;
                                len_bits += 1;
                            }
                            len_bits = std::cmp::max(len_bits, 4);

                            let len = (meta & ((1 << len_bits) - 1)) as usize + 3;
                            let disp = (meta >> len_bits) as usize + 1;

                            if decompressed.len() < disp {
                                return Err(OperationError::ProcessingError(
                                    "Invalid displacement".to_string(),
                                ));
                            }

                            let mut source_offset = decompressed.len() - disp;
                            for _ in 0..len {
                                let val = decompressed[source_offset];
                                decompressed.push(val);
                                source_offset += 1;
                            }
                        }
                    }
                }
            }
        }
        Ok(decompressed)
    }
}

impl Operation for LZNT1Decompress {
    fn name(&self) -> &'static str {
        "LZNT1 Decompress"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Decompresses data using the LZNT1 algorithm.<br><br>Similar to the Windows API <code>RtlDecompressBuffer</code>."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        Self::decompress(&input)
    }
}
