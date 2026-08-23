//! Known-answer tests built from published standards.
//!
//! Every expected value in this file is quoted from an external specification,
//! never produced by running rx-chef and freezing the result. Each block names
//! the document it comes from so a reviewer can check it independently.
//!
//! These vectors verify conformance to the *specification*. They do not by
//! themselves establish CyberChef parity — that is the job of the differential
//! harness in `tests/tests/differential.rs`.
//!
//! Run only these tests:
//!   cargo test -p cyberchef-rust-tests --test known_answer_vectors

use rxchef::operation::ArgValue;
use rxchef::Operation;

fn text(op: &dyn Operation, input: &[u8], args: &[ArgValue]) -> String {
    String::from_utf8(
        op.run(input.to_vec(), args)
            .expect("operation must succeed"),
    )
    .expect("operation declares text output")
}

fn str_arg(value: &str) -> ArgValue {
    ArgValue::Str(value.to_string())
}

// ---------------------------------------------------------------------------
// RFC 4648 section 10 — Base16, Base32 and Base64 test vectors
// ---------------------------------------------------------------------------

/// The seven-input progression used by every RFC 4648 vector table.
const RFC4648_INPUTS: [&str; 7] = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];

#[test]
fn rfc4648_base64_encoding_vectors() {
    use rxchef::operations::to_base64::ToBase64;

    const EXPECTED: [&str; 7] = [
        "", "Zg==", "Zm8=", "Zm9v", "Zm9vYg==", "Zm9vYmE=", "Zm9vYmFy",
    ];
    for (input, expected) in RFC4648_INPUTS.iter().zip(EXPECTED) {
        assert_eq!(
            text(&ToBase64, input.as_bytes(), &[str_arg("A-Za-z0-9+/=")]),
            expected,
            "BASE64({input:?})"
        );
    }
}

#[test]
fn rfc4648_base64_decoding_vectors() {
    use rxchef::operations::from_base64::FromBase64;

    const ENCODED: [&str; 7] = [
        "", "Zg==", "Zm8=", "Zm9v", "Zm9vYg==", "Zm9vYmE=", "Zm9vYmFy",
    ];
    for (encoded, expected) in ENCODED.iter().zip(RFC4648_INPUTS) {
        let args = [
            str_arg("A-Za-z0-9+/="),
            ArgValue::Bool(true),
            ArgValue::Bool(false),
        ];
        assert_eq!(
            FromBase64.run(encoded.as_bytes().to_vec(), &args).unwrap(),
            expected.as_bytes(),
            "BASE64 decode of {encoded:?}"
        );
    }
}

#[test]
fn rfc4648_base32_encoding_vectors() {
    use rxchef::operations::to_base32::ToBase32;

    const EXPECTED: [&str; 7] = [
        "",
        "MY======",
        "MZXQ====",
        "MZXW6===",
        "MZXW6YQ=",
        "MZXW6YTB",
        "MZXW6YTBOI======",
    ];
    for (input, expected) in RFC4648_INPUTS.iter().zip(EXPECTED) {
        assert_eq!(
            text(&ToBase32, input.as_bytes(), &[str_arg("A-Z2-7=")]),
            expected,
            "BASE32({input:?})"
        );
    }
}

#[test]
fn rfc4648_base32_decoding_vectors() {
    use rxchef::operations::from_base32::FromBase32;

    const ENCODED: [&str; 7] = [
        "",
        "MY======",
        "MZXQ====",
        "MZXW6===",
        "MZXW6YQ=",
        "MZXW6YTB",
        "MZXW6YTBOI======",
    ];
    for (encoded, expected) in ENCODED.iter().zip(RFC4648_INPUTS) {
        let args = [str_arg("A-Z2-7="), ArgValue::Bool(true)];
        assert_eq!(
            FromBase32.run(encoded.as_bytes().to_vec(), &args).unwrap(),
            expected.as_bytes(),
            "BASE32 decode of {encoded:?}"
        );
    }
}

#[test]
fn rfc4648_base16_encoding_vectors() {
    use rxchef::operations::to_hex::ToHex;

    // RFC 4648 BASE16 is uppercase, unseparated hexadecimal.
    const EXPECTED: [&str; 7] = [
        "",
        "66",
        "666f",
        "666f6f",
        "666f6f62",
        "666f6f6261",
        "666f6f626172",
    ];
    for (input, expected) in RFC4648_INPUTS.iter().zip(EXPECTED) {
        let encoded = text(&ToHex, input.as_bytes(), &[str_arg("None")]);
        assert_eq!(encoded.to_ascii_lowercase(), expected, "BASE16({input:?})");
    }
}

