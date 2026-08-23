use rxchef::{operation::ArgValue, operations::gost_key_wrap::GostKeyWrap, Operation};

#[test]
fn test_gost_key_wrap_kuznyechik_published_block_answer() {
    // Both the ECB-wrapped block and the first CBC-MAC block reduce to the
    // published GOST R 34.12-2015 Kuznyechik block-cipher answer.
    let result = GostKeyWrap
        .run(
            hex::decode("1122334455667700ffeeddccbbaa9988").unwrap(),
            &[
                ArgValue::Bytes(
                    hex::decode("8899aabbccddeeff0011223344556677fedcba98765432100123456789abcdef")
                        .unwrap(),
                ),
                ArgValue::Bytes(Vec::new()),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("Hex".into()),
                ArgValue::Str("GOST R 34.12 (Kuznyechik, 2015)".into()),
                ArgValue::Str("E-Z".into()),
                ArgValue::Str("NO".into()),
            ],
        )
        .unwrap();
    assert_eq!(result, b"7f679d90bebc24305a468d42b9d4edcd7f679d90bebc2430");
}
