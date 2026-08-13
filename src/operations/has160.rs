/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the HAS-160 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// HAS-160 operation
pub struct HAS160Op;

impl Operation for HAS160Op {
    fn name(&self) -> &'static str {
        "HAS-160"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "HAS-160 is a cryptographic hash function designed for use with the Korean KCDSA digital signature algorithm."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Rounds",
            description: "Number of rounds",
            default_value: "80",
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
        if !(1..=80).contains(&rounds) {
            return Err(OperationError::InvalidArgument {
                name: "Rounds".into(),
                reason: "Rounds must be between 1 and 80".into(),
            });
        }
        Ok(hex::encode(has160(&input, rounds)).into_bytes())
    }
}

const INITIAL_STATE: [u32; 5] = [
    0x6745_2301,
    0xefcd_ab89,
    0x98ba_dcfe,
    0x1032_5476,
    0xc3d2_e1f0,
];
const ROTATIONS: [u32; 20] = [
    5, 11, 7, 15, 6, 13, 8, 14, 7, 12, 9, 11, 8, 15, 6, 12, 9, 14, 5, 13,
];
const MESSAGE_INDEX: [[usize; 20]; 4] = [
    [
        18, 0, 1, 2, 3, 19, 4, 5, 6, 7, 16, 8, 9, 10, 11, 17, 12, 13, 14, 15,
    ],
    [
        18, 3, 6, 9, 12, 19, 15, 2, 5, 8, 16, 11, 14, 1, 4, 17, 7, 10, 13, 0,
    ],
    [
        18, 12, 5, 14, 7, 19, 0, 9, 2, 11, 16, 4, 13, 6, 15, 17, 8, 1, 10, 3,
    ],
    [
        18, 7, 2, 13, 8, 19, 3, 14, 9, 4, 16, 15, 10, 5, 0, 17, 11, 6, 1, 12,
    ],
];

fn has160(input: &[u8], rounds: usize) -> [u8; 20] {
    let mut padded = input.to_vec();
    let bit_length = (padded.len() as u64).wrapping_mul(8);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_length.to_le_bytes());
    let mut state = INITIAL_STATE;
    for block in padded.chunks_exact(64) {
        compress(&mut state, block, rounds);
    }
    let mut output = [0_u8; 20];
    for (bytes, word) in output.chunks_exact_mut(4).zip(state) {
        bytes.copy_from_slice(&word.to_le_bytes());
    }
    output
}

fn compress(state: &mut [u32; 5], block: &[u8], rounds: usize) {
    let mut words = [0_u32; 20];
    for (word, bytes) in words[..16].iter_mut().zip(block.chunks_exact(4)) {
        *word = u32::from_le_bytes(bytes.try_into().unwrap());
    }
    let mut work = *state;
    for step in 0..rounds {
        let group = step / 20;
        let local = step % 20;
        if local == 0 {
            let sources = match group {
                0 => [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]],
                1 => [[3, 6, 9, 12], [2, 5, 8, 15], [1, 4, 11, 14], [0, 7, 10, 13]],
                2 => [[5, 7, 12, 14], [0, 2, 9, 11], [4, 6, 13, 15], [1, 3, 8, 10]],
                _ => [[2, 7, 8, 13], [3, 4, 9, 14], [0, 5, 10, 15], [1, 6, 11, 12]],
            };
            for index in 0..4 {
                words[16 + index] = sources[index]
                    .into_iter()
                    .fold(0, |value, source| value ^ words[source]);
            }
        }
        let a = (5 - local % 5) % 5;
        let b = (a + 1) % 5;
        let c = (a + 2) % 5;
        let d = (a + 3) % 5;
        let e = (a + 4) % 5;
        let function = match group {
            0 => (work[b] & work[c]) | (!work[b] & work[d]),
            2 => work[c] ^ (work[b] | !work[d]),
            _ => work[b] ^ work[c] ^ work[d],
        };
        let constant: u32 = [0x0000_0000, 0x5a82_7999, 0x6ed9_eba1, 0x8f1b_bcdc][group];
        work[e] = work[e]
            .wrapping_add(work[a].rotate_left(ROTATIONS[local]))
            .wrapping_add(words[MESSAGE_INDEX[group][local]])
            .wrapping_add(constant)
            .wrapping_add(function);
        work[b] = work[b].rotate_left([10, 17, 25, 30][group]);
    }
    for (state_word, work_word) in state.iter_mut().zip(work) {
        *state_word = state_word.wrapping_add(work_word);
    }
}