// ---------------------------------------------------------------------------
// RFC 1321 appendix A.5 — MD5 test suite
// ---------------------------------------------------------------------------

#[test]
fn rfc1321_md5_test_suite() {
    use rxchef::operations::md5::MD5;

    const VECTORS: [(&str, &str); 7] = [
        ("", "d41d8cd98f00b204e9800998ecf8427e"),
        ("a", "0cc175b9c0f1b6a831c399e269772661"),
        ("abc", "900150983cd24fb0d6963f7d28e17f72"),
        ("message digest", "f96b697d7cb7938d525a2f31aaf161d0"),
        (
            "abcdefghijklmnopqrstuvwxyz",
            "c3fcd3d76192e4007dfb496cca67e13b",
        ),
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "d174ab98d277d9f5a5611c2c9f419d9f",
        ),
        (
            "123456789012345678901234567890123456789012345678901234567890123456789\
             01234567890",
            "57edf4a22be3c955ac49da2e2107b67a",
        ),
    ];
    for (input, expected) in VECTORS {
        assert_eq!(
            text(&MD5, input.as_bytes(), &[]).to_ascii_lowercase(),
            expected,
            "MD5({input:?})"
        );
    }
}

// ---------------------------------------------------------------------------
// FIPS 180-4 — SHA-1 and SHA-2 examples
// ---------------------------------------------------------------------------

/// The 56-character message from the FIPS 180-4 two-block examples.
const FIPS_TWO_BLOCK: &str = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";

