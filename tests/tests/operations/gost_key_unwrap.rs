use rxchef::operation::ArgValue;
use rxchef::operations::gost_key_unwrap::GOSTKeyUnwrapOp;
use rxchef::Operation;

fn kek32() -> Vec<u8> {
    hex::decode("8899aabbccddeeff0011223344556677fedcba98765432100123456789abcdef").unwrap()
}

fn cek32() -> Vec<u8> {
    hex::decode("0102030405060708101112131415161720212223242526273031323334353637").unwrap()
}

const UKM8: &str = "1234567890abcdef";
const UKM16: &str = "1234567890abcdef1234567890abcdef";

fn unwrap_args(
    kek: &[u8],
    ukm: &[u8],
    algorithm: &str,
    sbox: &str,
    key_wrapping: &str,
) -> Vec<ArgValue> {
    vec![
        ArgValue::Bytes(kek.to_vec()),
        ArgValue::Bytes(ukm.to_vec()),
        ArgValue::Str("Hex".into()),
        ArgValue::Str("Raw".into()),
        ArgValue::Str(algorithm.into()),
        ArgValue::Str(sbox.into()),
        ArgValue::Str(key_wrapping.into()),
    ]
}

// See gost_key_wrap.rs for the reference-implementation source these
// wrapped-key vectors were produced from.
#[test]
fn test_gost_key_unwrap_matches_cyberchef_reference_vectors() {
    let kek = kek32();
    let cek = cek32();
    let ukm8 = hex::decode(UKM8).unwrap();
    let ukm16 = hex::decode(UKM16).unwrap();

    // Note: "GOST 28147 (1989)" is deliberately NOT covered here; see
    // gost_key_wrap.rs for why.
    let cases: &[(&str, &str, &str, &str)] = &[
        (
            "2015 Magma NO",
            "GOST R 34.12 (Magma, 2015)",
            "NO",
            "dd5ca562bbe91e25000081b232e945f25c6e3185e543494098d5d6fb150c81465e2be609",
        ),
        (
            "2015 Magma CP",
            "GOST R 34.12 (Magma, 2015)",
            "CP",
            "784173977ed70005a7c374f30a2ab90da730459ef24c347bd7a44020e723376d689640b2",
        ),
    ];
    for (label, algorithm, key_wrapping, wrapped_hex) in cases {
        let result = GOSTKeyUnwrapOp
            .run(
                wrapped_hex.as_bytes().to_vec(),
                &unwrap_args(&kek, &ukm8, algorithm, "E-TEST", key_wrapping),
            )
            .unwrap();
        assert_eq!(result, cek, "{label}");
    }

    let kuzn = GOSTKeyUnwrapOp
        .run(
            b"b0b6cd75f7d4abaeef05831ebbdc2548ab6e07a461794982b30d29d48a5dda5476b7e051e5f72007"
                .to_vec(),
            &unwrap_args(&kek, &ukm16, "GOST R 34.12 (Kuznyechik, 2015)", "E-Z", "NO"),
        )
        .unwrap();
    assert_eq!(kuzn, cek);
}

#[test]
fn test_gost_key_unwrap_rejects_tampered_mac() {
    let kek = kek32();
    let ukm8 = hex::decode(UKM8).unwrap();
    let mut wrapped =
        hex::decode("dd5ca562bbe91e25000081b232e945f25c6e3185e543494098d5d6fb150c81465e2be609")
            .unwrap();
    *wrapped.last_mut().unwrap() ^= 0xFF;
    let err = GOSTKeyUnwrapOp
        .run(
            hex::encode(wrapped).into_bytes(),
            &unwrap_args(&kek, &ukm8, "GOST R 34.12 (Magma, 2015)", "E-Z", "NO"),
        )
        .unwrap_err();
    assert!(format!("{err}").to_lowercase().contains("mac"));
}

#[test]
fn test_gost_key_unwrap_rejects_tampered_ciphertext() {
    let kek = kek32();
    let ukm8 = hex::decode(UKM8).unwrap();
    let mut wrapped =
        hex::decode("dd5ca562bbe91e25000081b232e945f25c6e3185e543494098d5d6fb150c81465e2be609")
            .unwrap();
    wrapped[0] ^= 0xFF;
    let err = GOSTKeyUnwrapOp
        .run(
            hex::encode(wrapped).into_bytes(),
            &unwrap_args(&kek, &ukm8, "GOST R 34.12 (Magma, 2015)", "E-Z", "NO"),
        )
        .unwrap_err();
    assert!(format!("{err}").to_lowercase().contains("mac"));
}

#[test]
fn test_gost_key_unwrap_rejects_wrong_kek() {
    let mut kek = kek32();
    kek[0] ^= 0xFF;
    let ukm8 = hex::decode(UKM8).unwrap();
    let err = GOSTKeyUnwrapOp
        .run(
            b"dd5ca562bbe91e25000081b232e945f25c6e3185e543494098d5d6fb150c81465e2be609".to_vec(),
            &unwrap_args(&kek, &ukm8, "GOST R 34.12 (Magma, 2015)", "E-Z", "NO"),
        )
        .unwrap_err();
    assert!(format!("{err}").to_lowercase().contains("mac"));
}

#[test]
fn test_gost_key_unwrap_cp_rejected_for_kuznyechik() {
    let kek = kek32();
    let ukm16 = hex::decode(UKM16).unwrap();
    let err = GOSTKeyUnwrapOp
        .run(
            b"b0b6cd75f7d4abaeef05831ebbdc2548ab6e07a461794982b30d29d48a5dda5476b7e051e5f72007"
                .to_vec(),
            &unwrap_args(&kek, &ukm16, "GOST R 34.12 (Kuznyechik, 2015)", "E-Z", "CP"),
        )
        .unwrap_err();
    let msg = format!("{err}").to_lowercase();
    assert!(msg.contains("cryptopro") || msg.contains("64-bit"), "{msg}");
}
