// Tests for the cmac operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cmac::
//
// Known-answer vectors from RFC 4493, "The AES-CMAC Algorithm", section 4
// ("Test Vectors"), which reproduces the NIST SP 800-38B examples. All four
// use the AES-128 key 2b7e151628aed2a6abf7158809cf4f3c and cover the three
// distinct paths through the algorithm: the empty message, an exact block
// multiple, and a message needing padding.

use rxchef::runtime::{self, RuntimeError};

const RFC4493_KEY: &str = "hex:2b7e151628aed2a6abf7158809cf4f3c";

fn cmac(message_hex: &str, key: &str) -> Result<String, RuntimeError> {
    let message = if message_hex.is_empty() {
        Vec::new()
    } else {
        hex::decode(message_hex).expect("test vector must be valid hex")
    };
    runtime::run_operation("CMAC", message, &[key.to_string(), "AES".to_string()])
        .map(|out| String::from_utf8_lossy(&out).into_owned())
}

#[test]
fn test_cmac_rfc_4493_vectors() {
    // (RFC 4493 example, message, expected MAC)
    for (example, message, expected) in [
        ("1: len 0", "", "bb1d6929e95937287fa37d129b756746"),
        (
            "2: len 16",
            "6bc1bee22e409f96e93d7e117393172a",
            "070a16b46b4d4144f79bdd9dd04a287c",
        ),
        (
            "3: len 40",
            "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411",
            "dfa66747de9ae63030ca32611497c827",
        ),
        (
            "4: len 64",
            "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411\
             e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710",
            "51f0bebf7e3b9d92fc49741779363cfe",
        ),
    ] {
        assert_eq!(
            cmac(&message.replace(['\n', ' '], ""), RFC4493_KEY)
                .expect("a published vector must succeed"),
            expected,
            "RFC 4493 example {example} failed"
        );
    }
}

#[test]
fn test_cmac_empty_message_is_a_defined_case() {
    // RFC 4493 example 1: the empty message has a MAC, it is not an error.
    assert_eq!(
        cmac("", RFC4493_KEY).unwrap(),
        "bb1d6929e95937287fa37d129b756746"
    );
}

#[test]
fn test_cmac_output_is_one_aes_block() {
    // AES-CMAC produces a full 128-bit tag.
    assert_eq!(cmac("", RFC4493_KEY).unwrap().len(), 32);
    assert_eq!(
        cmac("6bc1bee22e409f96e93d7e117393172a", RFC4493_KEY)
            .unwrap()
            .len(),
        32
    );
}

#[test]
fn test_cmac_block_boundary_messages_differ() {
    // 15, 16 and 17 bytes take different padding paths through the algorithm.
    let fifteen = cmac(&"ab".repeat(15), RFC4493_KEY).unwrap();
    let sixteen = cmac(&"ab".repeat(16), RFC4493_KEY).unwrap();
    let seventeen = cmac(&"ab".repeat(17), RFC4493_KEY).unwrap();
    assert_ne!(fifteen, sixteen);
    assert_ne!(sixteen, seventeen);
}

#[test]
fn test_cmac_depends_on_the_key() {
    let a = cmac("6bc1bee22e409f96e93d7e117393172a", RFC4493_KEY).unwrap();
    let b = cmac(
        "6bc1bee22e409f96e93d7e117393172a",
        "hex:00000000000000000000000000000000",
    )
    .unwrap();
    assert_ne!(a, b, "the MAC must depend on the key");
}

#[test]
fn test_cmac_rejects_an_invalid_aes_key_length() {
    // AES keys are 128, 192 or 256 bits.
    for bad in [
        "hex:00",
        "hex:0011223344556677",
        "hex:",
        &format!("hex:{}", "ab".repeat(20)),
    ] {
        assert!(
            cmac("6bc1bee22e409f96e93d7e117393172a", bad).is_err(),
            "key {bad} must be rejected"
        );
    }
}

#[test]
fn test_cmac_rejects_an_unknown_algorithm() {
    let result = runtime::run_operation(
        "CMAC",
        b"message".to_vec(),
        &[RFC4493_KEY.to_string(), "NotACipher".to_string()],
    );
    assert!(result.is_err(), "an unknown cipher must be rejected");
}

#[test]
fn test_cmac_is_deterministic() {
    assert_eq!(
        cmac("6bc1bee22e409f96e93d7e117393172a", RFC4493_KEY).unwrap(),
        cmac("6bc1bee22e409f96e93d7e117393172a", RFC4493_KEY).unwrap()
    );
}