#[test]
fn fips180_sha1_vectors() {
    use rxchef::operations::sha1::SHA1;

    const VECTORS: [(&str, &str); 3] = [
        ("", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
        ("abc", "a9993e364706816aba3e25717850c26c9cd0d89d"),
        (FIPS_TWO_BLOCK, "84983e441c3bd26ebaae4aa1f95129e5e54670f1"),
    ];
    for (input, expected) in VECTORS {
        assert_eq!(
            text(&SHA1, input.as_bytes(), &[str_arg("80")]).to_ascii_lowercase(),
            expected,
            "SHA-1({input:?})"
        );
    }
}

#[test]
fn fips180_sha2_vectors() {
    use rxchef::operations::sha2::SHA2;

    // (size, input, expected digest)
    const VECTORS: [(&str, &str, &str); 6] = [
        (
            "224",
            "abc",
            "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7",
        ),
        (
            "256",
            "",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            "256",
            "abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            "256",
            FIPS_TWO_BLOCK,
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
        (
            "384",
            "abc",
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed\
             8086072ba1e7cc2358baeca134c825a7",
        ),
        (
            "512",
            "abc",
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        ),
    ];
    for (size, input, expected) in VECTORS {
        let args = [str_arg(size), str_arg("64"), str_arg("160")];
        assert_eq!(
            text(&SHA2, input.as_bytes(), &args).to_ascii_lowercase(),
            expected.replace([' ', '\n'], ""),
            "SHA-{size}({input:?})"
        );
    }
}

// ---------------------------------------------------------------------------
// FIPS 202 — SHA-3 examples
// ---------------------------------------------------------------------------

#[test]
fn fips202_sha3_vectors() {
    use rxchef::operations::sha3::SHA3;

    const VECTORS: [(&str, &str, &str); 5] = [
        (
            "224",
            "",
            "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7",
        ),
        (
            "256",
            "",
            "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a",
        ),
        (
            "256",
            "abc",
            "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532",
        ),
        (
            "384",
            "",
            "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2a\
             c3713831264adb47fb6bd1e058d5f004",
        ),
        (
            "512",
            "",
            "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a6\
             15b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26",
        ),
    ];
    for (size, input, expected) in VECTORS {
        assert_eq!(
            text(&SHA3, input.as_bytes(), &[str_arg(size)]).to_ascii_lowercase(),
            expected.replace([' ', '\n'], ""),
            "SHA3-{size}({input:?})"
        );
    }
}

// ---------------------------------------------------------------------------
// RFC 2202 and RFC 4231 — HMAC test cases
// ---------------------------------------------------------------------------

#[test]
fn rfc2202_and_rfc4231_hmac_vectors() {
    use rxchef::operations::hmac::HMAC;

    let key_0b_16 = vec![0x0b_u8; 16];
    let key_0b_20 = vec![0x0b_u8; 20];
    let jefe = b"Jefe".to_vec();

    // (key, hash, message, expected)
    let vectors: Vec<(Vec<u8>, &str, &str, &str)> = vec![
        // RFC 2202 test case 1 and 2 for HMAC-MD5.
        (
            key_0b_16.clone(),
            "MD5",
            "Hi There",
            "9294727a3638bb1c13f48ef8158bfc9d",
        ),
        (
            jefe.clone(),
            "MD5",
            "what do ya want for nothing?",
            "750c783e6ab0b503eaa86e310a5db738",
        ),
        // RFC 2202 test case 1 and 2 for HMAC-SHA1.
        (
            key_0b_20.clone(),
            "SHA-1",
            "Hi There",
            "b617318655057264e28bc0b6fb378c8ef146be00",
        ),
        (
            jefe.clone(),
            "SHA-1",
            "what do ya want for nothing?",
            "effcdf6ae5eb2fa2d27416d5f184df9c259a7c79",
        ),
        // RFC 4231 test case 1 and 2 for HMAC-SHA256.
        (
            key_0b_20.clone(),
            "SHA-256",
            "Hi There",
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7",
        ),
        (
            jefe.clone(),
            "SHA-256",
            "what do ya want for nothing?",
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843",
        ),
        // RFC 4231 test case 1 for HMAC-SHA512.
        (
            key_0b_20,
            "SHA-512",
            "Hi There",
            "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde\
             daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854",
        ),
    ];

    for (key, hash, message, expected) in vectors {
        let args = [ArgValue::Bytes(key), str_arg(hash), str_arg("Hex")];
        assert_eq!(
            text(&HMAC, message.as_bytes(), &args).to_ascii_lowercase(),
            expected.replace([' ', '\n'], ""),
            "HMAC-{hash}({message:?})"
        );
    }
}

// ---------------------------------------------------------------------------
// RFC 7693 appendix A / BLAKE2 reference vectors
// ---------------------------------------------------------------------------

#[test]
fn rfc7693_blake2_vectors() {
    use rxchef::operations::blake2b::BLAKE2b;
    use rxchef::operations::blake2s::BLAKE2s;

    let args_b = [str_arg("512"), str_arg("Hex"), ArgValue::Bytes(Vec::new())];
    assert_eq!(
        text(&BLAKE2b, b"abc", &args_b).to_ascii_lowercase(),
        "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d1\
         7d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923"
            .replace([' ', '\n'], ""),
        "BLAKE2b-512(\"abc\")"
    );

    let args_s = [str_arg("256"), str_arg("Hex"), ArgValue::Bytes(Vec::new())];
    assert_eq!(
        text(&BLAKE2s, b"abc", &args_s).to_ascii_lowercase(),
        "508c5e8c327c14e2e1a72ba34eeb452f37458b209ed63a294d999b4c86675982",
        "BLAKE2s-256(\"abc\")"
    );
}

// ---------------------------------------------------------------------------
// Checksums — the standard "123456789" check value
// ---------------------------------------------------------------------------

#[test]
fn checksum_check_values_for_123456789() {
    use rxchef::operations::adler32_checksum::Adler32Checksum;

    // Adler-32 is defined in RFC 1950. For "123456789":
    // A = 1 + sum(bytes) = 478 = 0x01DE, B = sum of running A = 2334 = 0x091E.
    assert_eq!(
        text(&Adler32Checksum, b"123456789", &[]).to_ascii_lowercase(),
        "091e01de",
        "Adler-32(\"123456789\")"
    );
}

// ---------------------------------------------------------------------------
// NIST SP 800-38A appendix F — AES-128 mode examples
// ---------------------------------------------------------------------------

/// The AES-128 key used throughout NIST SP 800-38A appendix F.
const NIST_AES128_KEY: &str = "2b7e151628aed2a6abf7158809cf4f3c";
/// The first plaintext block of the SP 800-38A examples.
const NIST_BLOCK1_PLAINTEXT: &str = "6bc1bee22e409f96e93d7e117393172a";
/// The initialisation vector used by the CBC, CFB and OFB examples.
const NIST_IV: &str = "000102030405060708090a0b0c0d0e0f";

fn aes_encrypt_hex(key: &str, iv: &str, mode: &str, plaintext_hex: &str) -> String {
    use rxchef::operations::aes_encrypt::AesEncrypt;

    let args = [
        ArgValue::Bytes(hex::decode(key).unwrap()),
        ArgValue::Bytes(hex::decode(iv).unwrap()),
        str_arg(mode),
        str_arg("Hex"),
        str_arg("Hex"),
        ArgValue::Bytes(Vec::new()),
    ];
    text(&AesEncrypt, plaintext_hex.as_bytes(), &args).to_ascii_lowercase()
}

#[test]
fn nist_sp800_38a_aes128_ecb_first_block() {
    // F.1.1 ECB-AES128.Encrypt, block #1.
    let output = aes_encrypt_hex(NIST_AES128_KEY, "", "ECB", NIST_BLOCK1_PLAINTEXT);
    assert!(
        output.starts_with("3ad77bb40d7a3660a89ecaf32466ef97"),
        "ECB-AES128 block 1 mismatch: {output}"
    );
}

#[test]
fn nist_sp800_38a_aes128_cbc_first_block() {
    // F.2.1 CBC-AES128.Encrypt, block #1.
    let output = aes_encrypt_hex(NIST_AES128_KEY, NIST_IV, "CBC", NIST_BLOCK1_PLAINTEXT);
    assert!(
        output.starts_with("7649abac8119b246cee98e9b12e9197d"),
        "CBC-AES128 block 1 mismatch: {output}"
    );
}

#[test]
fn nist_sp800_38a_aes128_ctr_first_block() {
    // F.5.1 CTR-AES128.Encrypt, block #1, initial counter f0f1..ff.
    let output = aes_encrypt_hex(
        NIST_AES128_KEY,
        "f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
        "CTR",
        NIST_BLOCK1_PLAINTEXT,
    );
    assert!(
        output.starts_with("874d6191b620e3261bef6864990db6ce"),
        "CTR-AES128 block 1 mismatch: {output}"
    );
}

#[test]
fn nist_sp800_38a_aes128_cfb_first_block() {
    // F.3.13 CFB128-AES128.Encrypt, segment #1.
    let output = aes_encrypt_hex(NIST_AES128_KEY, NIST_IV, "CFB", NIST_BLOCK1_PLAINTEXT);
    assert!(
        output.starts_with("3b3fd92eb72dad20333449f8e83cfb4a"),
        "CFB128-AES128 segment 1 mismatch: {output}"
    );
}

#[test]
fn nist_sp800_38a_aes128_ofb_first_block() {
    // F.4.1 OFB-AES128.Encrypt, block #1. The first output block matches CFB
    // because both XOR the plaintext with E(K, IV).
    let output = aes_encrypt_hex(NIST_AES128_KEY, NIST_IV, "OFB", NIST_BLOCK1_PLAINTEXT);
    assert!(
        output.starts_with("3b3fd92eb72dad20333449f8e83cfb4a"),
        "OFB-AES128 block 1 mismatch: {output}"
    );
}

fn aes_decrypt_hex(key: &str, iv: &str, mode: &str, ciphertext_hex: &str) -> String {
    use rxchef::operations::aes_decrypt::AesDecrypt;

    let args = [
        ArgValue::Bytes(hex::decode(key).unwrap()),
        ArgValue::Bytes(hex::decode(iv).unwrap()),
        str_arg(mode),
        str_arg("Hex"),
        str_arg("Hex"),
        ArgValue::Bytes(Vec::new()),
        ArgValue::Bytes(Vec::new()),
    ];
    text(&AesDecrypt, ciphertext_hex.as_bytes(), &args).to_ascii_lowercase()
}

#[test]
fn nist_sp800_38a_aes128_ecb_decrypt_first_block() {
    // NIST SP 800-38A F.1.2 ECB-AES128.Decrypt, block #1. NoPadding is
    // required because the published example is exactly one AES block.
    assert_eq!(
        aes_decrypt_hex(
            NIST_AES128_KEY,
            "",
            "ECB/NoPadding",
            "3ad77bb40d7a3660a89ecaf32466ef97",
        ),
        NIST_BLOCK1_PLAINTEXT
    );
}

// ---------------------------------------------------------------------------
// NIST CAVP DES ECB Variable Plaintext Known Answer Test
// ---------------------------------------------------------------------------

const NIST_DES_KEY: &str = "0101010101010101";
const NIST_DES_PLAINTEXT: &str = "8000000000000000";
const NIST_DES_CIPHERTEXT: &str = "95f8a5e5dd31d900";

#[test]
fn nist_cavp_des_ecb_encrypt_first_block() {
    use rxchef::operations::des_encrypt::DesEncrypt;

    let args = [
        ArgValue::Bytes(hex::decode(NIST_DES_KEY).unwrap()),
        ArgValue::Bytes(Vec::new()),
        str_arg("ECB"),
        str_arg("Hex"),
        str_arg("Hex"),
    ];
    // DES Encrypt applies PKCS#7, so it emits a second block. The first block
    // is still the exact CAVP single-block result and is checked independently.
    let output = text(&DesEncrypt, NIST_DES_PLAINTEXT.as_bytes(), &args);
    assert_eq!(&output[..16], NIST_DES_CIPHERTEXT);
}

#[test]
fn nist_cavp_des_ecb_decrypt_single_block() {
    use rxchef::operations::des_decrypt::DesDecrypt;

    let args = [
        ArgValue::Bytes(hex::decode(NIST_DES_KEY).unwrap()),
        ArgValue::Bytes(Vec::new()),
        str_arg("ECB/NoPadding"),
        str_arg("Hex"),
        str_arg("Hex"),
    ];
    assert_eq!(
        text(&DesDecrypt, NIST_DES_CIPHERTEXT.as_bytes(), &args),
        NIST_DES_PLAINTEXT
    );
}

// ---------------------------------------------------------------------------
// RFC 8439 section 2.3.2 — ChaCha20 block-function test vector
// ---------------------------------------------------------------------------

#[test]
fn rfc8439_chacha20_block_function_vector() {
    use rxchef::operations::chacha::ChaCha;

    let args = [
        str_arg("0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"),
        str_arg("0x000000090000004a00000000"),
        ArgValue::Num(1.0),
        str_arg("20"),
        str_arg("Hex"),
        str_arg("Hex"),
    ];
    let zero_block = "00".repeat(64);
    assert_eq!(
        text(&ChaCha, zero_block.as_bytes(), &args),
        "10f1e7e4d13b5915500fdd1fa32071c4c7d1f4c733c068030422aa9ac3d46c4e\
         d2826446079faa0914c2d705d98b02a2b5129cd1de164eb9cbd083e8a2503c4e"
            .replace([' ', '\n'], "")
    );
}

// ---------------------------------------------------------------------------
// BLAKE3 official test vectors (test_vectors.json, input length 0)
// ---------------------------------------------------------------------------

#[test]
fn blake3_official_empty_input_vector() {
    use rxchef::operations::blake3::BLAKE3;

    assert_eq!(
        text(&BLAKE3, b"", &[]),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );
}

// ---------------------------------------------------------------------------
// Haversine formula — quarter of a great circle at the mean Earth radius
// ---------------------------------------------------------------------------

#[test]
fn haversine_equatorial_quarter_circle() {
    use rxchef::operations::haversine_distance::HaversineDistance;

    // The operation documents the conventional mean Earth radius 6,371,000 m.
    // Two equatorial points 90 degrees apart are therefore pi*R/2 metres apart.
    // The literal below was evaluated independently at high precision; the
    // tolerance only accommodates binary floating-point rounding.
    let actual: f64 = text(&HaversineDistance, b"0,0,0,90", &[]).parse().unwrap();
    assert!((actual - 10_007_543.398_010_286).abs() < 1e-8);
}

#[test]
fn nist_international_mile_speed_conversion() {
    use rxchef::operations::convert_speed::ConvertSpeed;

    // NIST Handbook 44 Appendix C defines one international mile as exactly
    // 1.609344 km. Therefore 60 mph is exactly 96.56064 km/h.
    let args = [
        str_arg("Miles per hour (mph)"),
        str_arg("Kilometres per hour (km/h)"),
    ];
    let actual: f64 = text(&ConvertSpeed, b"60", &args).parse().unwrap();
    assert!((actual - 96.56064).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// Argon2 reference implementation / draft-irtf-cfrg-argon2-12 KAT
// ---------------------------------------------------------------------------

#[test]
fn argon2i_v13_reference_vector() {
    use rxchef::operations::argon2::Argon2;

    // Argon2 reference KAT: password="password", salt="somesalt", v=19,
    // m=65536 KiB, t=2, p=1, 32-byte raw tag.
    let args = [
        str_arg("somesalt"),
        ArgValue::Num(2.0),
        ArgValue::Num(65_536.0),
        ArgValue::Num(1.0),
        ArgValue::Num(32.0),
        str_arg("Argon2i"),
        str_arg("Hex hash"),
    ];
    assert_eq!(
        text(&Argon2, b"password", &args),
        "c1628832147d9720c5bd1cfd61367078729f6dfb6f8fea9ff98158e0d7816ed0"
    );
}
