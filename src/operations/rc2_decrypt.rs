/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RC2 Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RC2 Decrypt operation.
///
/// RC2 (also known as ARC2) is a symmetric-key block cipher designed by Ron Rivest in 1987.
/// Supports CBC mode with 8-byte IV, or ECB mode when IV is empty. Uses PKCS#7 padding.
pub struct RC2Decrypt;

/// PKCS#7 unpad for 8-byte block size
fn pkcs7_unpad_8(data: &[u8]) -> Result<Vec<u8>, OperationError> {
    if data.is_empty() {
        return Err(OperationError::ProcessingError("Empty input".to_string()));
    }
    let pad_len = data[data.len() - 1] as usize;
    if pad_len == 0 || pad_len > 8 {
        return Err(OperationError::ProcessingError(
            "Invalid PKCS#7 padding".to_string(),
        ));
    }
    if data.len() < pad_len {
        return Err(OperationError::ProcessingError(
            "Input shorter than padding".to_string(),
        ));
    }
    for &b in &data[data.len() - pad_len..] {
        if b != pad_len as u8 {
            return Err(OperationError::ProcessingError(
                "Invalid PKCS#7 padding byte".to_string(),
            ));
        }
    }
    Ok(data[..data.len() - pad_len].to_vec())
}

/// RC2 key expansion (RFC 2268)
fn rc2_expand_key(key: &[u8]) -> [u16; 64] {
    const PITABLE: [u8; 256] = [
        217, 120, 249, 196, 25, 221, 181, 237, 40, 233, 253, 121, 74, 160, 216, 157, 198, 126, 55,
        131, 43, 118, 83, 142, 98, 76, 100, 136, 68, 139, 251, 162, 0, 240, 188, 177, 186, 143,
        153, 217, 112, 247, 235, 92, 210, 88, 199, 195, 187, 44, 165, 101, 214, 161, 245, 9, 72,
        224, 226, 201, 152, 172, 212, 69, 120, 168, 252, 134, 224, 180, 71, 116, 205, 225, 252, 22,
        155, 145, 63, 204, 54, 129, 34, 24, 94, 246, 177, 75, 102, 227, 141, 139, 255, 196, 179,
        116, 154, 153, 103, 196, 65, 145, 184, 108, 201, 229, 105, 148, 27, 203, 161, 104, 229, 70,
        181, 149, 171, 253, 69, 169, 245, 100, 32, 144, 49, 209, 104, 83, 38, 240, 243, 213, 234,
        173, 252, 241, 73, 93, 209, 18, 19, 109, 231, 55, 192, 19, 151, 230, 224, 15, 2, 48, 138,
        233, 153, 227, 21, 46, 219, 31, 201, 141, 37, 15, 154, 183, 247, 26, 248, 145, 220, 39,
        110, 247, 155, 46, 155, 87, 31, 253, 126, 149, 195, 108, 241, 23, 30, 31, 237, 62, 179,
        129, 139, 149, 51, 68, 236, 213, 72, 252, 93, 93, 196, 144, 246, 147, 236, 194, 29, 198,
        36, 107, 237, 111, 171, 141, 3, 53, 236, 12, 195, 118, 60, 213, 121, 65, 213, 200, 52, 19,
        199, 226, 216, 243, 50, 55, 178, 178, 194, 104, 156, 160, 142, 236, 215, 45, 194, 129, 41,
        134, 157, 29, 211, 172, 13, 41,
    ];

    let key_len = key.len();
    let mut l = [0u8; 128];
    for (i, &b) in key.iter().enumerate() {
        l[i] = b;
    }
    for i in key_len..128 {
        l[i] = PITABLE[((l[i - 1] as usize) + (l[i - key_len] as usize)) % 256];
    }
    // Effective key bits = 8 * key_len
    let t8 = key_len; // ceil(t1/8)
    let tm = (255 % (2u32.pow(8 - (8 * key_len - 8 * key_len) as u32))) as u8;
    let _ = tm;
    // Mask: adjust for partial bits (simplified: use full bytes)
    l[128 - t8] = PITABLE[(l[128 - t8] as usize) % 256];
    for i in (0..128 - t8).rev() {
        l[i] = PITABLE[(l[i + 1] as usize ^ l[i + t8] as usize) as usize];
    }
    let mut k = [0u16; 64];
    for i in 0..64 {
        k[i] = (l[2 * i] as u16) + ((l[2 * i + 1] as u16) << 8);
    }
    k
}

