/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LM Hash operation.
 * -----------------------------------------------------------------------------
 */

use cipher::{BlockEncrypt, KeyInit};
use des::Des;
use generic_array::GenericArray;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// LM Hash operation
pub struct LMHash;

impl Operation for LMHash {
    fn name(&self) -> &'static str {
        "LM Hash"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "An LM Hash, or LAN Manager Hash, is a deprecated way of storing passwords on old Microsoft operating systems. It is particularly weak and can be cracked in seconds on modern hardware using rainbow tables."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let mut pwd = input_str.to_uppercase().as_bytes().to_vec();
        pwd.resize(14, 0);

        let k1 = &pwd[0..7];
        let k2 = &pwd[7..14];

        let r1 = des_encrypt_7byte_key(k1, b"KGS!@#$%");
        let r2 = des_encrypt_7byte_key(k2, b"KGS!@#$%");

        let mut result = [0u8; 32];
        hex::encode_to_slice(r1, &mut result[0..16]).unwrap();
        hex::encode_to_slice(r2, &mut result[16..32]).unwrap();

        Ok(result.to_vec())
    }
}

fn des_encrypt_7byte_key(key7: &[u8], data: &[u8; 8]) -> [u8; 8] {
    let mut key8 = [0u8; 8];
    key8[0] = key7[0] & 0xFE;
    key8[1] = ((key7[0] << 7) & 0x80) | ((key7[1] >> 1) & 0x7E);
    key8[2] = ((key7[1] << 6) & 0xC0) | ((key7[2] >> 2) & 0x3E);
    key8[3] = ((key7[2] << 5) & 0xE0) | ((key7[3] >> 3) & 0x1E);
    key8[4] = ((key7[3] << 4) & 0xF0) | ((key7[4] >> 4) & 0x0E);
    key8[5] = ((key7[4] << 3) & 0xF8) | ((key7[5] >> 5) & 0x06);
    key8[6] = ((key7[5] << 2) & 0xFC) | ((key7[6] >> 6) & 0x02);
    key8[7] = (key7[6] << 1) & 0xFE;

    let key = GenericArray::from_slice(&key8);
    let cipher = Des::new(key);
    let mut block = GenericArray::clone_from_slice(data);
    cipher.encrypt_block(&mut block);

    let mut out = [0u8; 8];
    out.copy_from_slice(&block);
    out
}
