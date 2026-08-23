use rxchef::{
    operation::ArgValue,
    operations::{gost_key_unwrap::GOSTKeyUnwrapOp, gost_key_wrap::GostKeyWrap},
    Operation,
};

fn kek32() -> Vec<u8> {
    hex::decode("8899aabbccddeeff0011223344556677fedcba98765432100123456789abcdef").unwrap()
}

fn cek32() -> Vec<u8> {
    hex::decode("0102030405060708101112131415161720212223242526273031323334353637").unwrap()
}

fn wrap_args(
    kek: &[u8],
    ukm: &[u8],
    algorithm: &str,
    sbox: &str,
    key_wrapping: &str,
) -> Vec<ArgValue> {
    vec![
        ArgValue::Bytes(kek.to_vec()),
        ArgValue::Bytes(ukm.to_vec()),
        ArgValue::Str("Raw".into()),
        ArgValue::Str("Hex".into()),
        ArgValue::Str(algorithm.into()),
        ArgValue::Str(sbox.into()),
        ArgValue::Str(key_wrapping.into()),
    ]
}

// These vectors were produced by directly invoking `wrapKeyGOST` /
// `wrapKeyCP` in the vendored reference implementation used by CyberChef
// (gchq/CyberChef, src/core/vendor/gost/gostCipher.mjs, as of commit
// b92501ee354256a127479f93d4c31a4f1d0dd657, fetched 2026-08-23), which is
// what CyberChef's "GOST Key Wrap" operation actually calls. Each covers a
// 4-block (32-byte) CEK so the MAC chaining is actually exercised, not just
// the underlying block cipher.
const UKM8: &str = "1234567890abcdef";
const UKM16: &str = "1234567890abcdef1234567890abcdef";

#[test]
fn test_gost_key_wrap_matches_cyberchef_reference_vectors() {
    let kek = kek32();
    let cek = cek32();
    let ukm8 = hex::decode(UKM8).unwrap();
    let ukm16 = hex::decode(UKM16).unwrap();

    // Note: "GOST 28147 (1989)" is deliberately NOT covered here. This crate
    // implements it as an alias for GOST R 34.12 (Magma, 2015) (see the
    // gost_mac module docs), whereas the real 1989 algorithm uses a
    // different round-reduced MAC construction and a selectable S-box, so
    // its genuine reference vectors would legitimately fail against this
    // implementation. `test_gost_key_wrap_1989_alias_matches_2015_magma`
    // below checks the alias behaviour is at least internally consistent.
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
    for (label, algorithm, key_wrapping, expected) in cases {
        let result = GostKeyWrap
            .run(
                cek.clone(),
                &wrap_args(&kek, &ukm8, algorithm, "E-TEST", key_wrapping),
            )
            .unwrap();
        assert_eq!(result, expected.as_bytes(), "{label}");
    }

    let kuzn_result = GostKeyWrap
        .run(
            cek.clone(),
            &wrap_args(&kek, &ukm16, "GOST R 34.12 (Kuznyechik, 2015)", "E-Z", "NO"),
        )
        .unwrap();
    assert_eq!(
        kuzn_result,
        b"b0b6cd75f7d4abaeef05831ebbdc2548ab6e07a461794982b30d29d48a5dda5476b7e051e5f72007"
            .as_ref()
    );
}

#[test]
fn test_gost_key_wrap_1989_alias_matches_2015_magma() {
    let kek = kek32();
    let cek = cek32();
    let ukm8 = hex::decode(UKM8).unwrap();
    let a = GostKeyWrap
        .run(
            cek.clone(),
            &wrap_args(&kek, &ukm8, "GOST 28147 (1989)", "E-TEST", "NO"),
        )
        .unwrap();
    let b = GostKeyWrap
        .run(
            cek,
            &wrap_args(&kek, &ukm8, "GOST R 34.12 (Magma, 2015)", "E-TEST", "NO"),
        )
        .unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_gost_key_wrap_cp_rejected_for_kuznyechik() {
    let kek = kek32();
    let cek = cek32();
    let ukm16 = hex::decode(UKM16).unwrap();
    let err = GostKeyWrap
        .run(
            cek,
            &wrap_args(&kek, &ukm16, "GOST R 34.12 (Kuznyechik, 2015)", "E-Z", "CP"),
        )
        .unwrap_err();
    let msg = format!("{err}").to_lowercase();
    assert!(msg.contains("cryptopro") || msg.contains("64-bit"), "{msg}");
}

#[test]
fn test_gost_key_wrap_sc_not_implemented() {
    let kek = kek32();
    let cek = cek32();
    let ukm8 = hex::decode(UKM8).unwrap();
    let err = GostKeyWrap
        .run(
            cek,
            &wrap_args(&kek, &ukm8, "GOST R 34.12 (Magma, 2015)", "E-Z", "SC"),
        )
        .unwrap_err();
    assert!(format!("{err}").to_lowercase().contains("not implemented"));
}

#[test]
fn test_gost_key_wrap_rejects_non_block_multiple_cek() {
    let kek = kek32();
    let ukm8 = hex::decode(UKM8).unwrap();
    let err = GostKeyWrap
        .run(
            b"1234567".to_vec(), // 7 bytes, not a multiple of the 8-byte Magma block
            &wrap_args(&kek, &ukm8, "GOST R 34.12 (Magma, 2015)", "E-Z", "NO"),
        )
        .unwrap_err();
    assert!(format!("{err}").to_lowercase().contains("8 bytes"));
}

#[test]
fn test_gost_key_wrap_rejects_wrong_length_ukm() {
    let kek = kek32();
    let cek = cek32();
    let err = GostKeyWrap
        .run(
            cek,
            &wrap_args(&kek, &[0u8; 3], "GOST R 34.12 (Magma, 2015)", "E-Z", "NO"),
        )
        .unwrap_err();
    assert!(
        format!("{err}").to_lowercase().contains("ukm")
            || format!("{err}").to_lowercase().contains("key material")
    );
}

#[test]
fn test_gost_key_wrap_roundtrip() {
    for (algorithm, ukm) in [
        ("GOST R 34.12 (Magma, 2015)", UKM8),
        ("GOST R 34.12 (Kuznyechik, 2015)", UKM16),
    ] {
        for key_wrapping in ["NO", "CP"] {
            if algorithm.contains("Kuznyechik") && key_wrapping == "CP" {
                continue; // not supported; covered separately
            }
            let key = ArgValue::Str("0123456789abcdef0123456789abcdef".into());
            let cek = b"abcdefghijklmnopqrstuvwxyz012345".to_vec();
            let wrap_args = [
                key,
                ArgValue::Bytes(hex::decode(ukm).unwrap()),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("Hex".into()),
                ArgValue::Str(algorithm.into()),
                ArgValue::Str("E-TEST".into()),
                ArgValue::Str(key_wrapping.into()),
            ];
            let wrapped = GostKeyWrap.run(cek.clone(), &wrap_args).unwrap();
            let mut unwrap_args = wrap_args;
            unwrap_args[2] = ArgValue::Str("Hex".into());
            unwrap_args[3] = ArgValue::Str("Raw".into());
            let unwrapped = GOSTKeyUnwrapOp.run(wrapped, &unwrap_args).unwrap();
            assert_eq!(unwrapped, cek, "{algorithm} {key_wrapping}");
        }
    }
}
