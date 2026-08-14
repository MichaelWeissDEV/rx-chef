/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SHA0 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SHA0 operation
///
/// SHA-0 is the original 160-bit hash function published in 1993 under the
/// name 'SHA'. It was withdrawn shortly after publication due to a significant
/// flaw and replaced by the slightly revised version SHA-1. The key difference
/// is that SHA-0 does not rotate expanded message-schedule words.
pub struct SHA0;

impl Operation for SHA0 {
    fn name(&self) -> &'static str {
        "SHA0"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "SHA-0 is the original 160-bit hash function published in 1993 under the name 'SHA'. It was withdrawn shortly after publication due to an undisclosed 'significant flaw' and replaced by the slightly revised version SHA-1."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Rounds",
            description: "Number of rounds (minimum 16)",
            default_value: "80",
            kind: crate::operation::ArgKind::UnsignedInteger,
            required: false,
            choices: &[],
            minimum: None,
            maximum: None,
            sensitive: false,
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let rounds = args.first().and_then(|a| a.as_usize()).unwrap_or(80);
        if !(16..=80).contains(&rounds) {
            return Err(OperationError::InvalidArgument {
                name: "Rounds".to_string(),
                reason: "Rounds must be between 16 and 80".to_string(),
            });
        }

        let mut message = input;
        let bit_len = (message.len() as u64).wrapping_mul(8);
        message.push(0x80);
        while message.len() % 64 != 56 {
            message.push(0);
        }
        message.extend_from_slice(&bit_len.to_be_bytes());

        let mut state = [
            0x6745_2301u32,
            0xefcd_ab89,
            0x98ba_dcfe,
            0x1032_5476,
            0xc3d2_e1f0,
        ];
        for block in message.chunks_exact(64) {
            let mut words = [0u32; 80];
            for (index, word) in block.chunks_exact(4).enumerate() {
                words[index] = u32::from_be_bytes(word.try_into().unwrap());
            }
            for index in 16..80 {
                words[index] =
                    words[index - 3] ^ words[index - 8] ^ words[index - 14] ^ words[index - 16];
            }
            let [mut a, mut b, mut c, mut d, mut e] = state;
            for (index, word) in words.iter().take(rounds).enumerate() {
                let (function, constant) = match index {
                    0..=19 => ((b & c) | ((!b) & d), 0x5a82_7999),
                    20..=39 => (b ^ c ^ d, 0x6ed9_eba1),
                    40..=59 => ((b & c) | (b & d) | (c & d), 0x8f1b_bcdc),
                    _ => (b ^ c ^ d, 0xca62_c1d6),
                };
                let next = a
                    .rotate_left(5)
                    .wrapping_add(function)
                    .wrapping_add(e)
                    .wrapping_add(constant)
                    .wrapping_add(*word);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = next;
            }
            state[0] = state[0].wrapping_add(a);
            state[1] = state[1].wrapping_add(b);
            state[2] = state[2].wrapping_add(c);
            state[3] = state[3].wrapping_add(d);
            state[4] = state[4].wrapping_add(e);
        }
        Ok(state
            .into_iter()
            .map(|word| format!("{word:08x}"))
            .collect::<String>()
            .into_bytes())
    }
}
