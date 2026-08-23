use rxchef::{operation::ArgValue, operations::gost_sign::GostSign, Operation};

fn key32() -> Vec<u8> {
    hex::decode("8899aabbccddeeff0011223344556677fedcba98765432100123456789abcdef").unwrap()
}

fn run_sign(
    algorithm: &str,
    key: &[u8],
    iv: &[u8],
    input: &[u8],
    mac_length_bits: &str,
) -> Vec<u8> {
    GostSign
        .run(
            input.to_vec(),
            &[
                ArgValue::Bytes(key.to_vec()),
                ArgValue::Bytes(iv.to_vec()),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("Hex".into()),
                ArgValue::Str(algorithm.into()),
                ArgValue::Str("E-Z".into()),
                ArgValue::Str(mac_length_bits.into()),
            ],
        )
        .unwrap()
}

// These vectors were produced by directly invoking `processMAC15` /
// `signMAC` in the vendored reference implementation used by CyberChef
// (gchq/CyberChef, src/core/vendor/gost/gostCipher.mjs, as of commit
// b92501ee354256a127479f93d4c31a4f1d0dd657, fetched 2026-08-23), which is
// what CyberChef's "GOST Sign" operation actually calls. They cover the
// empty message, a partial (non-block-aligned) message, an exact multiple
// of the block size spanning several blocks, and a non-default IV — not
// just a single full block, which (for this MAC construction) would be
// indistinguishable from a raw block-cipher KAT and would not actually
// exercise the MAC chaining/padding logic.
#[test]
fn test_gost_sign_matches_cyberchef_reference_vectors_magma() {
    let key = key32();
    let cases: &[(&str, &[u8], &str)] = &[
        ("single-block exact (8 bytes)", b"hello!!!", "2ac78909"),
        ("empty input", b"", "00000000"),
        ("partial block (2 bytes)", b"hi", "34e4d07a"),
        (
            "multiblock exact (32 bytes)",
            b"0123456789abcdefghijklmnopqrstu",
            "8059390d",
        ),
    ];
    for (label, input, expected) in cases {
        let result = run_sign("GOST R 34.12 (Magma, 2015)", &key, &[], input, "32");
        assert_eq!(result, expected.as_bytes(), "{label}");
    }
}

#[test]
fn test_gost_sign_matches_cyberchef_reference_vector_nonzero_iv() {
    let key = key32();
    let iv = hex::decode("1122334455667788").unwrap();
    let result = run_sign("GOST R 34.12 (Magma, 2015)", &key, &iv, b"hello!!!", "32");
    assert_eq!(result, b"daf3b103");
}

#[test]
fn test_gost_sign_matches_cyberchef_reference_vectors_kuznyechik() {
    let key = key32();
    let cases: &[(&str, &[u8], &str)] = &[
        (
            "single-block exact (16 bytes)",
            b"hello world!!!!!"[..16].as_ref(),
            "49e7c5186b022387",
        ),
        ("empty input", b"", "0000000000000000"),
        ("partial block (5 bytes)", b"hello", "7ab3961cfac88f5f"),
        (
            "multiblock exact (32 bytes)",
            &b"0123456789abcdef0123456789abcdef"[..32],
            "cea56d4c4574c273",
        ),
    ];
    for (label, input, expected) in cases {
        let result = run_sign("GOST R 34.12 (Kuznyechik, 2015)", &key, &[], input, "64");
        assert_eq!(result, expected.as_bytes(), "{label}");
    }
}

#[test]
fn test_gost_sign_1989_alias_matches_2015_magma() {
    // This crate implements "GOST 28147 (1989)" as an alias for
    // GOST R 34.12 (Magma, 2015) (see gost_mac module docs for why), so the
    // two algorithm selections must produce identical output.
    let key = key32();
    let a = run_sign("GOST 28147 (1989)", &key, &[], b"hello!!!", "32");
    let b = run_sign("GOST R 34.12 (Magma, 2015)", &key, &[], b"hello!!!", "32");
    assert_eq!(a, b);
}

#[test]
fn test_gost_sign_rejects_wrong_length_iv() {
    let key = key32();
    let err = GostSign
        .run(
            b"hello!!!".to_vec(),
            &[
                ArgValue::Bytes(key),
                ArgValue::Bytes(vec![0u8; 3]),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("Hex".into()),
                ArgValue::Str("GOST R 34.12 (Magma, 2015)".into()),
                ArgValue::Str("E-Z".into()),
                ArgValue::Str("32".into()),
            ],
        )
        .unwrap_err();
    assert!(format!("{err}").to_lowercase().contains("iv"));
}
