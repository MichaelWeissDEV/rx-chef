/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Description: Shared GOST R 34.13-2015 "MAC mode" (CMAC/OMAC1-style checksum)
 *              and CryptoPro KEK diversification primitives, used by the
 *              GOST Sign, GOST Verify, GOST Key Wrap, and GOST Key Unwrap
 *              operations.
 *
 *              Ported to match the vendored reference implementation used by
 *              CyberChef (gchq/CyberChef, src/core/vendor/gost/gostCipher.mjs,
 *              functions `processMAC15` and `diversifyKEK`, as of commit
 *              b92501ee354256a127479f93d4c31a4f1d0dd657), which in turn
 *              implements RFC 4357 section 6.5 (CryptoPro KEK Diversification
 *              Algorithm) and the GOST R 34.13-2015 MAC construction.
 *
 *              Note: `processMAC15` is used by the reference implementation
 *              for algorithm version 2015 (Magma and Kuznyechik). Algorithm
 *              version 1989 (GOST 28147-89) instead uses a different,
 *              round-reduced "imitovstavka" construction (`processMAC89`)
 *              with a selectable S-box, which is NOT implemented here. This
 *              crate's "GOST 28147 (1989)" algorithm selection is an alias
 *              for GOST R 34.12-2015 Magma (matching this crate's existing
 *              GOST Encrypt/Decrypt behaviour), so the MAC construction below
 *              is used for both selections. This is a documented, intentional
 *              divergence from strict GOST 28147-89 semantics, not an
 *              oversight.
 * -----------------------------------------------------------------------------
 */

use cipher::{BlockCipher, BlockEncrypt, BlockSizeUser, KeyInit};
use generic_array::GenericArray;

use crate::operation::OperationError;

/// Doubles `block` in GF(2^n) using the reduction polynomial appropriate for
/// its length (matches `processKeyMAC15` in the reference implementation):
/// x^64 + x^4 + x^3 + x + 1 (constant 0x1b) for 8-byte blocks, and
/// x^128 + x^7 + x^2 + x + 1 (constant 0x87) for 16-byte blocks, the same
/// construction used by AES-CMAC (RFC 4493) generalized to other block sizes.
fn gf_double(block: &mut [u8]) {
    let n = block.len();
    let carry = block[0] & 0x80 != 0;
    for i in 0..n - 1 {
        block[i] = (block[i] << 1) | (block[i + 1] >> 7);
    }
    block[n - 1] <<= 1;
    if carry {
        let r: u8 = if n == 16 { 0x87 } else { 0x1b };
        block[n - 1] ^= r;
    }
}

/// Padding used by the reference implementation's `bitPad`: append a single
/// 0x01 byte then zero-pad to a multiple of `block_size`. Note this is NOT
/// the ISO/IEC 7816-4 (0x80 ...) convention despite the superficial
/// resemblance — matching the exact byte value matters for bit-exact MAC
/// output.
fn bit_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut padded = data.to_vec();
    padded.push(0x01);
    while padded.len() % block_size != 0 {
        padded.push(0);
    }
    padded
}

