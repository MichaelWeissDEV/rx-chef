/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XXTEA Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError, Utils};

/// XXTEA Decrypt operation
pub struct XxteaDecryptOp;

impl Operation for XxteaDecryptOp {
    fn name(&self) -> &'static str {
        "XXTEA Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Corrected Block TEA (often referred to as XXTEA) is a block cipher designed to correct weaknesses in the original Block TEA. XXTEA operates on variable-length blocks that are some arbitrary multiple of 32 bits in size (minimum 64 bits). The number of full cycles depends on the block size, but there are at least six (rising to 32 for small block sizes)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Key",
            description: "Key for XXTEA decryption",
            default_value: "",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(input);
        }

        let key_arg = args.first().ok_or(OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "Missing key".to_string(),
        })?;
        let key_bytes = Utils::convert_to_byte_array(key_arg)?;
        let fixed_key = fix_key(&key_bytes);

        let v = to_uint32_array(&input, false);
        let k = to_uint32_array(&fixed_key, false);

        let decrypted_v = decrypt_uint32_array(v, &k);
        let result = to_uint8_array(&decrypted_v, true);

        if result.is_empty() && !input.is_empty() {
            // to_uint8_array returns empty if length check fails
            return Err(OperationError::ProcessingError(
                "Decryption failed (invalid length in trailer)".to_string(),
            ));
        }

        Ok(result)
    }
}

const DELTA: u32 = 0x9E3779B9;

fn mx(sum: u32, y: u32, z: u32, p: usize, e: u32, k: &[u32]) -> u32 {
    ((z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4))
        ^ ((sum ^ y).wrapping_add(k[(p & 3) ^ (e as usize)] ^ z))
}

fn fix_key(k: &[u8]) -> Vec<u8> {
    if k.len() < 16 {
        let mut key = vec![0u8; 16];
        key[..k.len()].copy_from_slice(k);
        key
    } else {
        k.to_vec()
    }
}

fn to_uint32_array(bs: &[u8], include_length: bool) -> Vec<u32> {
    let length = bs.len();
    let mut n = length >> 2;
    if (length & 3) != 0 {
        n += 1;
    }
    let mut v = if include_length {
        let mut vec = vec![0u32; n + 1];
        vec[n] = length as u32;
        vec
    } else {
        vec![0u32; n]
    };
    for i in 0..length {
        v[i >> 2] |= (bs[i] as u32) << ((i & 3) << 3);
    }
    v
}

fn to_uint8_array(v: &[u32], include_length: bool) -> Vec<u8> {
    let length = v.len();
    if length == 0 {
        return Vec::new();
    }
    let mut n = length << 2;
    if include_length {
        let m = v[length - 1] as usize;
        n -= 4;
        if m > n || m < n - 3 {
            return Vec::new();
        }
        n = m;
    }
    let mut bytes = vec![0u8; n];
    for i in 0..n {
        bytes[i] = (v[i >> 2] >> ((i & 3) << 3)) as u8;
    }
    bytes
}

fn decrypt_uint32_array(mut v: Vec<u32>, k: &[u32]) -> Vec<u32> {
    let length = v.len();
    if length < 2 {
        return v;
    }
    let n = length - 1;
    let mut y = v[0];
    let q = 6 + 52 / length;
    let mut sum: u32 = (q as u32).wrapping_mul(DELTA);
    while sum != 0 {
        let e = (sum >> 2) & 3;
        for p in (1..=n).rev() {
            let z = v[p - 1];
            v[p] = v[p].wrapping_sub(mx(sum, y, z, p, e, k));
            y = v[p];
        }
        let z = v[n];
        v[0] = v[0].wrapping_sub(mx(sum, y, z, 0, e, k));
        y = v[0];
        sum = sum.wrapping_sub(DELTA);
    }
    v
}
