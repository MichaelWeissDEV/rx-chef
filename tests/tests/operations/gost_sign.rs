use rxchef::{operation::ArgValue, operations::gost_sign::GostSign, Operation};

#[test]
fn test_gost_sign_matches_published_gost_r_3412_block_vectors() {
    let vectors = [
        (
            "GOST R 34.12 (Magma, 2015)",
            "ffeeddccbbaa99887766554433221100f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
            "fedcba9876543210",
            "4ee901e5c2d8ca3d",
            "64",
        ),
        (
            "GOST R 34.12 (Kuznyechik, 2015)",
            "8899aabbccddeeff0011223344556677fedcba98765432100123456789abcdef",
            "1122334455667700ffeeddccbbaa9988",
            "7f679d90bebc24305a468d42b9d4edcd",
            "128",
        ),
    ];
    for (algorithm, key, plaintext, expected, bits) in vectors {
        let result = GostSign
            .run(
                hex::decode(plaintext).unwrap(),
                &[
                    ArgValue::Bytes(hex::decode(key).unwrap()),
                    ArgValue::Bytes(Vec::new()),
                    ArgValue::Str("Raw".into()),
                    ArgValue::Str("Hex".into()),
                    ArgValue::Str(algorithm.into()),
                    ArgValue::Str("E-Z".into()),
                    ArgValue::Str(bits.into()),
                ],
            )
            .unwrap();
        assert_eq!(result, expected.as_bytes(), "{algorithm}");
    }
}
