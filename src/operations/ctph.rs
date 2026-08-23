/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CTPH operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

const HASH_PRIME: u32 = 0x0100_0193;
const HASH_INIT: u32 = 0x2802_1967;
const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Default)]
struct RollHash {
    x: i64,
    y: i64,
    z: u32,
    count: usize,
    window: [u8; 7],
}

impl RollHash {
    fn update(&mut self, byte: u8) {
        self.y -= self.x;
        self.y += 7 * i64::from(byte);
        self.x += i64::from(byte);
        self.x -= i64::from(self.window[self.count % 7]);
        self.window[self.count % 7] = byte;
        self.count += 1;
        self.z = self.z.wrapping_shl(5) ^ u32::from(byte);
    }

    fn sum(&self) -> u32 {
        (self.x + self.y + i64::from(self.z)) as u32
    }
}

/// Reproduce JavaScript's `((base * prime) ^ byte) >>> 0` exactly. The
/// multiplication is performed as an IEEE-754 number before ToUint32, which
/// intentionally differs from wrapping u32 multiplication for some values.
fn fnv_js(base: u32, byte: u8) -> u32 {
    let product = f64::from(base) * f64::from(HASH_PRIME);
    let coerced = product.rem_euclid(4_294_967_296.0).trunc() as u32;
    coerced ^ u32::from(byte)
}

fn piecewise_hash(bytes: &[u8], trigger: u32) -> (String, String) {
    let mut first = String::new();
    let mut second = String::new();
    let mut h1 = HASH_INIT;
    let mut h2 = HASH_INIT;
    let mut rolling = RollHash::default();

    for (index, &byte) in bytes.iter().enumerate() {
        h1 = fnv_js(h1, byte);
        h2 = fnv_js(h2, byte);
        rolling.update(byte);
        let last = index + 1 == bytes.len();
        if last || rolling.sum() % trigger == trigger - 1 {
            first.push(B64[(h1 & 63) as usize] as char);
            h1 = HASH_INIT;
        }
        let double_trigger = trigger * 2;
        if last || rolling.sum() % double_trigger == double_trigger - 1 {
            second.push(B64[(h2 & 63) as usize] as char);
            h2 = HASH_INIT;
        }
    }
    (first, second)
}

/// CyberChef's `ctph.js` digest format. This is deliberately distinct from
/// ssdeep, which rx-chef exposes as a separate operation.
pub(crate) fn digest(bytes: &[u8]) -> String {
    let ratio = bytes.len() as f64 / (64.0 * 3.0);
    let calculated = ratio.log2().ceil();
    let mut block_index = if calculated.is_finite() {
        calculated.max(3.0) as u32
    } else {
        3
    };
    let (mut first, mut second) = piecewise_hash(bytes, 3u32.wrapping_shl(block_index));
    while block_index > 0 && first.len() < 32 {
        block_index -= 1;
        (first, second) = piecewise_hash(bytes, 3u32.wrapping_shl(block_index));
    }
    format!(
        "{}:{}:{}",
        B64[block_index as usize] as char, first, second
    )
}

fn levenshtein(left: &str, right: &str) -> usize {
    if left == right {
        return 0;
    }
    if left.is_empty() {
        return right.len();
    }
    if right.is_empty() {
        return left.len();
    }
    let right = right.as_bytes();
    let mut row: Vec<usize> = (0..=right.len()).collect();
    for (i, &left_byte) in left.as_bytes().iter().enumerate() {
        let mut next = i + 1;
        for (j, &right_byte) in right.iter().enumerate() {
            let current = next;
            next = (row[j] + usize::from(left_byte != right_byte))
                .min(current + 1)
                .min(row[j + 1] + 1);
            row[j] = current;
        }
        row[right.len()] = next;
    }
    row[right.len()]
}

pub(crate) fn similarity(left: &str, right: &str) -> f64 {
    let left_index = left
        .as_bytes()
        .first()
        .and_then(|byte| B64.iter().position(|candidate| candidate == byte))
        .map(|index| index as i32)
        .unwrap_or(-1);
    let right_index = right
        .as_bytes()
        .first()
        .and_then(|byte| B64.iter().position(|candidate| candidate == byte))
        .map(|index| index as i32)
        .unwrap_or(-1);
    if left_index > right_index {
        return similarity(right, left);
    }
    if (left_index - right_index).abs() > 1 {
        return 0.0;
    }
    let left_parts: Vec<&str> = left.split(':').collect();
    let right_parts: Vec<&str> = right.split(':').collect();
    let (a, b) = if left_index == right_index {
        (
            left_parts.get(1).copied().unwrap_or(""),
            right_parts.get(1).copied().unwrap_or(""),
        )
    } else {
        (
            left_parts.get(2).copied().unwrap_or(""),
            right_parts.get(1).copied().unwrap_or(""),
        )
    };
    let max_len = a.len().max(b.len());
    if max_len == 0 {
        return f64::NAN;
    }
    (1.0 - levenshtein(a, b) as f64 / max_len as f64) * 100.0
}

/// CTPH (Context Triggered Piecewise Hashing) operation
///
/// Context Triggered Piecewise Hashing, also called Fuzzy Hashing, can match
/// inputs that have homologies. Such inputs have sequences of identical bytes
/// in the same order, although bytes in between these sequences may be
/// different in both content and length.
pub struct CTPH;

impl Operation for CTPH {
    fn name(&self) -> &'static str {
        "CTPH"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Context Triggered Piecewise Hashing, also called Fuzzy Hashing, can match inputs that have homologies. Such inputs have sequences of identical bytes in the same order, although bytes in between these sequences may be different in both content and length."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;
        Ok(digest(input.as_bytes()).into_bytes())
    }
}
