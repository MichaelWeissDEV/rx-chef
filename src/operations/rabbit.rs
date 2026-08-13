/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Rabbit operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

struct RabbitState {
    x: [u32; 8],
    c: [u32; 8],
    carry: u32,
}

impl RabbitState {
    fn update(&mut self) {
        const A: [u32; 8] = [
            0x4d34d34d, 0xd34d34d3, 0x34d34d34, 0x4d34d34d, 0xd34d34d3, 0x34d34d34, 0x4d34d34d,
            0xd34d34d3,
        ];
        for (counter, increment) in self.c.iter_mut().zip(A) {
            let sum = *counter as u64 + increment as u64 + self.carry as u64;
            *counter = sum as u32;
            self.carry = (sum >> 32) as u32;
        }
        let mut g = [0u32; 8];
        for index in 0..8 {
            let value = self.x[index].wrapping_add(self.c[index]) as u64;
            let square = value * value;
            g[index] = square as u32 ^ (square >> 32) as u32;
        }
        self.x = [
            g[0].wrapping_add(g[7].rotate_left(16))
                .wrapping_add(g[6].rotate_left(16)),
            g[1].wrapping_add(g[0].rotate_left(8)).wrapping_add(g[7]),
            g[2].wrapping_add(g[1].rotate_left(16))
                .wrapping_add(g[0].rotate_left(16)),
            g[3].wrapping_add(g[2].rotate_left(8)).wrapping_add(g[1]),
            g[4].wrapping_add(g[3].rotate_left(16))
                .wrapping_add(g[2].rotate_left(16)),
            g[5].wrapping_add(g[4].rotate_left(8)).wrapping_add(g[3]),
            g[6].wrapping_add(g[5].rotate_left(16))
                .wrapping_add(g[4].rotate_left(16)),
            g[7].wrapping_add(g[6].rotate_left(8)).wrapping_add(g[5]),
        ];
    }

    fn keystream(&mut self, little_endian: bool) -> [u8; 16] {
        self.update();
        let words = [
            (self.x[6] >> 16) ^ (self.x[1] & 0xffff),
            (self.x[6] & 0xffff) ^ (self.x[3] >> 16),
            (self.x[4] >> 16) ^ (self.x[7] & 0xffff),
            (self.x[4] & 0xffff) ^ (self.x[1] >> 16),
            (self.x[2] >> 16) ^ (self.x[5] & 0xffff),
            (self.x[2] & 0xffff) ^ (self.x[7] >> 16),
            (self.x[0] >> 16) ^ (self.x[3] & 0xffff),
            (self.x[0] & 0xffff) ^ (self.x[5] >> 16),
        ];
        let mut stream = [0; 16];
        for (index, word) in words.into_iter().enumerate() {
            stream[index * 2] = (word >> 8) as u8;
            stream[index * 2 + 1] = word as u8;
        }
        if little_endian {
            stream.reverse();
        }
        stream
    }
}

fn rabbit_state(key: &[u8], iv: &[u8], little_endian: bool) -> RabbitState {
    let mut k = [0u16; 8];
    for index in 0..8 {
        k[index] = if little_endian {
            u16::from_le_bytes([key[index * 2], key[index * 2 + 1]])
        } else {
            u16::from_be_bytes([key[14 - index * 2], key[15 - index * 2]])
        };
    }
    let mut state = RabbitState {
        x: std::array::from_fn(|index| {
            if index % 2 == 0 {
                ((k[(index + 1) % 8] as u32) << 16) | k[index] as u32
            } else {
                ((k[(index + 5) % 8] as u32) << 16) | k[(index + 4) % 8] as u32
            }
        }),
        c: std::array::from_fn(|index| {
            if index % 2 == 0 {
                ((k[(index + 4) % 8] as u32) << 16) | k[(index + 5) % 8] as u32
            } else {
                ((k[index] as u32) << 16) | k[(index + 1) % 8] as u32
            }
        }),
        carry: 0,
    };
    for _ in 0..4 {
        state.update();
    }
    for index in 0..8 {
        state.c[index] ^= state.x[(index + 4) % 8];
    }
    if iv.len() == 8 {
        let bytes = if little_endian {
            iv.to_vec()
        } else {
            iv.iter().rev().copied().collect()
        };
        let iv0 = u32::from_be_bytes([bytes[3], bytes[2], bytes[1], bytes[0]]);
        let iv1 = u32::from_be_bytes([bytes[7], bytes[6], bytes[3], bytes[2]]);
        let iv2 = u32::from_be_bytes([bytes[7], bytes[6], bytes[5], bytes[4]]);
        let iv3 = u32::from_be_bytes([bytes[5], bytes[4], bytes[1], bytes[0]]);
        for (index, value) in [iv0, iv1, iv2, iv3, iv0, iv1, iv2, iv3]
            .into_iter()
            .enumerate()
        {
            state.c[index] ^= value;
        }
        for _ in 0..4 {
            state.update();
        }
    }
    state
}

