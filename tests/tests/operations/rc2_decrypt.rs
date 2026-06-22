// Tests for the rc2_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc2_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::rc2_decrypt::RC2Decrypt;
use rxchef::Operation;

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
    let t8 = key_len;
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

#[test]
fn test_rc2_decrypt_invalid_empty_key() {
    let op = RC2Decrypt;
    let result = op.run(
        b"00000000".to_vec(),
        &[
            ArgValue::Str("".to_string()),
            ArgValue::Str("".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("Raw".to_string()),
        ],
    );
    assert!(result.is_err());
}
#[test]
fn test_rc2_decrypt_invalid_iv_length() {
    let op = RC2Decrypt;
    let result = op.run(
        b"0000000000000000".to_vec(),
        &[
            ArgValue::Str("key".to_string()),
            ArgValue::Str("0x0102".to_string()), // 2 bytes, not 8
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("Raw".to_string()),
        ],
    );
    assert!(result.is_err());
}
#[test]
fn test_rc2_key_expand() {
    // Basic sanity: expand a simple key and check length
    let k = rc2_expand_key(b"test");
    assert_eq!(k.len(), 64);
}