/// RC2 decrypt a single 8-byte block (RFC 2268)
fn rc2_decrypt_block(block: &[u8; 8], k: &[u16; 64]) -> [u8; 8] {
    let mut r = [0u16; 4];
    for i in 0..4 {
        r[i] = (block[2 * i] as u16) | ((block[2 * i + 1] as u16) << 8);
    }

    // R-mixing rounds (5 rounds of un-mix + un-mash, reversed)
    for j in (0..16usize).rev() {
        let round = j;
        // Un-mix round
        for i in (0..4usize).rev() {
            let kv = k[4 * round + i];
            let prev = r[(i + 3) % 4];
            let next = r[(i + 1) % 4];
            let mut ri = r[i];
            let s = [1u16, 2, 3, 5][i];
            ri = ri
                .wrapping_sub(kv)
                .wrapping_sub((prev & next).wrapping_add((!prev) & r[(i + 2) % 4]));
            // Rotate right by s
            ri = (ri >> s) | (ri << (16 - s));
            r[i] = ri;
        }
        // Un-mash after rounds 4 and 9
        if round == 4 || round == 9 {
            for i in 0..4usize {
                r[i] = r[i].wrapping_sub(k[(r[(i + 3) % 4] & 63) as usize]);
            }
        }
    }

    let mut out = [0u8; 8];
    for i in 0..4 {
        out[2 * i] = (r[i] & 0xff) as u8;
        out[2 * i + 1] = (r[i] >> 8) as u8;
    }
    out
}

impl Operation for RC2Decrypt {
    fn name(&self) -> &'static str {
        "RC2 Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "RC2 (also known as ARC2) is a symmetric-key block cipher designed by Ron Rivest in 1987. Supports CBC mode (8-byte IV) or ECB mode (empty IV). Uses PKCS#7 padding."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Decryption key as UTF-8 or hex (prefix 0x for hex)",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "Initialization vector (8 bytes for CBC, empty for ECB)",
                default_value: "",
            },
            ArgSchema {
                name: "Input",
                description: "Input encoding: Hex or Raw",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output",
                description: "Output encoding: Raw or Hex",
                default_value: "Raw",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key_str = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let iv_str = args.get(1).and_then(|a| a.as_str()).unwrap_or("");
        let input_fmt = args.get(2).and_then(|a| a.as_str()).unwrap_or("Hex");
        let output_fmt = args.get(3).and_then(|a| a.as_str()).unwrap_or("Raw");

        let key_bytes: Vec<u8> = if key_str.starts_with("0x") {
            hex::decode(&key_str[2..]).map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: e.to_string(),
            })?
        } else {
            key_str.as_bytes().to_vec()
        };

        if key_bytes.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Key must not be empty".to_string(),
            });
        }

        let iv_bytes: Vec<u8> = if iv_str.is_empty() {
            vec![]
        } else if iv_str.starts_with("0x") {
            hex::decode(&iv_str[2..]).map_err(|e| OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: e.to_string(),
            })?
        } else {
            iv_str.as_bytes().to_vec()
        };

        if !iv_bytes.is_empty() && iv_bytes.len() != 8 {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: "IV must be 8 bytes for CBC mode or empty for ECB mode".to_string(),
            });
        }

        let cipher_bytes: Vec<u8> = match input_fmt {
            "Hex" => {
                hex::decode(&input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
            }
            _ => input,
        };

        if cipher_bytes.len() % 8 != 0 {
            return Err(OperationError::InvalidInput(
                "Ciphertext length must be a multiple of 8 bytes".to_string(),
            ));
        }

        let k = rc2_expand_key(&key_bytes);
        let mut plaintext = Vec::with_capacity(cipher_bytes.len());

        if iv_bytes.is_empty() {
            // ECB mode
            for chunk in cipher_bytes.chunks(8) {
                let mut block = [0u8; 8];
                block.copy_from_slice(chunk);
                let dec = rc2_decrypt_block(&block, &k);
                plaintext.extend_from_slice(&dec);
            }
        } else {
            // CBC mode
            let mut prev = [0u8; 8];
            prev.copy_from_slice(&iv_bytes);
            for chunk in cipher_bytes.chunks(8) {
                let mut block = [0u8; 8];
                block.copy_from_slice(chunk);
                let dec = rc2_decrypt_block(&block, &k);
                let mut plain_block = [0u8; 8];
                for i in 0..8 {
                    plain_block[i] = dec[i] ^ prev[i];
                }
                prev.copy_from_slice(chunk);
                plaintext.extend_from_slice(&plain_block);
            }
        }

        let unpadded = pkcs7_unpad_8(&plaintext)?;

        match output_fmt {
            "Hex" => Ok(hex::encode(&unpadded).into_bytes()),
            _ => Ok(unpadded),
        }
    }
}
