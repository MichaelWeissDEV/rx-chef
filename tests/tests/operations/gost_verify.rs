use rxchef::operation::ArgValue;
use rxchef::operations::{gost_sign::GostSign, gost_verify::GOSTVerifyOp};
use rxchef::Operation;

#[test]
fn test_gost_verify_accepts_valid_and_rejects_invalid_mac() {
    let key = ArgValue::Str("0123456789abcdef0123456789abcdef".into());
    let iv = ArgValue::Str("12345678".into());
    let message = b"authenticated message".to_vec();
    let mac = GostSign
        .run(
            message.clone(),
            &[
                key.clone(),
                iv.clone(),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("Hex".into()),
                ArgValue::Str("GOST R 34.12 (Magma, 2015)".into()),
                ArgValue::Str("E-TEST".into()),
                ArgValue::Str("32".into()),
            ],
        )
        .unwrap();
    let mut args = vec![
        key,
        iv,
        ArgValue::Str(String::from_utf8(mac).unwrap()),
        ArgValue::Str("Raw".into()),
        ArgValue::Str("GOST R 34.12 (Magma, 2015)".into()),
        ArgValue::Str("E-TEST".into()),
    ];
    assert_eq!(GOSTVerifyOp.run(message.clone(), &args).unwrap(), b"true");
    args[2] = ArgValue::Str("00000000".into());
    assert_eq!(GOSTVerifyOp.run(message, &args).unwrap(), b"false");
}

// This vector was produced by directly invoking `processMAC15` in the
// vendored reference implementation used by CyberChef (gchq/CyberChef,
// src/core/vendor/gost/gostCipher.mjs, as of commit
// b92501ee354256a127479f93d4c31a4f1d0dd657, fetched 2026-08-23). It uses a
// 5-byte (non-block-aligned) message specifically because a single exact
// block would make this MAC construction indistinguishable from a raw
// block-cipher known-answer test and would not exercise the padding logic
// that GOST Verify actually depends on.
#[test]
fn test_gost_verify_accepts_cyberchef_reference_vector_kuznyechik() {
    let result = GOSTVerifyOp
        .run(
            b"hello".to_vec(),
            &[
                ArgValue::Bytes(
                    hex::decode("8899aabbccddeeff0011223344556677fedcba98765432100123456789abcdef")
                        .unwrap(),
                ),
                ArgValue::Bytes(Vec::new()),
                ArgValue::Str("7ab3961cfac88f5f".into()),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("GOST R 34.12 (Kuznyechik, 2015)".into()),
                ArgValue::Str("E-Z".into()),
            ],
        )
        .unwrap();
    assert_eq!(result, b"true");
}

#[test]
fn test_gost_verify_rejects_tampered_mac() {
    let result = GOSTVerifyOp
        .run(
            b"hello".to_vec(),
            &[
                ArgValue::Bytes(
                    hex::decode("8899aabbccddeeff0011223344556677fedcba98765432100123456789abcdef")
                        .unwrap(),
                ),
                ArgValue::Bytes(Vec::new()),
                // last byte flipped relative to the correct MAC
                ArgValue::Str("7ab3961cfac88fa0".into()),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("GOST R 34.12 (Kuznyechik, 2015)".into()),
                ArgValue::Str("E-Z".into()),
            ],
        )
        .unwrap();
    assert_eq!(result, b"false");
}