/// Rabbit operation
pub struct RabbitOp;

impl Operation for RabbitOp {
    fn name(&self) -> &'static str {
        "Rabbit"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Rabbit is a high-speed stream cipher introduced in 2003 and defined in RFC 4503.<br><br>The cipher uses a 128-bit key and an optional 64-bit initialization vector (IV).<br><br>big-endian: based on RFC4503 and RFC3447<br>little-endian: compatible with Crypto++"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "128-bit key",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "64-bit IV",
                default_value: "",
            },
            ArgSchema {
                name: "Endianness",
                description: "Big or Little",
                default_value: "Big",
            },
            ArgSchema {
                name: "Input",
                description: "Raw or Hex",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Output",
                description: "Raw or Hex",
                default_value: "Raw",
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
        let key_arg = args.first().ok_or(OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "Missing".to_string(),
        })?;
        let iv_arg = args.get(1).ok_or(OperationError::InvalidArgument {
            name: "IV".to_string(),
            reason: "Missing".to_string(),
        })?;
        let endianness = args.get(2).and_then(|v| v.as_str()).unwrap_or("Big");
        let input_type = args.get(3).and_then(|v| v.as_str()).unwrap_or("Raw");
        let output_type = args.get(4).and_then(|v| v.as_str()).unwrap_or("Raw");

        let key_bytes = crate::operation::Utils::convert_to_byte_array(key_arg)?;
        let iv_bytes = crate::operation::Utils::convert_to_byte_array(iv_arg)?;

        if key_bytes.len() != 16 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes (expected: 16)",
                    key_bytes.len()
                ),
            });
        }
        if !iv_bytes.is_empty() && iv_bytes.len() != 8 {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: format!(
                    "Invalid IV length: {} bytes (expected: 0 or 8)",
                    iv_bytes.len()
                ),
            });
        }

        let mut data = if input_type == "Hex" {
            let s = String::from_utf8_lossy(&input)
                .replace(' ', "")
                .replace('\n', "")
                .replace('\r', "");
            hex::decode(s)
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?
        } else {
            input
        };

        let little_endian = match endianness {
            "Big" => false,
            "Little" => true,
            value => {
                return Err(OperationError::InvalidArgument {
                    name: "Endianness".to_string(),
                    reason: format!("expected Big or Little, got '{value}'"),
                });
            }
        };
        let mut state = rabbit_state(&key_bytes, &iv_bytes, little_endian);
        let data_len = data.len();
        for (block_index, block) in data.chunks_mut(16).enumerate() {
            let stream = state.keystream(little_endian);
            let stream_start = if !little_endian && block.len() < 16 {
                16 - block.len()
            } else {
                0
            };
            for (index, byte) in block.iter_mut().enumerate() {
                *byte ^= stream[stream_start + index];
            }
            debug_assert!(block_index * 16 < data_len || data_len == 0);
        }

        if output_type == "Hex" {
            Ok(hex::encode(data).into_bytes())
        } else {
            Ok(data)
        }
    }
}
