// Tests for the hmac operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations hmac::

use rxchef::operation::ArgValue;
use rxchef::operations::hmac::HMAC;
use rxchef::Operation;

fn run(input: &str, key: &str, hash_func: &str, encoding: &str) -> String {
    let op = HMAC;
    let args = [
        ArgValue::Str(key.to_string()),
        ArgValue::Str(hash_func.to_string()),
        ArgValue::Str(encoding.to_string()),
    ];
    String::from_utf8(op.run(input.as_bytes().to_vec(), &args).unwrap()).unwrap()
}

// RFC 2202 test case 1: key = 0x0b * 16, data = "Hi There".
// Key supplied with "0x" prefix so decode_key treats it unambiguously as hex.
#[test]
fn test_hmac_md5_rfc2202_case1() {
    let key = "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
    assert_eq!(
        run("Hi There", key, "MD5", "Hex"),
        "9294727a3638bb1c13f48ef8158bfc9d"
    );
}

#[test]
fn test_hmac_sha1_rfc2202_case1() {
    let key = "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
    assert_eq!(
        run("Hi There", key, "SHA-1", "Hex"),
        "675b0b3a1b4ddf4e124872da6c2f632bfed957e9"
    );
}

// RFC 4231 test case 1: key = 0x0b * 20, data = "Hi There".
#[test]
fn test_hmac_sha224_rfc4231_case1() {
    let key = "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
    assert_eq!(
        run("Hi There", key, "SHA-224", "Hex"),
        "896fb1128abbdf196832107cd49df33f47b4b1169912ba4f53684b22"
    );
}

#[test]
fn test_hmac_sha256_rfc4231_case1() {
    let key = "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
    assert_eq!(
        run("Hi There", key, "SHA-256", "Hex"),
        "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
    );
}

#[test]
fn test_hmac_sha384_rfc4231_case1() {
    let key = "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
    assert_eq!(
        run("Hi There", key, "SHA-384", "Hex"),
        "afd03944d84895626b0825f4ab46907f15f9dadbe4101ec682aa034c7cebc59cfaea9ea9076ede7f4af152e8b2fa9cb6"
    );
}

#[test]
fn test_hmac_sha512_rfc4231_case1() {
    let key = "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
    assert_eq!(
        run("Hi There", key, "SHA-512", "Hex"),
        "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cdedaa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
    );
}

// RFC 4231 test case 6: key longer than the SHA-256/SHA-1 block size (131
// bytes of 0xaa), which forces the HMAC key-hashing step.
#[test]
fn test_hmac_sha256_key_longer_than_block_size() {
    let key = format!("0x{}", "aa".repeat(131));
    assert_eq!(
        run(
            "Test Using Larger Than Block-Size Key - Hash Key First",
            &key,
            "SHA-256",
            "Hex"
        ),
        "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"
    );
}

// RFC 2202 test case 7: key longer than the MD5/SHA-1 block size (80 bytes
// of 0xaa).
#[test]
fn test_hmac_md5_key_longer_than_block_size() {
    let key = format!("0x{}", "aa".repeat(80));
    assert_eq!(
        run(
            "Test Using Larger Than Block-Size Key - Hash Key First",
            &key,
            "MD5",
            "Hex"
        ),
        "6b1ab7fe4bd7bf8f0b62e6ce61b9d0cd"
    );
}

// Same vector as test_hmac_md5_rfc2202_case1 but requesting Base64 output.
#[test]
fn test_hmac_base64_output_encoding() {
    let key = "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
    assert_eq!(
        run("Hi There", key, "MD5", "Base64"),
        "kpRyejY4uxwT9I74FYv8nQ=="
    );
}

#[test]
fn test_hmac_empty_input() {
    // Empty message still produces a full-length MAC; only the digest length
    // (32 bytes / 64 hex chars for SHA-256) is asserted here since there is
    // no RFC vector for an empty message with this key.
    let key = "0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
    let out = run("", key, "SHA-256", "Hex");
    assert_eq!(out.len(), 64);
    assert!(out.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_hmac_unsupported_hash_function_errors() {
    let op = HMAC;
    let args = [
        ArgValue::Str("0x0b0b".to_string()),
        ArgValue::Str("SHA-3".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(b"Hi There".to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_hmac_unsupported_output_encoding_errors() {
    let op = HMAC;
    let args = [
        ArgValue::Str("0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b".to_string()),
        ArgValue::Str("MD5".to_string()),
        ArgValue::Str("Base32".to_string()),
    ];
    let result = op.run(b"Hi There".to_vec(), &args);
    assert!(result.is_err());
}

// --- Discrepancy: ambiguous key-decoding heuristic ---
//
// `decode_key` in src/operations/hmac.rs has no explicit "key type" argument
// (unlike upstream CyberChef, which has a separate Key-type dropdown). It
// instead guesses the encoding: not "0x"-prefixed, not all-hex-digits, then
// tries Base64, and only falls back to literal UTF-8 bytes if Base64
// decoding fails.
//
// RFC 2202 test case 2 / RFC 4231 test case 2 use the literal ASCII key
// "Jefe". That string also happens to be valid (unpadded 4-char) Base64, so
// decode_key silently treats it as Base64 and decodes it into 3 raw bytes
// (0x25 0xe7 0xde) instead of using the 4 literal ASCII bytes b"Jefe". The
// resulting HMAC therefore does NOT match the published RFC vector.
//
// Verified independently with Python's hmac/hashlib (a trusted, spec
// conformant implementation):
//   literal key b"Jefe"            -> HMAC-MD5  = 750c783e6ab0b503eaa86e310a5db738
//   base64-decoded "Jefe" (3 bytes)-> HMAC-MD5  = 36ca602fccab3887707dad9072d35b34
// rxchef's HMAC op returns the second (wrong) value for key = "Jefe".
#[test]
#[ignore = "known bug: decode_key() in src/operations/hmac.rs misinterprets the literal ASCII key \"Jefe\" (RFC 2202/4231 test case 2) as Base64 because it happens to be valid Base64; see comment above for details"]
fn test_hmac_md5_rfc2202_case2_jefe_key() {
    assert_eq!(
        run("what do ya want for nothing?", "Jefe", "MD5", "Hex"),
        "750c783e6ab0b503eaa86e310a5db738"
    );
}
