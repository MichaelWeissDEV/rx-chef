/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XXTEA Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError, Utils};

/// XXTEA Encrypt operation
pub struct XxteaEncryptOp;

impl Operation for XxteaEncryptOp {
    fn name(&self) -> &'static str {
        "XXTEA Encrypt"
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
            description: "Key for XXTEA encryption",
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

        let v = to_uint32_array(&input, true);
        let k = to_uint32_array(&fixed_key, false);

        let encrypted_v = encrypt_uint32_array(v, &k);
        let result = to_uint8_array(&encrypted_v, false);

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
    let mut n = length << 2;
    if include_length {
        let m = v[length - 1] as usize;
        n -= 4;
        if m < n - 3 || m > n {
            // This should not happen in encrypt if logic is correct, but for completeness:
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

fn encrypt_uint32_array(mut v: Vec<u32>, k: &[u32]) -> Vec<u32> {
    let length = v.len();
    if length < 2 {
        return v;
    }
    let n = length - 1;
    let mut z = v[n];
    let mut sum: u32 = 0;
    let q = 6 + 52 / length;
    for _ in 0..q {
        sum = sum.wrapping_add(DELTA);
        let e = (sum >> 2) & 3;
        for p in 0..n {
            let y = v[p + 1];
            v[p] = v[p].wrapping_add(mx(sum, y, z, p, e, k));
            z = v[p];
        }
        let y = v[0];
        v[n] = v[n].wrapping_add(mx(sum, y, z, n, e, k));
        z = v[n];
    }
    v
}