/// Computes the GOST R 34.13-2015 MAC ("processMAC15") of `data` under `key`,
/// with initial register `iv` (defaults to an all-zero register of one block
/// if `None`, matching the reference implementation's default IV). Returns
/// the full block-size register; callers truncate to the desired MAC length.
///
/// Faithfully replicates the reference implementation's specific (and
/// slightly non-standard relative to textbook CMAC) handling of an empty
/// message: an empty `data` is treated as zero already-aligned blocks, so no
/// block is processed at all and the returned value is simply the initial
/// register unchanged.
pub(crate) fn gost_cmac<C>(key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Vec<u8>
where
    C: BlockCipher + BlockSizeUser + KeyInit + BlockEncrypt,
{
    let n = C::block_size();
    let cipher = C::new(GenericArray::from_slice(key));

    let mut register: Vec<u8> = match iv {
        Some(v) => v.to_vec(),
        None => vec![0u8; n],
    };

    if data.is_empty() {
        return register;
    }

    let mut zero_block = GenericArray::<u8, C::BlockSize>::default();
    cipher.encrypt_block(&mut zero_block);
    let mut k1 = zero_block.to_vec();
    gf_double(&mut k1);

    let (blocks, tail_key) = if data.len() % n == 0 {
        (data.to_vec(), k1)
    } else {
        let mut k2 = k1.clone();
        gf_double(&mut k2);
        (bit_pad(data, n), k2)
    };

    let num_blocks = blocks.len() / n;
    for (i, chunk) in blocks.chunks(n).enumerate() {
        for (r, c) in register.iter_mut().zip(chunk) {
            *r ^= c;
        }
        if i == num_blocks - 1 {
            for (r, t) in register.iter_mut().zip(&tail_key) {
                *r ^= t;
            }
        }
        let mut block = GenericArray::<u8, C::BlockSize>::clone_from_slice(&register);
        cipher.encrypt_block(&mut block);
        register = block.to_vec();
    }

    register
}

/// CryptoPro KEK Diversification Algorithm (RFC 4357 section 6.5), used by
/// GOST Key Wrap/Unwrap "CP" mode. Only defined here for 64-bit block
/// ciphers (GOST 28147-89 / GOST R 34.12-2015 Magma): the reference
/// implementation generalizes the RFC's fixed 8-iteration/8-byte-UKM
/// algorithm to `block_size` iterations for other block sizes, but that
/// generalization is internally inconsistent for 128-bit ciphers (its own
/// CFB helper cannot self-key-encrypt a 32-byte value one 16-byte block at a
/// time against a 16-byte derived IV) and throws at runtime for Kuznyechik.
/// Callers must reject CP-mode wrapping for 128-bit block ciphers rather
/// than invent a result the reference implementation itself cannot produce.
pub(crate) fn diversify_kek_64<C>(kek: &[u8], ukm: &[u8]) -> Result<Vec<u8>, OperationError>
where
    C: BlockCipher + BlockSizeUser + KeyInit + BlockEncrypt,
{
    let n = C::block_size();
    if n != 8 {
        return Err(OperationError::InvalidArgument {
            name: "Key wrapping".to_string(),
            reason:
                "CryptoPro ('CP') key wrapping is only supported for 64-bit block ciphers (GOST 28147 (1989) / GOST R 34.12 (Magma, 2015)); the reference implementation's KEK diversification algorithm is not well-defined for 128-bit block ciphers such as Kuznyechik."
                    .to_string(),
        });
    }
    if ukm.len() != n {
        return Err(OperationError::InvalidArgument {
            name: "User Key Material".to_string(),
            reason: format!("UKM must be {n} bytes for this algorithm"),
        });
    }
    if kek.len() % 4 != 0 {
        return Err(OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "KEK length must be a multiple of 4 bytes".to_string(),
        });
    }

    // K[0] = KEK, as 32-bit little-endian words.
    let mut k: Vec<u32> = kek
        .chunks_exact(4)
        .map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
        .collect();

    for byte in ukm.iter().take(n) {
        // Split this UKM byte into bits a[0..8), summing the corresponding
        // 32-bit words of K into s0 (bit set) / s1 (bit clear).
        let mut s0: u32 = 0;
        let mut s1: u32 = 0;
        for (j, word) in k.iter().enumerate().take(8) {
            if (byte >> j) & 1 == 1 {
                s0 = s0.wrapping_add(*word);
            } else {
                s1 = s1.wrapping_add(*word);
            }
        }
        let mut iv = [0u8; 8];
        iv[0..4].copy_from_slice(&s0.to_le_bytes());
        iv[4..8].copy_from_slice(&s1.to_le_bytes());

        // K[i+1] = encryptCFB(iv = S[i], key = K[i], data = K[i]) with
        // full-block feedback (self-keyed CFB of the key material).
        let key_bytes: Vec<u8> = k.iter().flat_map(|w| w.to_le_bytes()).collect();
        let cipher = C::new(GenericArray::from_slice(&key_bytes));
        let mut register = iv;
        let mut next_bytes = Vec::with_capacity(key_bytes.len());
        for chunk in key_bytes.chunks(n) {
            let mut block = GenericArray::<u8, C::BlockSize>::clone_from_slice(&register);
            cipher.encrypt_block(&mut block);
            let mut out = [0u8; 8];
            for i in 0..n {
                out[i] = chunk[i] ^ block[i];
            }
            next_bytes.extend_from_slice(&out[..n]);
            register.copy_from_slice(&out[..n]);
        }
        k = next_bytes
            .chunks_exact(4)
            .map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
            .collect();
    }

    Ok(k.iter().flat_map(|w| w.to_le_bytes()).collect())
}
